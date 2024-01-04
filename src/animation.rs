use raylib::drawing::RaylibDrawHandle;
use std::collections::HashMap;
use std::hash::Hash;

use crate::assets::AssetManager;
use crate::draw_settings::DrawSettings;
use crate::spritesheet::SpriteSheet;

#[derive(Debug)]
pub struct AnimationManager<State: Eq + Hash> {
    state_map: HashMap<State, SpriteSheet>,
    current_state: State,
}

impl<State: Eq + Hash> AnimationManager<State> {
    pub fn new(initial_state: State) -> Self {
        Self {
            state_map: HashMap::new(),
            current_state: initial_state,
        }
    }

    pub fn add_state(&mut self, new_state: State, spritesheet: SpriteSheet) -> &mut Self {
        self.state_map.insert(new_state, spritesheet);
        self
    }

    pub fn change_state(&mut self, new_state: State) {
        self.current_state = new_state;
    }

    pub fn draw(
        &self,
        handle: &mut RaylibDrawHandle,
        asset_manager: &AssetManager,
        settings: DrawSettings,
    ) {
        let spritesheet = self.state_map.get(&self.current_state).unwrap();
        asset_manager.draw_frame(handle, *spritesheet, settings);
    }
}
