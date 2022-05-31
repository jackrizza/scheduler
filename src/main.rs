mod taskhandler;

use std::sync::{mpsc, Arc, Mutex};
use std::{thread, time};
use rand::Rng; // 0.8.0

fn main() {
    let frame = Arc::new(Mutex::new(taskhandler::scheduler::Scheduler::new()));

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
        let f = frame.clone();
        {
            let mut f = f.lock().unwrap();
            if f.epoch % 10 == 0 {
                println!("Cleaner...");
                f.reprioritize();
                // f.restructure();
            }
        };

        let f = frame.clone();
        let executer = thread::spawn(move || {
            let mut f = f.lock().unwrap();
            if f.oplen() == 0 {
                println!("Sleeping executer... (EPOCH : {})", f.epoch);
                thread::sleep(time::Duration::from_millis(1000));
            } else {
                println!("test {} : {:?} \n", f.epoch, f.oplog);
                f.firstchild();
            }

            f.inc_epoch();
        });

        executer.join().unwrap();
    }
}
