#![allow(non_snake_case)]

use std::{thread, fmt::Debug, time::Duration, sync::{Arc, Mutex, atomic::{AtomicBool, Ordering, AtomicI16}}, cell::RefCell, rc::Rc};

use event_listener::Event;


fn main() {
    let flag = Arc::new(AtomicI16::new(0));
    let event = Arc::new(Event::new());
    
    // Spawn a thread that will set the flag after 1 second.
    thread::spawn({
        let flag = flag.clone();
        let event = event.clone();
        move || {
            for value in vec![1, 3, 5, 7, 9, -1] {
                // Wait for a second.
                thread::sleep(Duration::from_secs(1));
                // Set the flag.
                flag.store(value, Ordering::SeqCst);
        
                // Notify all listeners that the flag has been set.
                event.notify(usize::MAX);
            }
        }
    });


    // Wait until the flag is set.
    loop {
        // Check the flag.
        if flag.load(Ordering::SeqCst) < 0 {
            break;
        }

        // Start listening for events.
        let mut listener = event.listen();

        // Check the flag again after creating the listener.
        if flag.load(Ordering::SeqCst) < 0 {
            break;
        }

        // Wait for a notification and continue the loop.
        listener.as_mut().wait();
        println!("flag value: {:?}", flag.load(Ordering::SeqCst))
    }
}