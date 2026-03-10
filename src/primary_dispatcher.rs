use crate::action_record::ActionRecord;
use crate::voice_req::VoiceReqCommands;
use crate::input_dispatcher;
use crate::voice_req;
use crate::voice_req::VoiceReqResults;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;
use std::thread;
use tokio::runtime::Builder;
use std::io;

/// Sends keyboard simulations to the GUI/X11
fn process_voice_input(recognized_string: &str, action_list: &[ActionRecord]) {
    if let Some(a) = action_list
        .iter()
        .find(|x| x.activator_text == recognized_string)
    {
        let guard = Arc::new(Mutex::new(a.action_stream.clone()));
        if let Err(e) = input_dispatcher::send_input_sequence(guard, Duration::from_millis(20)) {
            eprintln!(
                "Got and Eror during voice processing for command {}: {}",
                a.name, e
            );
        }
    }
}

/// # Listen for voice input and send reconized commands to [input_dispatcher]
pub async fn listener_loop(rx_commands: mpsc::Receiver<VoiceReqCommands>, action_list: Vec<ActionRecord>) {
    let (tx_results, mut rx_results) = mpsc::channel(50);

    let _handle = voice_req::start_voice_req(rx_commands, tx_results);

    while let Some(r) = rx_results.recv().await {
        match r {
            VoiceReqResults::Recognized(e) => process_voice_input(&e, &action_list),
            VoiceReqResults::Halting => rx_results.close(),
        }
    }
}

pub fn begin_dispatch(action_list: Vec<ActionRecord>, rx_commands: mpsc::Receiver<VoiceReqCommands>) -> io::Result<()> {
    let rt = Builder::new_current_thread()
        .enable_all()
        .build()?;
    thread::spawn( move || {
        rt.block_on(listener_loop( rx_commands, action_list));
    });
    Ok(())
}
