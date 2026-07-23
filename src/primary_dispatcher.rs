use crate::action_record::ActionRecord;
use crate::config::DispatcherConfig;
use crate::input_dispatcher;
use crate::voice_req;
use crate::voice_req::VoiceReqCommands;
use crate::voice_req::VoiceReqContext;
use crate::voice_req::VoiceReqResults;
use crate::audio_playback;
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::sync::mpsc;

/// Sends keyboard simulations to the GUI/X11
fn process_voice_input(
    recognized_string: &str,
    action_list: &[ActionRecord],
    config: DispatcherConfig,
) {
    if let Some(a) = action_list
        .iter()
        .find(|x| x.activator_text == recognized_string)
    {
        let guard = Arc::new(Mutex::new(a.action_stream.clone()));
        if let Err(e) = input_dispatcher::send_input_sequence(
            guard,
            Duration::from_millis(config.default_command_delay.into()),
        ) {
            eprintln!(
                "Got and Eror during voice processing for command {}: {}",
                a.name, e
            );
        } else {
            audio_playback::play_file(&a.completion_audio_path.audio_file.clone().unwrap_or_default().to_string_lossy());
        }
    }
}

/// # Listen for voice input and send reconized commands to [input_dispatcher]
pub async fn listener_loop(
    rx_commands: mpsc::Receiver<VoiceReqCommands>,
    action_list: Vec<ActionRecord>,
    vosk_path: String,
    config: DispatcherConfig,
) {
    let (tx_results, mut rx_results) = mpsc::channel(50);

    let mut activators: Vec<_> = action_list
        .iter()
        .map(|x| x.activator_text.clone().to_lowercase())
        .collect();
    activators.push("[unk]".to_string());
    let _handle = voice_req::start_voice_req(VoiceReqContext::new(
        rx_commands,
        tx_results,
        activators,
        vosk_path,
    ));

    while let Some(r) = rx_results.recv().await {
        match r {
            VoiceReqResults::Recognized(e) => process_voice_input(&e, &action_list, config.clone()),
            VoiceReqResults::Halting => rx_results.close(),
        }
    }
}

pub fn begin_dispatch(
    action_list: Vec<ActionRecord>,
    rx_commands: mpsc::Receiver<VoiceReqCommands>,
    vosk_path: String,
    config: DispatcherConfig,
) -> io::Result<()> {
    let rt = Builder::new_current_thread().enable_all().build()?;
    thread::spawn(move || {
        rt.block_on(listener_loop(rx_commands, action_list, vosk_path, config));
    });
    Ok(())
}
