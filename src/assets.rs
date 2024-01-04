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

        let src_rect = Rectangle::new(
            spritesheet.current_frame as f32 * spritesheet.frame_width,
            spritesheet.current_state as f32 * spritesheet.frame_height,
            spritesheet.frame_width,
            spritesheet.frame_height,
        );

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
