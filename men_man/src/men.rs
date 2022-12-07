use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct Memory {
    // this stores free pages of memory
    free: Vec<(usize, usize)>,
    // this stores which pages are allocated to which task
    allocated: HashMap<String, Vec<(usize, usize)>>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            free: vec![(0, 1024)],
            allocated: HashMap::new(),
        }
    }
    // this function allocates memory pages to a task,
    // it returns the pages allocated
    pub fn allocate(&mut self, task: &str, size: usize) -> Option<Vec<(usize, usize)>> {
        let mut allocated = Vec::new();
        let mut free = Vec::new();
        let mut size = size;
        for (start, end) in self.free.clone() {
            if size == 0 {
                free.push((start, end));
                continue;
            }
            if end - start >= size {
                allocated.push((start, start + size));
                free.push((start + size, end));
                size = 0;
            } else {
                allocated.push((start, end));
                size -= end - start;
            }
        }
        if size == 0 {
            self.free = free;
            self.allocated.insert(task.to_string(), allocated.clone());
            Some(allocated)
        } else {
            None
        }
    }
    // this function deallocates memory pages from a task,
    // it returns the pages deallocated
    pub fn deallocate(&mut self, task: &str) -> Option<Vec<(usize, usize)>> {
        if let Some(allocated) = self.allocated.remove(task) {
            self.free.extend(allocated.clone());
            self.merge();
            Some(allocated)
        } else {
            None
        }
    }
    // merge pages that are next to each other
    pub fn merge(&mut self) {
        println!("before merge");
        self.free.sort_by(|a, b| a.0.cmp(&b.0));
        dbg!(&self.free);
        let mut merged = Vec::new();
        let mut last = self.free[0];
        for (index, (start, end)) in self.free.clone().iter().enumerate() {
            if *start == last.1 {
                last.1 = *end;
            } else {
                if index != 0 {
                    merged.push(last);
                    last = (*start, *end);
                }
            }
        }
        merged.push(last);
        // removes the same element in the list
        merged.dedup();
        println!("after merge");
        dbg!(&merged);
        println!("====================");
        self.free = merged;
    }
}
