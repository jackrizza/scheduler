mod taskhandler;

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let frame = Arc::new(Mutex::new(taskhandler::scheduler::Scheduler::new()));

    // test set
    for i in 1..3 {
        let mut f = frame.lock().unwrap();
        f.addop(taskhandler::event::Event::new(
            i,
            i.to_string(),
            i,
            1,
            false,
        ))
    }

    let executer = thread::spawn(move || {
        loop {
            let mut f = frame.lock().unwrap();
            if f.oplen() == 0 {
                break;
            } // clean exit

            println!("test {} : {:?} \n", f.epoch, f.oplog);
            f.firstchild();

            f.inc_epoch();
        }
    });

    executer.join().unwrap();
}
