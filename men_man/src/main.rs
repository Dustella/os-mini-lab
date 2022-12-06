mod task;
use rand::Rng;
use task::*;
mod men;
use men::*;
#[tokio::main]
async fn main() {
    let mut task_list = Vec::new();
    // prepare tasks with randomized duration, men need and name
    // duplicate
    let mut rng = rand::thread_rng();
    for i in 0..10 {
        let task = Task {
            name: format!("Task {}", i),
            duration: rng.gen_range(1, 10),
            need_men: rng.gen_range(0, 10),
        };
        task_list.push(task);
    }
    // create tasks
    let mut a = Memory::new();
    a.allocate("a", 10);
    a.allocate("b", 10);
    a.allocate("c", 10);
    a.allocate("d", 10);
    a.deallocate("a");
    a.deallocate("b");
    a.allocate("3", 100);
    dbg!(&a);
}
