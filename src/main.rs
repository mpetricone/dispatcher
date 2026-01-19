use dispatcher::{input_recorder, input_dispatcher::send_input_sequence};
use rdev::stop_listening;
use std::{thread, time::Duration};

use std::sync::{ Arc, Mutex};



fn main() {
    let keys = input_recorder::record_sequence().unwrap();
    thread::sleep(Duration::from_secs(10));
    stop_listening();
    println!("listening stopped");
    let normalized_keys = input_recorder::normalize_sequence(keys).unwrap();
    print!("\n\r");
    println!("{}", serde_json::to_string(&normalized_keys).unwrap());
    let arc_sequence = Arc::new(Mutex::new(normalized_keys));
    send_input_sequence(arc_sequence.clone(), Duration::from_millis(30)).unwrap();
    print!("\n\r");
}
