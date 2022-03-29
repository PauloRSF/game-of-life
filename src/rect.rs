use std::convert::TryInto;

pub type Point = (i32, i32);

#[derive(Clone)]
pub struct Rect {
    pub bottom_right: Point,
    pub top_left: Point,
}

impl Rect {
    pub fn contains(&self, (x, y): &Point) -> bool {
        x - self.bottom_right.0 >= 0
            && y - self.bottom_right.1 >= 0
            && self.top_left.0 - x > 0
            && self.top_left.1 - y > 0
    }

    pub fn area(&self) -> usize {
        ((self.top_left.0 - self.bottom_right.0) * (self.top_left.1 - self.bottom_right.1))
            .try_into()
            .unwrap()
    }

    pub fn point_to_index(&self, (x, y): &Point) -> usize {
        (x - self.bottom_right.0 + (self.top_left.0 * (y - self.bottom_right.1)))
            .try_into()
            .unwrap()
    }
}
