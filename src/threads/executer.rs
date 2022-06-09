use super::*;

pub fn executer(
    f: Arc<Mutex<crate::taskhandler::scheduler::Scheduler<String>>>,
) -> std::thread::JoinHandle<()> {
    thread::spawn(move || {
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
    })
}
