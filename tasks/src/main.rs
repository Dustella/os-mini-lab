mod task;
use task::*;

#[tokio::main]
async fn main() {
    let all_task_list = vec![
        Task {
            name: "".to_string(),
            duration: 5,
            start_time: 0,
        },
        Task {
            name: "".to_string(),
            duration: 5,
            start_time: 5,
        },
    ];
    let mut tasklist = TaskList::new();
    tasklist.start_tasks(&all_task_list).await;

    // for i in ls {
    //     i.join().unwrap()
    // }
    // println!("end");

    // run the first task
}
