use crate::{
    avatar_window::AvatarWindow,
    backend::{Backend, BackendError, BackendWindow},
};
use std::collections::btree_map::{BTreeMap, Entry};

pub struct Windowing<B: Backend> {
    pub windows: BTreeMap<u32, AvatarWindow<B>>,
    backend: B,
}

impl<B: Backend> Windowing<B> {
    pub fn new() -> Self {
        Windowing {
            windows: BTreeMap::new(),
            backend: B::new(),
        }
    }
    pub fn window_for_char(&mut self, char: u32) -> Result<&mut AvatarWindow<B>, BackendError<B>> {
        if let Entry::Vacant(vacant) = self.windows.entry(char) {
            let inner = self.backend.new_window()?;
            let window = AvatarWindow::new(inner);
            vacant.insert(window);
        }
        Ok(self.windows.get_mut(&char).unwrap())
    }
    pub fn maintain(&mut self, frame: u64) {
        let mut windows_to_drop = Vec::new();
        for (char, window) in &mut self.windows {
            if !window.maintain(frame) {
                windows_to_drop.push(*char);
            }
        }
        for char in &windows_to_drop {
            self.windows.remove(char);
        }
    }
    pub fn poll_events(&mut self) -> bool {
        self.backend.poll_events()
    }
}
