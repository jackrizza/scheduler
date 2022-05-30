use super::*;

#[derive(Debug, Clone)]
pub struct Scheduler {
    pub oplog: Vec<event::Event>,
    pub epoch: i32,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            oplog: vec![event::Event::new(0, "Hello_World".to_string(), 0, 0, true)],
            epoch: 0,
        }
    }

    pub fn lookup(&self, id: i32) -> usize {
        self.oplog
            .iter()
            .position(|x| x.id == id.clone())
            .unwrap()
            .clone()
    }

    pub fn inc_epoch(&mut self) {
        self.epoch += 1;
    }

    pub fn reprioritize(&mut self) {
        let oplog = self.oplog.clone();
        for mut event in oplog {
            match ((self.epoch - event.epoch) as f32 / 100.00).floor() as i32 {
                0 => event.priority = 0,
                1..=3 => {
                    if event.priority > 0 {
                        event.priority -= 1;
                    } else {
                        event.priority = 0;
                    }
                }
                _ => {
                    event.priority = 0;
                }
            };
        };
    }

    pub fn firstchild(&mut self) {
        if self.oplog[0].executed {
            self.oplog.remove(0);
        } else {
            self.oplog[0].execute();
        }
    }

    pub fn updateop(&mut self, id: i32, task_name: String, executed: bool) {
        let lookup = self.lookup(id).clone();
        self.oplog[lookup].task_name = task_name;
        self.oplog[lookup].executed = executed;
    }

    pub fn oplen(&self) -> i32 {
        self.oplog.len() as i32
    }

    pub fn addop(&mut self, event: event::Event) {
        let _ = self.oplog.push(event);
    }
}

#[cfg(test)]
mod traits {
    use super::*;

    #[test]
    fn create_scheduler() {
        let scheduler = Scheduler::new();
        assert_eq!(scheduler.oplen(), 1);
    }

    #[test]
    fn add_to_oplog() {
        let mut scheduler = Scheduler::new();
        scheduler.addop(event::Event::new(
            scheduler.oplen().clone(),
            "test".to_string(),
            scheduler.epoch,
            0,
            false,
        ));

        assert_eq!(scheduler.oplen(), 2);
    }
}
