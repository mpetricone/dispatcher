use crate::action_record::ActionRecord;
use crate::voice_req;
use crate::voice_req::VoiceReqResults;
use crate::input_dispatcher;
use tokio::sync::mpsc;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn process_voice_input(recognized_string: &str, action_list: &Vec<ActionRecord>) {
    if let Some(a) = action_list.iter().find(|x| x.activator_text == recognized_string) {
        let guard = Arc::new(Mutex::new(a.action_stream.clone()));
        if let Err(e) = input_dispatcher::send_input_sequence(guard, Duration::from_millis(20)) {
            eprintln!("Got and Eror during voice processing for command {}: {}",a.name, e);
        }
    }
}

/// # Listen for voice input and send reconized commands to {input_dispatcher}
pub async fn listener_loop(action_list: Vec<ActionRecord>) {
    let (_tx_commands, rx_commands) = mpsc::channel(10);
    let (tx_results, mut rx_results) = mpsc::channel(50);

    match voice_req::start_voice_req(rx_commands, tx_results).await {
        Ok(_) => {
            while let Some(r) =  rx_results.recv().await {
                match r {
                    VoiceReqResults::Recognized(e) => process_voice_input(&e, &action_list),
                    VoiceReqResults::Halting =>  rx_results.close(),
                }
            }
        },
        Err(e) => eprintln!("Got Error in main listener loop: {}", e),
    }
}
