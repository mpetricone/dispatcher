use std::fs::File;
use rodio::{ Decoder, MixerSinkDevice, source::Source};

pub fn play_file(path: &str){
    if let Ok(handle) = rodio::default_sink_device().open_default_sink() &&
        let Ok(player) = rodio::player::connect_new(&handle) &&
        let Ok(file) = File::open(path) &&
        let (source) = Decoder::try_frome(file) {
            handle.mixer.add_source(source);`
        }
}
