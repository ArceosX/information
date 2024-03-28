use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn main() {
    let data = Arc::new((Mutex::new(false), Condvar::new(), Mutex::new(0)));

    let data1 = Arc::clone(&data);
    let data2 = Arc::clone(&data);

    let thread1 = thread::spawn(move || {
        let (lock, cvar, _) = &*data1;

        let mut started = lock.lock().unwrap();
        *started = true;

        while !*started {
            started = cvar.wait(started).unwrap();
        }

        drop(started);

        println!("Thread 1 running");
    });

    let thread2 = thread::spawn(move || {
        let (lock, cvar, _) = &*data2;

        let started = lock.lock().unwrap();

        let _ = cvar.notify_one();

        println!("Thread 2 running");
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}