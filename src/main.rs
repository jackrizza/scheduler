mod taskhandler;

use rand::Rng;
use std::sync::{Arc, Mutex};
use std::{thread, time};

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
        let cleaner = thread::spawn(move || {
            let mut f = f.lock().unwrap();
            if f.epoch % 10 == 0 {
                println!("Cleaner...");
                f.reprioritize();
            }

            f.inc_epoch();
        });

        cleaner.join().unwrap();

        let f = frame.clone();
        let executer = thread::spawn(move || {
            let mut f = f.lock().unwrap();
            if f.oplen() == 0 {
                println!("Sleeping executer... (EPOCH : {})", f.epoch);
                thread::sleep(time::Duration::from_millis(1000));
            } else {
                for event in &f.oplog {
                    println!("Priority {}, Event {:?} \n", event.priority, event);
                }
                // f.firstchild();
            }

            f.inc_epoch();
        });

        executer.join().unwrap();
        
        println!("End of loop... \n\n\n\n");
        
        // let _ = time::Duration::from_millis(500);
    }
}
