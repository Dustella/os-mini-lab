type Res = (usize, usize, usize, usize);
#[derive(Debug, Clone, Copy)]
pub struct Resource(pub usize, pub usize, pub usize, pub usize);

impl Resource {
    pub fn add(mut self, to_add: Resource) {
        self += to_add;
    }
    pub fn is_safe(self, res: Resource) -> bool {
        (res.0 <= self.0) && (res.1 <= self.1) && (res.2 <= self.2) && (res.3 <= self.3)
    }
    pub fn take(mut self, taken: Resource) {
        self -= taken;
    }
}

impl std::ops::Add<Resource> for Resource {
    type Output = Resource;
    fn add(self, rhs: Resource) -> Self::Output {
        Resource(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl std::ops::AddAssign<Resource> for Resource {
    fn add_assign(&mut self, rhs: Resource) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        self.3 += rhs.3;
    }
}

impl std::ops::Sub<Resource> for Resource {
    type Output = Resource;
    fn sub(self, rhs: Resource) -> Self::Output {
        Resource(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}
impl std::ops::SubAssign<Resource> for Resource {
    fn sub_assign(&mut self, rhs: Resource) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
        self.3 -= rhs.3;
    }
}
