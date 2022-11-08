use std::{sync::Arc};

use tokio::{sync::{Semaphore}, time::{sleep,Duration}};

async fn consume(seme:Arc<Semaphore>){
    println!("from lala{}, brfore dropping",seme.available_permits());
    let lock = seme.acquire_owned().await.unwrap();
    sleep(Duration::from_micros(1000000)).await;
    println!("begin to drop");
    drop(lock);
}

#[tokio::main]
async fn main(){
    let sema = Arc::new(Semaphore::new(3));
    let mut task_list = Vec::new();
    println!("init");
    for _ in 0..10{
        let owned = sema.clone();
        let task = tokio::spawn(consume(owned));
        task_list.push(task)
    }
    for i in task_list{
        i.await.unwrap();
    }
}