extern crate chrono;
extern crate timer;

use std::sync::mpsc::channel;

fn main() {
    let timer = timer::Timer::new();
    let (tx, rx) = channel();

    let _guard = timer.schedule_with_delay(chrono::Duration::seconds(3), move || {
        // This closure is executed on the scheduler thread,
        // so we want to move it away asap.

        let _ignored = tx.send(()); // Avoid unwrapping here.
    });

    rx.recv().unwrap();
    println!("This code has been executed after 3 seconds");
}
