use enigo::*;
use std::{
    thread,
    sync::{Arc, Mutex},
    time::Duration
};
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
                    *mode = 1;
                } else if *key == Keycode::Down {
                    *mode = 2;
                } else if *key == Keycode::Left || *key == Keycode::Right {
                    *mode = 0;
                }
            }
            thread::sleep(Duration::from_micros(5));
        }
    });

    loop {
        thread::sleep(Duration::from_millis(80));
        let mode = mode.lock().unwrap(); 
        if *mode == 1 {
            let _ = enigo.scroll(-1, Axis::Vertical);
        } else if *mode == 2 {
            let _ = enigo.scroll(1, Axis::Vertical);
        }
    }
}

