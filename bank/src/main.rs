mod res;
use res::*;
mod tasks;
use tasks::*;

fn main() {
    let mut init_res = Resource(1, 6, 2, 2);
    let mut task_list = vec![
        Task {
            name: "1".to_string(),
            max: Resource(0, 0, 4, 4),
            allocated: Resource(0, 0, 3, 2),
        },
        Task {
            name: "2".to_string(),
            max: Resource(2, 7, 5, 0),
            allocated: Resource(1, 0, 0, 0),
        },
        Task {
            name: "3".to_string(),
            max: Resource(3, 6, 10, 10),
            allocated: Resource(1, 3, 5, 4),
        },
        Task {
            name: "4".to_string(),
            max: Resource(0, 6, 6, 10),
            allocated: Resource(0, 0, 1, 4),
        },
    ];

    println!("Hello, world!");
}

fn iter(ls: &mut Vec<Resource>, init: &mut Resource) -> Result<(), ()> {
    while ls.len() != 0 {
        let safe: Vec<Resource> = ls
            .iter()
            .filter(|&i| i.is_safe(init.clone()))
            .map(|a| *a)
            .collect();
        if safe.len() == 0 {
            return Err(());
        }
    }
    Ok(())
}
