mod task;
use task::*;

#[tokio::main]
async fn main() {
    let all_task_list = vec![
        Task {
            name: "".to_string(),
            duration: 1,
            start_time: 5,
        },
        Task {
            name: "".to_string(),
            duration: 1,
            start_time: 5,
        },
    ];

    let (task_list, ls) = TaskList::new(&all_task_list);
    for i in ls {
        i.join().unwrap()
    }
    println!("end");

    // run the first task
}
