use std::{
    sync::{Arc, Condvar, Mutex},
    thread::{self, sleep},
    time::Duration,
};

#[derive(Debug)]
struct Product {
    count: u32,
    full: Arc<(Mutex<bool>, Condvar)>,
    empty: Arc<(Mutex<bool>, Condvar)>,
}
impl Product {
    pub fn new(count: u32) -> Self {
        let full = Arc::new((Mutex::new(false), Condvar::new()));
        let empty = Arc::new((Mutex::new(false), Condvar::new()));
        Product { count, full, empty }
    }
    fn insert(&mut self) {
        let (full_lock, full_condvar) = &*self.full;
        let (_empty_lock, empty_condvar) = &*self.empty;
        if self.count == 5 {
            let _guard = full_condvar
                .wait_while(full_lock.lock().unwrap(), |pending| *pending)
                .unwrap();
            let mut lock = full_lock.lock().unwrap();
            *lock = false;
        }
        println!("produce one product, {} left", self.count);
        self.count += 1;
        if self.count == 1 {
            empty_condvar.notify_one();
        }
    }
    fn consume(&mut self) {
        let (_full_lock, full_condvar) = &*self.full;
        let (empty_lock, empty_condvar) = &*self.empty;
        if self.count == 0 {
            let _guard = empty_condvar
                .wait_while(empty_lock.lock().unwrap(), |pending| *pending)
                .unwrap();
            let mut lock = empty_lock.lock().unwrap();
            *lock = false;
        }
        self.count -= 1;
        println!("consumed one product, {} left", self.count);
        if self.count == 4 {
            full_condvar.notify_one();
        }
    }
}

fn main() {
    let _cond = Arc::new((Mutex::new(false), Condvar::new()));

    // thread_local! {static p :Product= Product::new(1)}
    let mut p = Product::new(1);
    // let mut p2 = p.as_ref();
    // let mut p2 = p.clone();
    let mut task_list = Vec::new();
    produce(&mut p);
    let task = thread::spawn(move || consume(&mut p)).join().unwrap();
    task_list.push(task)
}

fn consume(p: &mut Product) {
    sleep(Duration::from_secs(1));
    p.consume();
    sleep(Duration::from_secs(1));
    p.consume();
    sleep(Duration::from_secs(1));
    p.consume();
    sleep(Duration::from_secs(1));
    p.consume();
    sleep(Duration::from_secs(1));
    p.consume();
}

fn produce(p: &mut Product) {
    sleep(Duration::from_secs(1));
    p.insert();
    sleep(Duration::from_secs(1));
    p.insert();
    sleep(Duration::from_secs(1));
    p.insert();
}
