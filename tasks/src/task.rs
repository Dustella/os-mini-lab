use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, sleep, JoinHandle},
    time::Duration,
};

#[derive(Clone)]
pub struct Task {
    pub name: String,
    pub duration: u64,
    pub start_time: u64,
}

impl Task {
    fn run(self) {
        sleep(Duration::from_secs(self.duration));
    }
    fn get_response_ratio(&self, time_passed: u64) -> f64 {
        let pending_time = time_passed - self.start_time;
        ((pending_time + self.duration) / self.duration) as f64
    }
    fn Copy(self) -> Self {
        self
    }
}

pub struct TaskList {
    adder: Sender<Task>,
    getter: Receiver<Task>,
}

impl TaskList {
    pub fn new(ls: &Vec<Task>) -> (Self, Vec<JoinHandle<()>>) {
        let (adder, getter) = channel::<Task>();
        let mut add_queue = Vec::new();
        for e in ls.clone() {
            let adder = adder.clone();
            let sp = thread::spawn(move || {
                sleep(Duration::from_secs(e.clone().start_time));
                adder.send(e.clone()).unwrap();
            });
            add_queue.push(sp);
        }
        (Self { adder, getter }, add_queue)
    }
    pub fn get_max_ratio_task(&mut self, time_passed: u64) -> Task {
        let rec = self.getter.iter();
        let mut tasks: Vec<Task> = rec.into_iter().collect();
        tasks.sort_by(|a, b| {
            a.get_response_ratio(time_passed)
                .total_cmp(&b.get_response_ratio(time_passed))
        });
        let (max, left) = tasks.split_at(1);
        for i in left {
            self.adder.send(i.clone()).unwrap();
        }
        // let max = &rec
        //     .max_by(|a, b| {
        //         a.get_response_ratio(time_passed)
        //             .total_cmp(&b.get_response_ratio(time_passed))
        //     })
        //     .unwrap();
        // for i in self.getter.iter().filter(|x| x.name != max.clone().name) {
        //     self.adder.send(i).unwrap();
        // }
        max.first().unwrap().clone()
    }
}
