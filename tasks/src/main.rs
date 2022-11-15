mod task;
use task::*;

#[tokio::main]
async fn main() {
    let all_task_list = vec![
        Task {
            name: "Task1".to_string(),
            duration: 12.0,
            start_time: 0.0,
        },
        Task {
            name: "Task2".to_string(),
            duration: 3.0,
            start_time: 5.0,
        },
        Task {
            name: "Task3".to_string(),
            duration: 0.6,
            start_time: 6.0,
        },
        Task {
            name: "Task4".to_string(),
            duration: 1.2,
            start_time: 8.0,
        },
    ];
    let tasklist = TaskList::new();
    tasklist.start_tasks(&all_task_list).await;

    // for i in ls {
    //     i.join().unwrap()
    // }
    // println!("end");

    // run the first task
}
