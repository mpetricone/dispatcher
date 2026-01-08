use rdev::{ EventType, Event, listen, ListenError};
use std::thread;
use std::sync::{ Arc, Mutex };

pub fn record_sequence() -> Result<Arc<Mutex<Vec<rdev::Event>>>, ListenError> {
    let mut sequence = Arc::new(Mutex::new(vec!()));
    let rs = sequence.clone();
    thread::spawn(move || {
        listen( move|event| {
           rs.lock().unwrap().push(event);
        })
    });

    return Ok(sequence.clone())
}

fn input_callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(k) => {
            println!("user wrote {:?}", k);
        }
        _ => (),
    }
}
