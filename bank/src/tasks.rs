use crate::res::Resource;

pub struct Task {
    pub name: String,
    pub allocated: Resource,
    pub max: Resource,
}

impl Task {
    fn get_need(self) -> Resource {
        self.max - self.allocated
    }
    fn consume(self) {
        println!("Running {}", self.name);
        println!("Freed Res {:?}", self.max)
    }
}
