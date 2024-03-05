use crate::data::{Ident, Typing};
use std::collections::HashMap;

// #[derive(Debug)]
// pub enum RawTyping {
//     Int64,
//
//     Ident,
// }
//
// // TODO: Should typing and WrappedValue be merged? They seem to be doing the same thing...
// #[derive(Debug)]
// pub enum Typing {
//     Int64(i64),
//
//     Ident(Ident)
// }
// impl Typing {
//     pub fn raw(&self) -> RawTyping {
//         match self {
//             Self::Int64(_) => RawTyping::Int64
//         }
//     }
// }

pub type EnvironmentFrame = HashMap<Ident, Typing>;

/// offset functions only search the offset frame.
#[derive(Debug)]
pub struct Environment {
    pub frames: Vec<EnvironmentFrame>,
}
impl Environment {
    #[must_use]
    pub fn new() -> Self {
        // TODO: Default frame might need KOT constant.
        Self {
            frames: vec![EnvironmentFrame::new()],
        }
    }

    pub fn push(&mut self) {
        self.frames.push(EnvironmentFrame::new());
    }

    pub fn pop(&mut self) {
        self.frames.pop();
    }

    pub fn get(&self, id: &Ident) -> Option<&Typing> {
        for i in 0..self.frames.len() {
            let s = self.get_offset(id, i);
            if s.is_some() {
                return s;
            }
        }
        None
    }
    pub fn get_offset(&self, id: &Ident, offset: usize) -> Option<&Typing> {
        self.frames.get(self.index(offset)).and_then(|f| f.get(id))
    }

    pub fn contains(&self, id: &Ident) -> bool {
        for i in 0..self.frames.len() {
            let s = self.contains_offset(id, i);
            if s {
                return s;
            }
        }
        false
    }
    pub fn contains_offset(&self, id: &Ident, offset: usize) -> bool {
        self.frames
            .get(self.index(offset))
            .map_or(false, |f| f.contains_key(id))
    }

    /// Sets the var and returns the old value if any
    pub fn set(&mut self, id: Ident, data: Typing) -> Option<Typing> {
        self.set_offset(id, data, 0)
    }
    pub fn set_offset(&mut self, id: Ident, data: Typing, offset: usize) -> Option<Typing> {
        let i = self.index(offset);
        self.frames.get_mut(i).and_then(|f| f.insert(id, data))
    }

    pub fn remove(&mut self, id: &Ident) -> Option<Typing> {
        self.remove_offset(id, 0)
    }
    pub fn remove_offset(&mut self, id: &Ident, offset: usize) -> Option<Typing> {
        let i = self.index(offset);
        self.frames.get_mut(i).and_then(|f| f.remove(id))
    }

    // TODO: Should become debug assert?
    fn index(&self, offset: usize) -> usize {
        assert!(!self.frames.is_empty() && offset < self.frames.len());
        self.frames.len() - offset - 1
    }
}
impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
