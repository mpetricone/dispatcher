use rdev::{stop_listening, EventType::KeyPress, EventType, EventType::KeyRelease};
use std::{thread, time::Duration};
use voice_stream::VoiceStream;
use voice_stream::cpal::traits::StreamTrait;
use vosk::{Model, Recognizer};
use dispatcher::input_recorder;

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
    let keys = input_recorder::record_sequence();
    thread::sleep(Duration::from_secs(10));
    stop_listening();
    if let Ok(e) = keys {
        print!("\n\r");
        for k in e.lock().unwrap().iter() {
            match k.event_type {
                KeyPress(k) =>  print!("{:?}", k),
                KeyRelease(k) => print!("<KR>{:?}</KR>",k),
                _ => (),
            }
        }
        print!("\n\r");
    }
}
