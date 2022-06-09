use super::*;

pub fn cleaner(
    f: Arc<Mutex<crate::taskhandler::scheduler::Scheduler<String>>>,
) -> std::thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut f = f.lock().unwrap();
        if f.epoch % 10 == 0 {
            println!("Cleaner...");
            f.reprioritize();
        }

        f.inc_epoch();
    })
}
