use futures::{future::join_all, lock};
use std::{cmp::Ordering, sync::Arc, time::Duration};
use tokio::{
    sync::Mutex,
    time::{self, sleep},
};

#[derive(Clone)]
pub struct Task {
    pub name: String,
    pub duration: u64,
    pub start_time: u64,
}

impl Task {
    pub async fn run(self) {
        println!("Running {}", self.clone().name);
        sleep(Duration::from_secs(self.duration)).await;
    }
    pub fn get_response_ratio(&self, time_passed: u64) -> f64 {
        let pending_time = time_passed - self.start_time;
        ((pending_time + self.duration) / self.duration) as f64
    }
}

pub struct TaskList {
    list: Arc<Mutex<Vec<Task>>>,
    timer: time::Instant,
}

impl TaskList {
    pub fn new() -> Self {
        let list = Arc::new(Mutex::new(Vec::new()));
        let timer = time::Instant::now();
        Self { list, timer }
    }
    pub async fn start_tasks(mut self, tasks: &Vec<Task>) {
        let mut joined_jobs = Vec::new();
        for e in tasks.clone() {
            let list_lock = self.list.clone();
            let sp = tokio::spawn(async move {
                sleep(Duration::from_secs(e.clone().start_time)).await;
                let mut task_list = list_lock.lock().await;
                // push task
                println!(
                    "{}s passed, push in task {}.",
                    e.clone().start_time,
                    e.clone().name
                );
                task_list.push(e);
            });
            joined_jobs.push(sp);
        }
        let main_task = tokio::spawn(async move {
            println!("lala");
            sleep(Duration::from_secs(1)).await;
            while let Some(task) = self.get_max_ratio_task().await {
                task.run().await;
            }
        });
        joined_jobs.push(main_task);
        join_all(joined_jobs).await;
        println!("All task finished")
    }
    pub async fn get_max_ratio_task(&mut self) -> Option<Task> {
        let time_passed = self.timer.elapsed().as_secs();
        let mut locked_list = self.list.lock().await;
        // let mut tasks: Vec<Task> = rec.into_iter().collect();
        locked_list.sort_by(|a, b| {
            if (a.get_response_ratio(time_passed) - b.get_response_ratio(time_passed))
                .is_sign_negative()
            {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        let res = locked_list.pop();

        drop(locked_list);
        res
    }
}
