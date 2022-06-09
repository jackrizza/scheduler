extern crate scheduler;
use scheduler::{taskhandler,threads};
use std::sync::{Arc, Mutex};
use std::{thread, time};


#[derive(Debug)]
struct Test {
    foo : i32,
    bar : i32,
}

fn main() {
    let frame : Arc<Mutex<taskhandler::scheduler::Scheduler<Test>>> = Arc::new(Mutex::new(taskhandler::scheduler::Scheduler::new()));
    let mut test_killer = 0;
    {
        let mut f = frame.lock().unwrap();
        let epoch = f.epoch.clone();
        f.addop(taskhandler::event::Event::new(1, Test {foo : 1, bar : 1,}, epoch, 0, false));
        println!("{:?}", f);
    }
    loop {
        // kill runaway in debug
        if cfg!(debug_assertions) {
            if test_killer > 199 {
                break;
            }
            test_killer += 1;
        }

        let cleaner = threads::cleaner::cleaner(frame.clone());
        cleaner.join().unwrap();

        let executer = threads::executer::executer(frame.clone());
        executer.join().unwrap();
    }
}
