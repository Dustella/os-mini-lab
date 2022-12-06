use tokio::time::{sleep, Duration};

pub struct Task {
    pub duration: usize,
    pub name: String,
    pub need_men: usize,
}

impl Task {
    pub async fn run(&self) {
        println!("Running {}", self.name);
        sleep(Duration::from_secs(self.duration as u64)).await;
        println!("Finished {}", self.name);
    }
}
