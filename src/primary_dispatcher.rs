use crate::action_record::ActionRecord;
use crate::input_dispatcher;
use crate::voice_req::{self, VoiceReqCommands, VoiceReqResults};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct VoiceListenerHandle {
    pub tx_commands: mpsc::Sender<VoiceReqCommands>,
}

pub struct VoiceListenerThreadHandle {
    pub handle: VoiceListenerHandle,
    shutdown_complete: Arc<Mutex<Option<std::thread::JoinHandle<()>>>>,
}

impl VoiceListenerThreadHandle {
    pub fn wait_for_shutdown(self) {
        if let Some(handle) = self.shutdown_complete.lock().unwrap().take() {
            let _ = handle.join();
        }
    }
}

pub fn start_listener(
    action_list: Vec<ActionRecord>,
) -> Result<VoiceListenerThreadHandle, Box<dyn std::error::Error + Send + Sync>> {
    let (tx_results, mut rx_results) = mpsc::channel(50);

    let handle = voice_req::start_voice_req(tx_results)?;
    let tx_commands = handle.tx_commands.clone();

    let shutdown_complete: Arc<Mutex<Option<std::thread::JoinHandle<()>>>> = Arc::new(Mutex::new(None));

    let thread_handle = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            while let Some(r) = rx_results.recv().await {
                match r {
                    VoiceReqResults::Recognized(e) => {
                        if let Some(a) = action_list
                            .iter()
                            .find(|x| x.activator_text == e)
                        {
                            let guard = Arc::new(Mutex::new(a.action_stream.clone()));
                            if let Err(ee) = input_dispatcher::send_input_sequence(guard, Duration::from_millis(20)) {
                                eprintln!(
                                    "Got an Error during voice processing for command {}: {}",
                                    a.name, ee
                                );
                            }
                        }
                    }
                    VoiceReqResults::Halting => break,
                }
            }
        });
    });

    *shutdown_complete.lock().unwrap() = Some(thread_handle);

    Ok(VoiceListenerThreadHandle { 
        handle: VoiceListenerHandle { tx_commands },
        shutdown_complete,
    })
}
