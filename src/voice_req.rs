use cpal::{traits::DeviceTrait, traits::HostTrait};
use std::error::Error;
use tokio::sync::mpsc;
use vosk::{Model, Recognizer};

/// # Vosk requires i16 audio data, but we can only capture in f32
trait AudioThunk {
    fn to_i16(&self) -> Vec<i16>;
}

// /// Google found this, but I don't understand it as well as AudioThunk
//trait AudioThunk2 {
//    fn to_i16_a(&self) -> Vec<i16>;
//}

impl AudioThunk for Vec<f32> {
    /// Note this has not been benchmarked
    fn to_i16(&self) -> Vec<i16> {
        let mut newv = Vec::with_capacity(self.len());

        for val in self {
            newv.push((val.clamp(-1.0, 1.0) * 3276.0) as i16);
        }
        newv
    }
}

//impl AudioThunk2 for Vec<f32> {
//    fn to_i16_a(&self) -> Vec<i16> {
//        self.into_iter()
//            .map(|v| (v.clamp(-1.0, 1.0) * 3276.0) as i16)
//            .collect::<Vec<i16>>()
//    }
//}

/// # Voice recognition main loop.
/// I am happy with it at this point, except for the need to thunk to Vosk.
/// I suspect the thunking may be causing delays, but I have not found a
/// microphone input library that records data as i16
async fn voice_req_loop(mut vr_context: VoiceReqContext) -> Result<(), Box<dyn Error>> {
    let vmodel = Model::new(vr_context.vosk_path.clone()).ok_or("Failed to load Vosk model")?;
    let mut vrec = Recognizer::new_with_grammar(&vmodel, 16000.0, &vr_context.grammar[..])
        .ok_or("Failed to create Vosk recognizer")?;

    let cpal_host = cpal::default_host();
    let cpal_device = cpal_host
        .default_input_device()
        .ok_or("No default input device")?;
    let cpal_config = cpal::StreamConfig {
        channels: 1,
        sample_rate: 16000,
        buffer_size: cpal::BufferSize::Default,
    };
    let (input_tx, input_rx) = mpsc::channel::<Vec<f32>>(128);
    let input_stream = cpal_device.build_input_stream(
        cpal_config,
        move |data: &[f32], _| {
            input_tx.try_send(data.to_vec());
        },
        |err| {
            eprintln!("Input stream error: {}", err);
        },
        None,
    )?;

    if let Ok((voice_stream, mut rx)) = VoiceStream::default_device() {
        voice_stream.play()?;
        while let Some(r) = rx.recv().await {
            if !r.is_empty() {
                let _ = vrec.accept_waveform(&r.to_i16());
                // Clippy, my fried, this is to allow future growth
                #[allow(clippy::single_match)]
                match vr_context.rx_commands.try_recv() {
                    Ok(c) => {
                        if c == VoiceReqCommands::Stop {
                            let _ = vr_context.tx_results.send(VoiceReqResults::Halting).await;
                            rx.close();
                            continue;
                        }
                    }
                    _ => {}
                }
                if let Some(heard) = vrec.final_result().single() {
                    vr_context
                        .tx_results
                        .send(VoiceReqResults::Recognized(heard.text.to_string()))
                        .await?;
                }
            }
        }
    } else {
        return Err("Failed to open audio device".into());
    }
    drop(vrec);
    drop(vmodel);
    Ok(())
}

/// # Commands sent to the voce recognition thread.
///
/// Currently not fully implemented.
#[derive(PartialEq)]
pub enum VoiceReqCommands {
    Stop,
    Pause,
    Start,
}

/// # Results sent by [start_voice_req]
///
/// Recognized will be sent for any succesfully transcribed voice events
/// Halting will be sent when the thread determines it is shutting down.
#[derive(PartialEq)]
pub enum VoiceReqResults {
    Recognized(String),
    Halting,
}

/// # Settings and channels used by the voice recognizer
pub struct VoiceReqContext {
    tx_results: mpsc::Sender<VoiceReqResults>,
    rx_commands: mpsc::Receiver<VoiceReqCommands>,
    grammar: Vec<String>,
    vosk_path: String,
}

impl VoiceReqContext {
    pub fn new(
        rx_commands: mpsc::Receiver<VoiceReqCommands>,
        tx_results: mpsc::Sender<VoiceReqResults>,
        grammar: Vec<String>,
        vosk_path: String,
    ) -> Self {
        Self {
            tx_results,
            rx_commands,
            grammar,
            vosk_path,
        }
    }
}

/// # Start a thread for voice recognition.
/// excepts a couple channels to be setup first
///
/// This function reports results on any voice recognition
/// for command processing, look at [crate::primary_dispatcher]
pub fn start_voice_req(context: VoiceReqContext) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        if let Err(e) = voice_req_loop(context).await {
            eprintln!("Voice recognition error: {}", e);
        }
    })
}
