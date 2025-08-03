use crate::queue::Queue;
use crate::scan::{KC, Scan};

const REPORT_QUEUE_LIMIT: usize = 16;
const PRESSED_MASK: u8 = 0b1000_0000;
const POS_MASK: u8 = 0b0111_1111;

#[derive(Clone, Copy)]
pub enum Keybind {
    NoOP,
    Key(KC),
    OneShot(KC),
    Combo(KC, KC),
    Layer(u8),
}

struct KeyState {
    last: Keybind,
    pressed: bool,
}

impl KeyState {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            last: Keybind::NoOP,
            pressed: false,
        }
    }
}

pub struct KeymapSM<const N: usize, const M: usize> {
    curr_scan: Scan,
    scans: Queue<Scan, REPORT_QUEUE_LIMIT>,
    key_state: [KeyState; N],
    keymap: [[Keybind; N]; M],
    changed: bool,
    layer: usize,
}

impl<const N: usize, const M: usize> KeymapSM<N, M> {
    #[must_use]
    pub fn new(keymap: [[Keybind; N]; M]) -> Self {
        Self {
            curr_scan: Scan::new(),
            scans: Queue::new(),
            key_state: core::array::from_fn(|_| KeyState::new()),
            changed: false,
            layer: 0,
            keymap,
        }
    }

    #[inline]
    pub fn begin_scan(&mut self) {
        self.curr_scan.reset();
        self.changed = false;
    }

    #[inline]
    pub fn register_press(&mut self, pos: u8) {
        let pressed = pos & PRESSED_MASK != 0;
        let pos = (pos & POS_MASK) as usize;
        if self.key_state[pos].pressed == pressed {
            return;
        }
        self.changed = true;
        self.key_state[pos].pressed = pressed;
        if !pressed {
            return;
        }
        self.key_state[pos].last = self.keymap[self.layer][pos];
    }

    #[inline]
    pub fn finish_scan(&mut self) {
        if !self.changed {
            return;
        }
        for state in &self.key_state {
            if !state.pressed {
                continue;
            }
            match state.last {
                Keybind::NoOP => {},
                Keybind::Key(k) => {
                    if self.curr_scan.add_key(k).is_err() {
                        break;
                    }
                }
                Keybind::OneShot(_) => {},
                Keybind::Combo(_, _) => {},
                Keybind::Layer(_) => {},
            }
        }
        let _ = self.scans.push(self.curr_scan);
    }

    #[inline]
    #[must_use]
    pub fn get_scan(&mut self) -> Option<&Scan> {
        self.scans.peek()
    }

    #[inline]
    pub fn clear_last_scan(&mut self) {
        let _ = self.scans.pop();
    }
}

#[macro_export]
macro_rules! KC {
    ($key:tt) => {
        $crate::keymap::Keybind::Key($crate::scan::KC::$key)
    };
}
