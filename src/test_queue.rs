#![allow(non_snake_case)]

#[path = "../src/circular_queue.rs"]
mod circular_queue;

use std::time::Instant;

use circular_queue::CircularQueue;
use heapless::spsc::Queue;

const COUNT: usize = 1000;
const QSIZE: usize = 320_000;
const QSIZEADD: usize = QSIZE + 1;
fn main() {
    let mut queue: Queue<f64, QSIZEADD> = Queue::new();
    let mut buf = vec![0.0; QSIZE];
    {
        buf = vec![0.0; QSIZE];
        // println!("buf: {:?}", buf);
        let start = Instant::now();
        testQueue(&mut queue, &mut buf);
        println!("heapless::spsc::Queue elapsed: {:?}", start.elapsed());
    }
    {
        buf = vec![0.0; QSIZE];
        // println!("buf: {:?}", buf);
        let mut cQueue: CircularQueue<f64> = CircularQueue::with_capacity(QSIZE);
        let start = Instant::now();
        testCQeque(&mut cQueue, &mut buf);
        println!("CircularQueue elapsed: {:?}", start.elapsed());
    }
    {
        buf = vec![0.0; QSIZE];
        // println!("buf: {:?}", buf);
        let mut cQueue: CircularQueue<f64> = CircularQueue::with_capacity(QSIZE);
        let start = Instant::now();
        testCQeque1(&mut cQueue, &mut buf);
        println!("CircularQueue elapsed: {:?}", start.elapsed());
    }
    {
        buf = vec![0.0; QSIZE];
        // println!("buf: {:?}", buf);
        let mut cQueue: CircularQueue<f64> = CircularQueue::with_capacity(QSIZE);
        // for i in 1..QSIZE {
        //     cQueue.push(0.0 as f64);
        // }
        let start = Instant::now();
        testCQeque2(&mut cQueue, &mut buf);
        println!("CircularQueue elapsed: {:?}", start.elapsed());
    }
}


fn testQueue(queue: &mut Queue<f64, QSIZEADD>, buf: &mut Vec<f64>) {
    for i in 0..COUNT {
        match queue.enqueue(i as f64) {
            Ok(_) => {},
            Err(val) => {
                println!("error adding value: {:?}", val);
            },
        };
        
        for (i, item) in queue.iter().enumerate() {
            buf[i] = *item;
            // println!("readed buf: {:?}\t{:?}", i, item);
        }
        if queue.is_full() {
            queue.dequeue().unwrap();
            // println!("dequeue: {:?}", d);
        }
        // println!("readed buf: {:?}", &buf);
        // println!("queue: {:?}", &queue);
        // let oldest = queue.peek();
        // println!("readed: {:?}", oldest);    
    }
}

fn testCQeque(cQueue: &mut CircularQueue<f64>, buf: &mut [f64]) {
    for i in 0..COUNT {
        cQueue.push(i as f64);
        let mut i = 0;
        for item in cQueue.iter() {
            buf[i] = *item;
            i += 1;
            // println!("readed buf: {:?}\t{:?}", i, item);
        }
        // println!("readed: {:?}", &buf);
        // println!("queue: {:?}", &cQueue);
    }
}

fn testCQeque1(cQueue: &mut CircularQueue<f64>, buf: &mut Vec<f64>) {
    for i in 0..COUNT {
        cQueue.push(i as f64);
        buf.clear();
        buf.append(
            &mut cQueue.buffer().clone()
        );
        // cQueue.buffer().to_owned().clone();
        // println!("readed: {:?}", &buf);
        // println!("queue: {:?}", &cQueue);
    }
}

fn testCQeque2(cQueue: &mut CircularQueue<f64>, buf: &mut Vec<f64>) {
    // let mut b;
    for i in 0..COUNT {
        cQueue.push(i as f64);
        // cQueueBuffer = cQueue.buffer();
        // println!("readed: {:?}   {:?}", buf.len(), cQueueBuffer.len());
        cQueue.buffer().clone_into(buf);
        // buf  = b;
        // cQueue.buffer().to_owned().clone();
        // println!("readed: {:?}", &buf);
        // println!("queue: {:?}", &cQueue);
    }
}