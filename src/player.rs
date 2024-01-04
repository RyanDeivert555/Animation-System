use crate::{animation::AnimationManager, assets::AssetManager, draw_settings::DrawSettings};
use raylib::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum State {
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
    state: State,
    animation: AnimationManager<State>,
}

impl Player {
    pub fn new(animation: AnimationManager<State>) -> Self {
        Self {
            position: Vector2::new(100.0, 100.0),
            color: Color::WHITE,
            animation,
            state: State::Left,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        let dt = rl.get_frame_time();
        let speed = 250.0;

        self.state = State::Idle;

        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.state = State::Up;
            self.position.y -= speed * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            self.state = State::Down;
            self.position.y += speed * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            self.state = State::Left;
            self.position.x -= speed * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            self.state = State::Right;
            self.position.x += speed * dt;
        }

        self.animation.change_state(self.state);
        self.animation.animate(dt);
    }

    pub fn draw(&self, asset_manager: &AssetManager, handle: &mut RaylibDrawHandle) {
        let mut settings = DrawSettings::new(
            self.position,
            Vector2::new(2.0, 2.0),
            false,
            false,
            0.0,
            Vector2::zero(),
            self.color,
        );
        if self.state == State::Left {
            settings.flip_y = true;
        }

        self.animation.draw(handle, asset_manager, settings);
    }
}
