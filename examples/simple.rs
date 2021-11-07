extern crate graceful;

use std::sync::atomic::{AtomicBool, Ordering};

use std::thread;
use std::time::Duration;
use graceful::SignalGuard;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref STOP: AtomicBool = AtomicBool::new(false);
}

fn main() {
    let signal_guard = SignalGuard::new();

    let handle = thread::spawn(|| {
        println!("Worker thread started. Type Ctrl+C to stop.");
        while !STOP.load(Ordering::Acquire) {
            println!("working...");
            thread::sleep(Duration::from_millis(500));
        }
        println!("Bye.");
    });

    signal_guard.at_exit(move |sig| {
        println!("Signal {} received.", sig);
        STOP.store(true, Ordering::Release);
        handle.join().unwrap();
    });
}
