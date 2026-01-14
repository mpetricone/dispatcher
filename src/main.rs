use dispatcher::{input_recorder, input_dispatcher::send_input_sequence};
use rdev::stop_listening;
use std::{thread, time::Duration};
use voice_stream::VoiceStream;
use voice_stream::cpal::traits::StreamTrait;
use vosk::{Model, Recognizer};
use std::sync::{ Arc, Mutex};

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
async fn _voicereq_trial() {
    let vmodel = Model::new("./vosk-model-small-en-us-0.15").unwrap();
    let mut vrec = Recognizer::new(&vmodel, 16000.0).unwrap();

    let (voice_stream, mut rx) = VoiceStream::default_device().unwrap();

    voice_stream.play().unwrap();

    loop {
        match rx.recv().await {
            Some(samples) => {
                if samples.len() > 0 {
                    println!("Samples size: {}", samples.len());
                    let _ = vrec.accept_waveform(&samples.to_i16());

                    println!("{:#?}", vrec.final_result());
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let keys = input_recorder::record_sequence().unwrap();
    thread::sleep(Duration::from_secs(10));
    stop_listening();
    println!("listening stopped");
    let normalized_keys = input_recorder::normalize_sequence(keys).unwrap();
    print!("\n\r");
    println!("{}", serde_json::to_string(&normalized_keys).unwrap());
    let arc_sequence = Arc::new(Mutex::new(normalized_keys));
    send_input_sequence(arc_sequence.clone(), Duration::from_millis(30)).unwrap();
    print!("\n\r");
}
