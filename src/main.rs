mod assets;
mod context;
mod draw_settings;
mod player;
mod spritesheet;
use crate::context::RaylibContext;
use crate::player::Player;
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
    let atlas_id = assets
        .load_texture(&mut context, "assets/sprites.png")
        .unwrap();

    let mut player = Player::new(&assets, atlas_id);

    while !context.rl.window_should_close() {
        player.update(&context.rl);
        let mut d = context.begin_drawing();
        d.clear_background(Color::WHITE);
        player.draw(&assets, &mut d);
        d.draw_fps(0, 0);
    }
}
