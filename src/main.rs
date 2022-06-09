use rand::Rng;
use std::sync::{Arc, Mutex};
use std::{thread, time};

mod taskhandler;
mod threads;

fn main() {
    let frame = Arc::new(Mutex::new(taskhandler::scheduler::Scheduler::new()));
    let mut test_killer = 0;

    // test set
    for i in 1..40 {
        let mut f = frame.lock().unwrap();
        f.addop(taskhandler::event::Event::new(
            i,
            i.to_string(),
            i,
            rand::thread_rng().gen_range(1..5),
            false,
        ))
    }
    loop {
        // kill runaway in debug
        if cfg!(debug_assertions) {
            if test_killer > 199 {
                break;
            }
            test_killer += 1;
        }

        let f = frame.clone();
        let cleaner = threads::cleaner::cleaner(f);
        cleaner.join().unwrap();

        let f = frame.clone();
        let executer = threads::executer::executer(f);
        executer.join().unwrap();
    }
}
