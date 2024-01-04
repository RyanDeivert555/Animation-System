use raylib::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct DrawSettings {
    pub position: Vector2,
    pub scale: Vector2,
    pub flip_x: bool,
    pub flip_y: bool,
    pub rotation: f32,
    pub origin: Vector2,
    pub tint: Color,
}

impl DrawSettings {
    pub fn new(
        position: Vector2,
        scale: Vector2,
        flip_x: bool,
        flip_y: bool,
        rotation: f32,
        origin: Vector2,
        tint: Color,
    ) -> Self {
        Self {
            position,
            scale,
            flip_x,
            flip_y,
            rotation,
            origin,
            tint,
        }
    }
}
