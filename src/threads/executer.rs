use super::*;

pub fn executer<T : Send + 'static>(
    f: Arc<Mutex<crate::taskhandler::scheduler::Scheduler<T>>>,
) -> std::thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut f = f.lock().unwrap();
        if f.oplen() == 0 {
            println!("Sleeping executer... (EPOCH : {})", f.epoch);
            thread::sleep(time::Duration::from_millis(1000));
        } else {

            f.oplog[0 as usize].execute();

            // for event in &f.oplog {
                // println!("Priority {}, Event {:?} \n", event.priority, event);
            // }
            // f.remove_firstchild();
        }

        f.inc_epoch();
    })
}
