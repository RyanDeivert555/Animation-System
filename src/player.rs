use crate::{
    assets::{AssetManager, TextureId},
    draw_settings::DrawSettings,
    spritesheet::SpriteSheet,
};
use raylib::prelude::*;

#[derive(Debug, PartialEq)]
enum State {
    Idle,
    Down,
    Up,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Player {
    position: Vector2,
    color: Color,
    spritesheet: SpriteSheet,
    state: State,
}

impl Player {
    pub fn new(asset_manager: &AssetManager, atlas_id: TextureId) -> Self {
        Self {
            position: Vector2::new(100.0, 100.0),
            color: Color::WHITE,
            spritesheet: SpriteSheet::new(asset_manager, atlas_id, 4, 4, 0.15),
            state: State::Left,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        let dt = rl.get_frame_time();

        self.state = State::Idle;

        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.state = State::Up;
            self.position.y -= 100.0 * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            self.state = State::Down;
            self.position.y += 100.0 * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            self.state = State::Left;
            self.position.x -= 100.0 * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            self.state = State::Right;
            self.position.x += 100.0 * dt;
        }

        match self.state {
            State::Idle | State::Down => self.spritesheet.update_state(0),
            State::Up => self.spritesheet.update_state(1),
            State::Left => self.spritesheet.update_state(2),
            State::Right => self.spritesheet.update_state(3),
        }

        if self.state != State::Idle {
            self.spritesheet.animate(dt);
        } else {
            self.spritesheet.reset();
        }
    }

    pub fn draw(&self, asset_manager: &AssetManager, handle: &mut RaylibDrawHandle) {
        let settings = DrawSettings::new(
            self.position,
            Vector2::new(0.3, 0.3),
            0.0,
            Vector2::zero(),
            self.color,
        );

        asset_manager.draw_frame(handle, self.spritesheet, settings);
    }
}
