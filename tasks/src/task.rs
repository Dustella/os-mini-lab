use core::fmt;
use futures::future::join_all;
use std::{cmp::Ordering, sync::Arc, time::Duration};
use tokio::{
    sync::Mutex,
    time::{self, sleep},
};

#[derive(Clone, Debug)]
pub struct Task {
    pub name: String,
    pub duration: f64,
    pub start_time: f64,
}
impl Task {
    pub async fn run(self) {
        println!("Running {}", self.clone().name);
        sleep(Duration::from_secs_f64(self.duration)).await;
        println!("Finished {}", self.clone().name);
    }
    pub fn get_response_ratio(&self, time_passed: f64) -> f64 {
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
                sleep(Duration::from_secs_f64(e.clone().start_time)).await;
                let mut task_list = list_lock.lock().await;
                // push task
                println!(
                    "{}s passed, {} is added to the queue.",
                    e.clone().start_time,
                    e.clone().name
                );
                task_list.push(e);
            });
            joined_jobs.push(sp);
        }
        let main_task = tokio::spawn(async move {
            println!("Start");
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
        let time_passed = self.timer.elapsed().as_secs_f64();
        println!("开始执行调度算法：");
        println!("现在剩余任务中，有：");
        let mut locked_list = self.list.lock().await;
        // let mut tasks: Vec<Task> = rec.into_iter().collect();
        locked_list.sort_by(|a, b| {
            if (a.get_response_ratio(time_passed) - b.get_response_ratio(time_passed))
                .is_sign_negative()
            {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        for i in locked_list.clone() {
            print!(
                "任务{}, 响应比{}; ",
                i.name,
                i.get_response_ratio(time_passed)
            );
        }
        println!();
        let res = locked_list.pop();

        drop(locked_list);
        res
    }
}
