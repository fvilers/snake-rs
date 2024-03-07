#[derive(Default, Clone, PartialEq, Eq)]
pub struct Point {
    pub(crate) x: u16,
    pub(crate) y: u16,
}

impl Point {
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub const fn coords(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}
