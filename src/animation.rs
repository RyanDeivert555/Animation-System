#![allow(unused)]
use crate::assets::AssetManager;
use crate::draw_settings::DrawSettings;
use crate::spritesheet::SpriteSheet;
use raylib::drawing::RaylibDrawHandle;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub struct AnimationManager<State: Eq + Hash + Copy + Debug> {
    state_map: HashMap<State, SpriteSheet>,
    current_state: State,
}

impl<State: Eq + Hash + Copy + Debug> AnimationManager<State> {
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

    pub fn animate(&mut self, dt: f32) {
        self.state_map
            .entry(self.current_state)
            .and_modify(|ss| ss.animate(dt));
    }

    pub fn reset(&mut self) {
        self.state_map
            .entry(self.current_state)
            .and_modify(|ss| ss.reset());
    }

    pub fn current_frame(&self) -> Option<usize> {
        self.state_map
            .get(&self.current_state)
            .map(|ss| ss.current_frame)
    }

    pub fn is_done(&self) -> Option<bool> {
        self.state_map
            .get(&self.current_state)
            .map(|ss| ss.is_done())
    }

    pub fn draw(
        &self,
        handle: &mut RaylibDrawHandle,
        asset_manager: &AssetManager,
        settings: DrawSettings,
    ) {
        if let Some(ss) = self.state_map.get(&self.current_state) {
            asset_manager.draw_frame(handle, *ss, settings);
        } else {
            println!(
                "State {:?} does not have an animation mapped",
                self.current_state
            );
        }
    }
}
