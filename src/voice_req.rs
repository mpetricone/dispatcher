use std::sync::{mpsc};
use std::thread::JoinHandle;
use std::thread;
use std::error::Error;
use voice_stream::VoiceStream;
use voice_stream::cpal::traits::StreamTrait;
use vosk::{Model, Recognizer};

trait AudioThunk {
    fn to_i16(&self) -> Vec<i16>;
}

trait AudioThunk2 {
    fn to_i16_a(&self) -> Vec<i16>;
}

impl AudioThunk for Vec<f32> {
    fn to_i16(&self) -> Vec<i16> {
        let mut newv = Vec::with_capacity(self.len());

        for val in self {
            newv.push((val.clamp(-1.0, 1.0) * 3276.0) as i16);
        }
        newv
    }
}

impl AudioThunk2 for Vec<f32> {
    fn to_i16_a(&self) -> Vec<i16> {
        self.into_iter()
            .map(|v| (v.clamp(-1.0, 1.0) * 3276.0) as i16)
            .collect::<Vec<i16>>()
    }
}

/// Test of voice recognition
/// I am happy with it at this point, except for the need to thunk to Vosk
async fn voicereq_trial(vr_context: &VoiceReqContext) -> Result<(), Box<dyn Error>> {
    let vmodel = Model::new("./vosk-model-small-en-us-0.15").unwrap();
    let mut vrec = Recognizer::new(&vmodel, 16000.0).unwrap();

    let (voice_stream, mut rx) = VoiceStream::default_device().unwrap();

    voice_stream.play().unwrap();

    while let Some(r) = rx.recv().await  {
        if r.len() > 0 {
            let _ = vrec.accept_waveform(&r.to_i16());
            match vr_context.rx_commands.try_recv() {
                Ok(c) => {
                    if c == VoiceReqCommands::Stop {
                        let _  = vr_context.tx_results.send(VoiceReqResults::Halting);
                        rx.close();
                        continue;
                    }
                },
                _ => {}
            }
            if let Some(heard) = vrec.final_result().single() {
                if vr_context.listen_for.contains(&heard.text.to_string()) {
                    vr_context.tx_results.send(VoiceReqResults::Recognized(heard.text.to_string()))?;
                }
            }
        }
    }
    Ok(())
}

#[derive(PartialEq)]
pub enum VoiceReqCommands {
    Stop,
    Pause,
    Start,
}

#[derive(PartialEq)]
pub enum VoiceReqResults {
    Recognized(String),
    Halting,
}

struct VoiceReqContext {
    tx_results: mpsc::Sender<VoiceReqResults>,
    rx_commands: mpsc::Receiver<VoiceReqCommands>,
    listen_for: Vec<String>,
}

pub fn start_voice_req(
    listen_for: Vec<String>,
    rx_commands: mpsc::Receiver<VoiceReqCommands>,
    tx_results: mpsc::Sender<VoiceReqResults>) -> Result<JoinHandle<()>, Box<dyn Error>> {
        let vr = VoiceReqContext{
            rx_commands,
            tx_results,
            listen_for,
        };
        Ok(thread::spawn( move || {
            voicereq_trial(&vr);
        }))
    }
