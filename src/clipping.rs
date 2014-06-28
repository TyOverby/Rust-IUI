use std::num::abs;

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64
}

pub struct ClippedRectangle {
    pub bounds: Rectangle,
    pub clipping_box: Rectangle
}

impl ClippedRectangle {
    pub fn contains(&self, pos: (f64, f64)) -> bool {
        self.bounds.contains(pos) && self.clipping_box.contains(pos)
    }
    pub fn is_part_visible(&self) -> bool {
        self.bounds.intersects(&self.clipping_box)
    }
}

pub fn raw_rect(x: f64, y: f64, w: f64, h: f64) -> ClippedRectangle {
    ClippedRectangle {
        bounds: Rectangle { x: x, y: y, w: w, h:h },
        clipping_box: Rectangle { x: x, y: y, w: w, h:h }
    }
}

impl Rectangle {
    pub fn contains(&self, pos: (f64, f64)) -> bool {
        let (x, y) = pos;
        if x < self.x || y < self.y {
            return false;
        }
        if x > self.x + self.w || y > self.y + self.h {
            return false;
        }
        return true;
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        (abs(self.x - other.x) * 2.0 < (self.w + other.w)) &&
        (abs(self.y - other.y) * 2.0 < (self.h + other.h))
    }
}
