use enigo::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let mode = Arc::new(Mutex::new(0));

    let device_state = DeviceState::new();
    let mode_clone = Arc::clone(&mode);

    thread::spawn(move || {
        loop {
            let keys: Vec<Keycode> = device_state.get_keys();
            let mut mode = mode_clone.lock().unwrap(); 
            for key in &keys {
                if *key == Keycode::Up {
                    if *mode != 0 {
                        *mode = 0;
                    } else {
                        *mode = 1;
                    }
                } else if *key == Keycode::Down {
                    if *mode != 0 {
                        *mode = 0;
                    } else {
                        *mode = 2;
                    }
                }
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    loop {
        thread::sleep(Duration::from_millis(170));
        let mode = mode.lock().unwrap(); 
        if *mode == 1 {
            let _ = enigo.scroll(-1, Axis::Vertical);
        } else if *mode == 2 {
            let _ = enigo.scroll(1, Axis::Vertical);
        }
    }
}

