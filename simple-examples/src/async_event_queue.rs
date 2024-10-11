#![allow(non_snake_case)]

use std::{thread, time::{Duration, Instant}, sync::{Arc, Mutex}, ops::Add, cmp::Ordering};

use event_listener::Event;


const QSIZE: u64 = 320_000;
// const QSIZEADD: usize = QSIZE + 1;
// static mut queue: Queue<i16, QSIZEADD> = Queue::new();
// let (mut prod, mut cons) = unsafe { queue.split() };

fn main() {
    let (send, recv) = multiqueue::mpmc_queue(QSIZE);
    let flag = Arc::new(Mutex::new(0));
    let event = Arc::new(Event::new());
    // Spawn a thread that will set the flag after 1 second.
    let count1 = 100000;
    let count2 = 100000;
    thread::spawn({
        let flag = flag.clone();
        let event = event.clone();
        let send = send.clone();
        move || {
            for value in 0..count1 {
                // thread::sleep(Duration::from_secs_f32(0.01));
                match send.try_send(value) {
                    Ok(_) => {},
                    Err(err) => {
                        println!("main.tread2 | send error: {:?}", err);
                    },
                };        
                // Notify all listeners that the flag has been set.
                event.notify(usize::MAX);
            }
            flag.lock().unwrap().add(2);
        }
    });
    // Spawn a thread that will set the flag after 1 second.
    thread::spawn({
        let flag = flag.clone();
        let event = event.clone();
        move || {
            for value in 0..count2 {
                // thread::sleep(Duration::from_secs_f32(0.01));
                match send.try_send(value) {
                    Ok(_) => {},
                    Err(err) => {
                        println!("main.tread2 | send error: {:?}", err);
                    },
                };
                // Notify all listeners that the flag has been set.
                event.notify(usize::MAX);
            }
            flag.lock().unwrap().add(1);
        }
    });


    // Wait until the flag is set.
    // loop {
    //     println!("main.loop | wait for event...");
    //     // Check the flag.
    //     if flag.lock().unwrap().cmp(&1) == Ordering::Greater {
    //         break;
    //     }

    //     // Start listening for events.
    //     let mut listener = event.listen();

    //     // Check the flag again after creating the listener.
    //     if flag.lock().unwrap().cmp(&1) == Ordering::Greater {
    //         break;
    //     }

    //     // Wait for a notification and continue the loop.
    //     listener.as_mut().wait();
    let mut count = 0;
    let start = Instant::now();
    for event in recv {
        // println!("main.listen | event: {:?}", event);
        count += 1;
    };
    assert_eq!(count, count1 + count2);
    println!("main | elapsed: {:?}", start.elapsed());

    // }
}