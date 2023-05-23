#![allow(dead_code)]
use std::{
    println,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn threads() {
    println!("Using Threads to Run Code Simultaneously");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    println!("\nUsing move Closures with Threads");
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Hear's a vector {:?}", v);
    });

    handle.join().unwrap();
}

fn channels(sleep_millis: u64) {
    println!("Channels");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {val}"); Cannot use val as it is moved in the line before.
    });

    let recieved = rx.recv().unwrap();
    println!("Got: {}", recieved);

    println!("\nMultiple Values and Reciever Waiting");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(sleep_millis));
        }
    });

    for recieved in rx {
        println!("Got: {recieved}")
    }

    println!("\nMultiple Producers by Cloning Transmitter");
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(sleep_millis));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(sleep_millis));
        }
    });

    for recieved in rx {
        println!("Got: {recieved}")
    }
}

fn shared_state() {
    println!("Shared-Stete Concurrency");
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    println!("\nSharing a Mutex<T> Between Multiple Threads");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn main() {
    // threads();
    // println!();
    // channels(500);
    shared_state();
}
