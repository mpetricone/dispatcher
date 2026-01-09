use rdev::{ EventType, Event, listen, ListenError, EventType::{ KeyPress, KeyRelease}};
use std::thread;
use std::sync::{ Arc, Mutex};
use std::time::Duration;

pub fn record_sequence() -> Result<Arc<Mutex<Vec<rdev::Event>>>, ListenError> {
    let sequence = Arc::new(Mutex::new(vec!()));
    let rs = sequence.clone();
    thread::spawn(move || {
        listen( move|event| {
            rs.lock().unwrap().push(event);
        })
    });

    return Ok(sequence.clone())
}

struct InputEvent {
    event_type: rdev::EventType,
    duration: Option<Duration>,
    end_index: Option<usize>,
}

impl InputEvent {
    fn new(event_type: rdev::EventType, duration: Option<Duration>, end_index: Option<usize>) -> InputEvent{
        InputEvent {
            event_type,
            duration,
            end_index,
        }
    }
}

pub fn normalize_sequence(raw_seq: Arc<Mutex<Vec<rdev::Event>>>) -> Result<Vec<InputEvent>,String> {
    if let Ok(guard) = &raw_seq.lock() {
        let mut event_chain: Vec<InputEvent> = vec!();
        let mut skip_ind = vec!();
        let index = 0;
        while index <= guard.len() {
            let cur = &guard[index];
            let ts = cur.time;
            let duration = 0u32;
            match cur.event_type {
                KeyPress(e) => {
                    if let Some(next_ind) = guard[index..].iter().position(|item| item.event_type == KeyRelease(e)) {
                        if let Ok(dur) = guard[next_ind].time.duration_since(guard[index].time) {
                            event_chain.push(InputEvent::new(KeyPress(e), Some(dur), Some(next_ind)));
                            skip_ind.push(next_ind);
                        } else {return Err(format!("Issue with duration of keypress for events {:?} - {:?}", guard[next_ind], guard[index]))}
                    }
                }
                KeyRelease(e) => {
                    event_chain.push(InputEvent::new(KeyRelease(e),None, None));
                }
                _ => (),
            }
        }
        return Ok(event_chain)
    }
    return Err("Could not unlock the Arc".to_string())
}

fn input_callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(k) => {
            println!("user wrote {:?}", k);
        }
        _ => (),
    }
}
