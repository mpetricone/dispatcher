use rdev::{
    EventType::{KeyPress, KeyRelease},
    Key, ListenError, listen,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

/// Records keyboard input
///
/// Returns a vector of raw input information.
/// There will be multple KeyPress events, with timestamps,
/// but no durations.
///
/// you may want to call [normalize_sequence] to
/// get a more usefull event sequence.
///
/// events are capturd in a seperate thread until
/// [rdev::stop_listening] is called
pub fn record_sequence() -> Result<Arc<Mutex<Vec<rdev::Event>>>, ListenError> {
    let sequence = Arc::new(Mutex::new(vec![]));
    let rs = sequence.clone();
    thread::spawn(move || {
        listen(move |event| {
            rs.lock().unwrap().push(event);
        })
    });

    Ok(sequence.clone())
}

/// Used to retain data of [rdev::EventType] data with durations
///
/// returned by [normalize_sequence] for cleaner keypress data
/// as the X11 system returns quite a few extraneous events, without
/// duration context
///
/// These are serializable for long term storage
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct InputEvent {
    pub event_type: rdev::EventType,
    pub duration: Option<Duration>,
    time: SystemTime,
}

impl InputEvent {
    fn new(
        event_type: rdev::EventType,
        duration: Option<Duration>,
        time: SystemTime,
    ) -> InputEvent {
        InputEvent {
            event_type,
            duration,
            time,
        }
    }
}

const CONTROLKEYS: [Key; 6] = [
    Key::ShiftLeft,
    Key::ShiftRight,
    Key::ControlLeft,
    Key::ControlRight,
    Key::MetaLeft,
    Key::MetaRight,
];

/// Takes a Vec of [rdev::Event] and returns a more usable format.
///
/// Durations to [rdev::EventType::KeyPress] are calculated.
/// Mouse events, Non Key related events, duplicate [rdev::EventType::KeyPress]
/// are all removed.
///
/// Order of normalized events is retained.
pub fn normalize_sequence(
    raw_seq: Arc<Mutex<Vec<rdev::Event>>>,
) -> Result<Vec<InputEvent>, String> {
    if let Ok(guard) = &raw_seq.lock() {
        let mut event_chain: Vec<InputEvent> = vec![];
        let mut open_events: Vec<InputEvent> = vec![];
        let mut index = 0;
        'outer: while index < guard.len() {
            let cur = &guard[index];
            match cur.event_type {
                KeyPress(e) => {
                    //The event is already open, this is a repeat
                    if let Some(_pos) = open_events.iter().position(|x| x.event_type == KeyPress(e))
                    {
                        index += 1;
                        continue 'outer;
                    }
                    if let Some(np) = &guard[index..]
                        .iter()
                        .position(|x| x.event_type == KeyRelease(e))
                    {
                        open_events.push(InputEvent::new(
                            KeyPress(e),
                            None,
                            guard[index + np].time,
                        ));
                        let is_ctrl_key = CONTROLKEYS.iter().find(|x| **x == e);
                        if is_ctrl_key.is_none() {
                            event_chain.push(InputEvent::new(
                                KeyPress(e),
                                Some(
                                    guard[np + index]
                                        .time
                                        .duration_since(cur.time)
                                        .unwrap_or(Duration::from_secs(0)),
                                ),
                                guard[index].time,
                            ));
                        } else {
                            event_chain.push(InputEvent::new(KeyPress(e), None, guard[index].time))
                        }
                    } else {
                        event_chain.push(InputEvent::new(KeyPress(e), None, guard[index].time))
                    }
                }
                KeyRelease(e) => {
                    // I think there should only ever be 1
                    if let Some(e) = open_events.iter().position(|x| x.event_type == KeyPress(e)) {
                        open_events.swap_remove(e);
                    }
                    event_chain.push(InputEvent::new(KeyRelease(e), None, cur.time));
                }
                _ => (),
            }
            index += 1;
        }
        return Ok(event_chain);
    }
    Err("Could not unlock the Arc".to_string())
}

//fn input_callback(event: Event) {
//    match event.event_type {
//        EventType::KeyPress(k) => {
//            println!("user wrote {:?}", k);
//        }
//        _ => (),
//    }
//}
