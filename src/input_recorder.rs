use rdev::{ EventType, Event, listen, ListenError, EventType::{ KeyPress, KeyRelease}};
use std::thread;
use std::sync::{ Arc, Mutex};
use std::time::{ Duration, SystemTime };
use std::collections::LinkedList;

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

#[derive(Debug)]
pub struct InputEvent {
    event_type: rdev::EventType,
    duration: Option<Duration>,
    time: SystemTime,
}

impl InputEvent {
    fn new(event_type: rdev::EventType, duration: Option<Duration>, time: SystemTime) -> InputEvent{
        InputEvent {
            event_type,
            duration,
            time,
        }
    }
}

pub fn normalize_sequence(raw_seq: Arc<Mutex<Vec<rdev::Event>>>) -> Result<Vec<InputEvent>,String> {
    if let Ok(guard) = &raw_seq.lock() {
        let mut event_chain: Vec<InputEvent> = vec!();
        let mut open_events: Vec<InputEvent> = vec!();
        let mut index = 0;

        while index < guard.len() {
            let cur = &guard[index];
            let duration = 0u32;
            match cur.event_type {
                KeyPress(e) => {
                    let mut existing_idx = 0usize;
                    if open_events.is_empty() {
                        open_events.push(InputEvent::new(guard[index].event_type, None, guard[index].time));
                    }
                    existing_idx = open_events.iter().position(|x| x.event_type == KeyPress(e)).unwrap_or(0);
                    if open_events[existing_idx].time <= cur.time {
                        if let Some(np) = &guard[index..].iter().position(|x| x.event_type == KeyRelease(e)) {
                            open_events.push(InputEvent::new(KeyPress(e), None, guard[index+np].time));
                            event_chain.push(InputEvent::new(KeyPress(e),Some(guard[np+index].time.duration_since(cur.time).unwrap()), guard[index].time ));
                        } else {
                            event_chain.push(InputEvent::new(KeyPress(e), None, guard[index].time))
                        }
                    }
                },
                KeyRelease(e) => {
                    // I think there should only ever be 1
                    open_events.remove(
                        open_events.iter().position(|x| x.event_type == KeyPress(e)).unwrap());
                    event_chain.push(InputEvent::new(KeyRelease(e),None, cur.time));
                },
                _ => (),
            }
            index+=1;
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
