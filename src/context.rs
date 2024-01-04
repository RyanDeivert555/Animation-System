use raylib::prelude::*;

pub struct RaylibContext {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
}

impl RaylibContext {
    pub fn new(rl: RaylibHandle, thread: RaylibThread) -> Self {
        Self { rl, thread }
    }

    pub fn begin_drawing(&mut self) -> RaylibDrawHandle {
        self.rl.begin_drawing(&self.thread)
    }

    pub fn load_texture(&mut self, filename: &str) -> Result<Texture2D, String> {
        self.rl.load_texture(&self.thread, filename)
    }
}
