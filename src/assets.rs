#![allow(unused)]
use crate::{context::RaylibContext, draw_settings::DrawSettings, spritesheet::SpriteSheet};
use raylib::prelude::*;

#[derive(Default, Debug)]
pub struct AssetManager {
    textures: Vec<Texture2D>,
}

pub type TextureId = usize;

impl AssetManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_texture(
        &mut self,
        context: &mut RaylibContext,
        filename: &str,
    ) -> Result<TextureId, String> {
        let texture = context.load_texture(filename)?;
        self.textures.push(texture);

        Ok(self.textures.len() - 1)
    }

    pub fn load_spritesheet(
        &mut self,
        context: &mut RaylibContext,
        filename: &str,
        frame_count: usize,
        animation_speed: f32,
    ) -> Result<SpriteSheet, String> {
        let atlas = context.load_texture(filename)?;
        // id will be correct after pushing texture
        let id = self.textures.len();
        let spritesheet = SpriteSheet::new(&atlas, id, frame_count, animation_speed);
        self.textures.push(atlas);

        Ok(spritesheet)
    }

    pub fn texture(&self, id: TextureId) -> Option<&Texture2D> {
        self.textures.get(id)
    }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub fn draw(
        &self,
        handle: &mut RaylibDrawHandle,
        id: TextureId,
        position: Vector2,
        scale: Vector2,
        rotation: f32,
        origin: Vector2,
        tint: Color,
    ) {
        let texture = self.texture(id).unwrap();
        let width = texture.width as f32;
        let height = texture.height as f32;

        let src_rect = Rectangle::new(0.0, 0.0, width, height);
        let dest_rect = Rectangle::new(position.x, position.y, width * scale.x, height * scale.y);

        handle.draw_texture_pro(texture, src_rect, dest_rect, origin, rotation, tint);
    }

    #[allow(dead_code)]
    pub fn draw_with_settings(
        &self,
        handle: &mut RaylibDrawHandle,
        id: TextureId,
        settings: DrawSettings,
    ) {
        self.draw(
            handle,
            id,
            settings.position,
            settings.scale,
            settings.rotation,
            settings.origin,
            settings.tint,
        );
    }

    #[allow(dead_code)]
    pub fn draw_frame(
        &self,
        handle: &mut RaylibDrawHandle,
        spritesheet: SpriteSheet,
        settings: DrawSettings,
    ) {
        let texture = self.texture(spritesheet.atlas_id).unwrap();

        let src_rect = {
            let width = if settings.flip_y {
                -spritesheet.frame_width
            } else {
                spritesheet.frame_width
            };
            let height = if settings.flip_x {
                -spritesheet.frame_height
            } else {
                spritesheet.frame_height
            };

            Rectangle::new(
                spritesheet.current_frame as f32 * spritesheet.frame_width,
                spritesheet.frame_height,
                width,
                height,
            )
        };

        let dest_rect = Rectangle::new(
            settings.position.x,
            settings.position.y,
            spritesheet.frame_width * settings.scale.x,
            spritesheet.frame_height * settings.scale.y,
        );

        handle.draw_texture_pro(
            texture,
            src_rect,
            dest_rect,
            settings.origin,
            settings.rotation,
            settings.tint,
        );
    }
}
