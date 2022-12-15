use macroquad::color::Color;
use macroquad::prelude::draw_rectangle;
use crate::Vec2I32;

/// Collider with x, y, width, and height that can check for AABB Collision Overlap with any other collider.
/// Can Be Disabled
#[derive(Clone, Debug)]
pub struct Collider {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub enabled: bool
}

impl Collider {
    /// Creates a collider with enabled true
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            enabled: true
        }
    }

    fn min_x(&self) -> i32 {
        self.x
    }
    fn min_y(&self) -> i32 {
        self.y
    }
    fn max_x(&self) -> i32 {
        self.x + self.width
    }
    fn max_y(&self) -> i32 {
        self.y + self.height
    }

    /// Checks for AABB Collision between collider and self with an offset
    pub fn is_overlapping(&self, offset: Vec2I32, other: &Collider) -> bool {
        if !(self.enabled && other.enabled) { return false };
        (self.max_x() + offset.x) > other.min_x() && (self.min_x() + offset.x) < other.max_x()
            && (self.min_y() + offset.y) < other.max_y() && (self.max_y() + offset.y) > other.min_y()
    }

    /// Draws a box to the screen with pos: x and y, size: width and height, and defined color
    pub fn debug_draw(&self, color: Color) {
        draw_rectangle(self.x as f32, self.y as f32, self.width as f32, self.height as f32, color)
    }
}