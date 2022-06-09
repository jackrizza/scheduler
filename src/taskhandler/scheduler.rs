use super::*;

#[derive(Debug, Clone)]
pub struct Scheduler<T> {
pub oplog: Vec<event::Event<T>>,
pub epoch: i32,
}

impl<T> Scheduler<T>
where T : Clone {
pub fn new() -> Self {
    Scheduler {
        oplog: Vec::new(),
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

pub fn refit(&mut self, re : [usize; 2]) {
    // let mut op = self.clone();
    println!("{:?}", re);
    let obj = self.oplog[re[1]].clone();
    self.oplog.remove(re[1]);
    let mut vec_a = Vec::new();
    vec_a = self.oplog[0..re[0]].to_vec();
    vec_a.push(obj);
    for i in re[0] + 1..self.oplen() as usize {
        vec_a.push(self.oplog[i].clone());
    }
    self.oplog = vec_a;   
}

pub fn reprioritize(&mut self) {
    let oplog = self.oplog.clone();
    for mut event in oplog {
        match ((self.epoch - event.epoch) as f32 / 10.00).floor() as i32 {
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
    }
}

pub fn restructure(&mut self) {
    let opdec = self.oplen();

    for i in 0..opdec - 1 {
        let index_1 = self.lookup(i);
        let mut index_2: usize = usize::MAX;
        for j in i..opdec - 2 {
            if self.oplog[self.lookup(j)].priority < self.oplog[index_1].priority {
                index_2 = self.lookup(i + 1);
                self.refit([index_1, index_2]);
                break;
            }
        }
    }
}

pub fn firstchild(&mut self) {
    if self.oplog[0].executed {
        self.oplog.remove(0);
    } else {
        self.oplog[0].execute();
    }
}

pub fn updateop(&mut self, id: i32, task_name: T, executed: bool) {
    let lookup = self.lookup(id).clone();
    self.oplog[lookup].task_name = task_name;
    self.oplog[lookup].executed = executed;
}

pub fn oplen(&self) -> i32 {
    self.oplog.len() as i32
}

pub fn addop(&mut self, event: event::Event<T>) {
    let _ = self.oplog.push(event);
}
}

#[cfg(test)]
mod traits {
    use super::*;

    #[test]
    fn create_scheduler() {
        let scheduler: Scheduler<i32> = Scheduler::new();
        assert_eq!(scheduler.oplen(), 0);
    }

    #[test]
    fn add_to_oplog() {
        let mut scheduler : Scheduler<String>= Scheduler::new();
        scheduler.addop(event::Event::new(
            scheduler.oplen().clone(),
            "test".to_string(),
            scheduler.epoch,
            0,
            false,
        ));

        assert_eq!(scheduler.oplog[0].task_name, "test");
    }
}
