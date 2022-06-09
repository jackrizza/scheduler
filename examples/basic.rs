extern crate scheduler;
use scheduler::{taskhandler,threads};

fn main() {
    let mut frame : taskhandler::scheduler::Scheduler<i32> = taskhandler::scheduler::Scheduler::new();
    let epoch = frame.inc_epoch();
    frame.addop(taskhandler::event::Event::new(1, 1, epoch, 0, false));
    println!("{:?}", frame);
}
