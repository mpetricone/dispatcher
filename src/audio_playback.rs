use std::fs::File;
use std::io::BufReader;

/// Plays an audio file at the given path using the default audio device.
/// Does nothing on error.
pub fn play_file(path: &str) {
    if let Ok(sink_handle) = rodio::DeviceSinkBuilder::open_default_sink() &&
        let Ok(open_path) = File::open(path) {
            let file = BufReader::new(open_path);
            let _player = rodio::play(sink_handle.mixer(), file).ok();
        }
        //Do we need a sleep? I hope not.
}
