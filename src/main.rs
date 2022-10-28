use std::sync::Arc;
use tokio::sync::{ Semaphore};
use tokio::time::{sleep, Duration};

async fn consumer(
    mutex: Arc<Semaphore>,
    full: Arc<Semaphore>,
    empty:Arc<Semaphore>
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let full_lock = full.acquire_owned().await.unwrap();
    let mutex_lock = mutex.acquire_owned().await.unwrap();
    sleep(Duration::from_secs(1)).await;
    empty.add_permits(1);
    println!("consumed one product");
    drop(mutex_lock);
    drop(full_lock);
    Ok(())
}

async fn provider(
    mutex: Arc<Semaphore>,
    empty: Arc<Semaphore>,
    full:Arc<Semaphore>
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let empty_lock = empty.acquire_owned().await.unwrap();
    let mutex_lock= mutex.acquire_owned().await.unwrap();
    sleep(Duration::from_secs(1)).await;
    full.add_permits(1);
    println!("provide one product");
    drop(mutex_lock);
    drop(empty_lock);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mutex = Arc::new(Semaphore::new(1));
    let empty = Arc::new(Semaphore::new(5));
    let full = Arc::new(Semaphore::new(0));

    let mut task_list = Vec::new();

    // genrerate provider
    for _ in 1..10 {
        let task = tokio::spawn(provider(mutex.clone(), empty.clone(),full.clone()));
        task_list.push(task);
    }
    
    for _ in 1..10 {
        let task = tokio::spawn(consumer(mutex.clone(), full.clone(),empty.clone()));
        task_list.push(task);
    }
    
    futures::future::join_all(task_list).await;

    Ok(())
}
