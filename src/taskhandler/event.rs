#[derive(Debug, Clone)]
pub struct Event <T> {
    pub id: i32,
    // TODO : generics please
    pub task_name: T,
    pub epoch: i32,
    pub priority: i32,
    pub executed: bool,
}


impl<T> Event<T> {
    pub fn new(id: i32, task_name: T, epoch: i32, priority: i32, executed: bool) -> Self {
        Event {
            id: id,
            task_name: task_name,
            epoch: epoch,
            priority: priority,
            executed: executed, // TODO : set default to false
        }
    }

    pub fn togglex(&mut self) {
        if self.executed {
            self.executed = false;
        } else {
            self.executed = true;
        }
    }

    pub fn execute(&mut self) {
        if self.executed != true {
            //do stuff
            //mark as executed
            self.togglex();
        }
    }
}

#[cfg(test)]
mod traits {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn create_event() {
        let event : Event<String> = Event::new(1, "hello".to_string(), 1, 0, false);
        assert_eq!(event.task_name, "hello");
    }

    #[test]
    fn toggle_excuted() {
        let event = Arc::new(Mutex::new(Event::new(1, "hello".to_string(), 1, 0, false)));
        let e = event.clone();
        let mut e = e.lock().unwrap();
        let _ = e.togglex();
        assert_eq!(e.executed, true);
    }
}
