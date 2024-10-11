#![allow(non_snake_case)]

use std::{thread, fmt::Debug, time::Duration, sync::{Arc, Mutex}, cell::RefCell, rc::Rc};

type OnEvent<T> = dyn Fn(T) -> () + Send + Sync;
struct Stream<'a, T> {
    onEvent: &'a OnEvent<T>,
}

impl<'a, T: Send + Sync + Debug> Stream<'a, T> {
    pub fn new() -> Self {
        Self { onEvent: &|_|{} }
    }
    pub fn add(&mut self, event: T) {
        println!("Stream.add | event: {:?}", &event);
        let onEvent = self.onEvent;
            (onEvent)(event);
    }
    pub fn listen(&mut self, onEvent: &'a OnEvent<T>) {
        self.onEvent = onEvent;
    }
}

fn main() {
    // let streamRef = &stream;
    let stream: Arc<Mutex<Stream<i64>>> = Arc::new(Mutex::new(Stream::new()));

    let streamMutax1 = stream.clone();
    let joinHandle1 = thread::spawn({
        move || {
            for event in vec![11, 13, 15, 17, 19,] {
                println!("main.thread1::spawn | trying lock");
                match streamMutax1.try_lock() {
                    Ok(mut stream1) => {
                        println!("main.thread1::spawn | trying lock - done");
                        stream1.add(event);
                        println!("main.thread1::spawn | event: {:?}", event);        
                    },
                    Err(err) => {
                        println!("main.thread1::spawn | error: {:?}", err);
                    },
                };
                // thread::sleep(Duration::from_secs_f32(0.1))
            }
        }
    });    
    // let stream2Mutax = stream.clone();
    // let joinHandle2 = thread::spawn({
    //     move || {
    //         for event in vec![21, 23, 25, 27, 29,] {
    //             println!("main.thread2::spawn | trying lock");
    //             let mut stream2 = streamMutax2.lock().unwrap();
    //             println!("main.thread2::spawn | trying lock - done");
    //             stream2.add(event);
    //             println!("main.thread2::spawn | event: {:?}", event);
    //             thread::sleep(Duration::from_secs_f32(0.1))
    //         }
    //     }
    // });    
    // println!("main | trying lock");
    let mut streamListen = stream.try_lock().unwrap();
    // println!("main | trying lock - done");
    streamListen.listen(&|event| {
        println!("main.stream.listen | event: {:?}", event)
    });
    println!("main | sleep...");
    // thread::sleep(Duration::from_secs_f32(10.1))

    joinHandle1.join().unwrap();
    // joinHandle2.join().unwrap();
}