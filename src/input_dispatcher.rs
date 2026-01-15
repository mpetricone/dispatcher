use crate::input_recorder::InputEvent;
use std::sync::{Arc, Mutex };
use std::{time, thread};
use std::error::Error;

/// This takes a Vec of InputEvent and sends them to the OS/GUI in order.
///
///
/// It will ignore [InputEvent] of less than = 500millis, instead using the delay between events
/// I don't think it's necessary to place the duration behind a Arc-mutex
///
/// Currently, we are only processing keyboard events
pub fn send_input_sequence(sequence: Arc<Mutex<Vec<InputEvent>>>, delay: time::Duration) -> Result<(), Box<dyn Error>> {
    match sequence.lock() {
        Ok(guard) => {
            for e in guard.iter() {
                rdev::simulate(&e.event_type)?;
                let mut final_delay = delay;
                if let Some(e) = e.duration {
                    if e.as_millis() > 500u128 {final_delay = e }
                }
                thread::sleep(final_delay);
            }
        },
        Err(e) => {
            return Err(e.to_string().into())
        }
    }
    Ok(())
}
