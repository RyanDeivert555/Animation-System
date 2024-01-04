#![allow(unused)]
use crate::assets::TextureId;
use raylib::texture::Texture2D;

// Single, linear spritesheet
#[derive(Debug, Copy, Clone)]
pub struct SpriteSheet {
    pub atlas_id: TextureId,
    pub frame_width: f32,
    pub frame_height: f32,
    pub frame_count: usize,
    pub current_frame: usize,
    pub current_time: f32,
    pub max_time: f32,
}

impl SpriteSheet {
    pub fn new(
        texture: &Texture2D,
        atlas_id: TextureId,
        frame_count: usize,
        animation_speed: f32,
    ) -> Self {
        let width = texture.width;
        let height = texture.height;

        let frame_width = width as f32 / frame_count as f32;
        let frame_height = height as f32;

        Self {
            atlas_id,
            frame_width,
            frame_height,
            frame_count,
            current_frame: 0,
            current_time: 0.0,
            max_time: animation_speed,
        }
    }

    pub fn animate(&mut self, dt: f32) {
        self.current_time += dt;
        if self.current_time >= self.max_time {
            self.current_time = 0.0;
            self.current_frame = (self.current_frame + 1) % self.frame_count;
        }
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.current_time = 0.0
    }

    pub fn done(&self) -> bool {
        self.current_frame == self.frame_count
    }
}
