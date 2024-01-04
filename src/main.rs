mod animation;
mod assets;
mod context;
mod draw_settings;
mod player;
mod spritesheet;
use crate::context::RaylibContext;
use crate::player::{Player, State};
use animation::AnimationManager;
use assets::AssetManager;
use raylib::prelude::*;

fn main() {
    set_trace_log(TraceLogLevel::LOG_WARNING);
    let mut context = {
        let (rl, thread) = raylib::init().size(800, 600).build();

        RaylibContext::new(rl, thread)
    };

    context.rl.set_target_fps(60);
    let mut assets = AssetManager::new();

    let mut player_animation = AnimationManager::new(State::Idle);
    let player_idle = assets
        .load_spritesheet(&mut context, "assets/character_idle.png", 30, 0.01)
        .unwrap();
    let player_walk = assets
        .load_spritesheet(&mut context, "assets/character_walk.png", 27, 0.01)
        .unwrap();
    player_animation
        .add_state(State::Idle, player_idle)
        .add_state(State::Left, player_walk)
        .add_state(State::Right, player_walk)
        .add_state(State::Up, player_walk)
        .add_state(State::Down, player_walk);

    let mut player = Player::new(player_animation);

    while !context.rl.window_should_close() {
        player.update(&context.rl);
        let mut d = context.begin_drawing();
        d.clear_background(Color::WHITE);
        player.draw(&assets, &mut d);
        d.draw_fps(0, 0);
    }
}
