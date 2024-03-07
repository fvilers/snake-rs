use rand::Rng;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Point {
    pub(crate) x: u16,
    pub(crate) y: u16,
}

impl Point {
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn random(columns: u16, rows: u16) -> Self {
        let x = rand::thread_rng().gen_range(0..columns);
        let y = rand::thread_rng().gen_range(0..rows);

        Self::new(x, y)
    }

    pub const fn coords(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}
