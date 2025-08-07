use crate::queue::Queue;
use crate::seq::Seq;
use usbd_human_interface_device::page::Keyboard;

const SEQUENCE_LIMIT: usize = 16;
const REPORT_QUEUE_LIMIT: usize = 16;
const PRESSED_MASK: u8 = 0b1000_0000;
const POS_MASK: u8 = 0b0111_1111;
const OS_MASK: u8 = 0b0000_0001;

#[derive(Clone, Copy)]
pub enum Mod {
    LeftControl,
    LeftShift,
    LeftAlt,
    LeftGUI,
    RightControl,
    RightShift,
    RightAlt,
    RightGUI,
}

impl Mod {
    #[must_use]
    pub const fn to_key(self) -> Keyboard {
        match self {
            Self::LeftControl => Keyboard::LeftControl,
            Self::LeftShift => Keyboard::LeftShift,
            Self::LeftAlt => Keyboard::LeftAlt,
            Self::LeftGUI => Keyboard::LeftGUI,
            Self::RightControl => Keyboard::RightControl,
            Self::RightShift => Keyboard::RightShift,
            Self::RightAlt => Keyboard::RightAlt,
            Self::RightGUI => Keyboard::RightGUI,
        }
    }

    #[must_use]
    pub const fn to_byte(self) -> u8 {
        match self {
            Self::LeftControl => 0b0000_0001,
            Self::LeftShift => 0b0000_0010,
            Self::LeftAlt => 0b0000_0100,
            Self::LeftGUI => 0b0000_1000,
            Self::RightControl => 0b0001_0000,
            Self::RightShift => 0b0010_0000,
            Self::RightAlt => 0b0100_0000,
            Self::RightGUI => 0b1000_0000,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Keybind {
    NoOP,
    Key(Keyboard),
    Mod(Mod),
    OneShot(Mod),
    Combo(Mod, Keyboard),
    Layer(u8),
}

struct KeyState {
    last: Keybind,
    changed: bool,
    pressed: bool,
}

impl KeyState {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            last: Keybind::NoOP,
            changed: false,
            pressed: false,
        }
    }
}

struct OneShotSM {
    pressed: u8,
    active: u8,
    used: u8,
}

impl OneShotSM {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            pressed: 0,
            active: 0,
            used: 0,
        }
    }

    #[inline]
    pub fn any(&self) -> bool {
        self.active != 0
    }

    #[inline]
    pub fn add(&mut self, m: u8) {
        self.active |= m;
        self.pressed |= m;
    }

    #[inline]
    pub fn remove_if_used(&mut self, m: u8) {
        self.pressed &= 0b1111_1111 ^ m;
        if self.used & m == 0 {
            return;
        }
        self.active &= 0b1111_1111 ^ m;
        self.used &= 0b1111_1111 ^ m;
    }

    #[inline]
    pub fn apply<const N: usize>(&mut self, seq: &mut Seq<Keyboard, N>) {
        self.used |= self.active & self.pressed;
        let mut curr_mod: u8 = 0xE0; // 0xE0 - 0xE7 correspond to modifier keycodes.
        while self.active != 0 {
            if self.active & OS_MASK != 0 {
                let _ = seq.add(Keyboard::from(curr_mod));
            }
            curr_mod += 1;
            self.active >>= 1;
        }
    }
}

struct LayerSM {
    head: usize,
    stack: [usize; 256],
}

impl LayerSM {
    const LIMIT: usize = 1;

    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            head: 0,
            stack: [0; 256],
        }
    }

    #[inline]
    pub fn current_layer(&self) -> usize {
        self.stack[self.head]
    }

    #[inline]
    pub fn add(&mut self, v: usize) {
        self.head += 1;
        self.stack[self.head] = v;
    }

    #[inline]
    // Sadly we need to accommodate for a layer other than the top most one to be removed.
    pub fn remove(&mut self, v: usize) {
        let mut s = 0;
        for i in (Self::LIMIT..=self.head).rev() {
            if self.stack[i] == v {
                s = i;
                break;
            }
        }
        if s == 0 {
            return;
        }
        if s != self.head {
            unsafe {
                let ptr = self.stack.as_mut_ptr();
                let src_ptr = ptr.add(s + 1);
                let dest_ptr = ptr.add(s);
                core::ptr::copy(src_ptr, dest_ptr, self.head - s);
            }
        }
        self.head -= 1;
    }
}

pub struct KeymapSM<const N: usize, const M: usize> {
    key_state: [KeyState; N],
    keymap: [[Keybind; N]; M],
    curr_scan: Seq<Keyboard, SEQUENCE_LIMIT>,
    scans: Queue<Seq<Keyboard, SEQUENCE_LIMIT>, REPORT_QUEUE_LIMIT>,
    layer_stack: LayerSM,
    one_shots: OneShotSM,
    layer: usize,
}

impl<const N: usize, const M: usize> KeymapSM<N, M> {
    #[must_use]
    pub fn new(keymap: [[Keybind; N]; M]) -> Self {
        Self {
            keymap,
            curr_scan: Seq::new(),
            scans: Queue::new(),
            key_state: core::array::from_fn(|_| KeyState::new()),
            layer: 0,
            layer_stack: LayerSM::new(),
            one_shots: OneShotSM::new(),
        }
    }

    #[inline]
    pub fn begin_scan(&mut self) {
        self.layer = self.layer_stack.current_layer();
        self.curr_scan.reset();
    }

    #[inline]
    pub fn register_press(&mut self, pos: u8) {
        let pressed = pos & PRESSED_MASK != 0;
        let pos = (pos & POS_MASK) as usize;
        let state = &mut self.key_state[pos];
        if state.pressed == pressed {
            return;
        }
        // Only change keybind when pressed.
        if pressed {
            state.last = self.keymap[self.layer][pos];
        }
        // No ops should not trigger changes on being pressed or being released.
        state.changed = true;
        state.pressed = pressed;
    }

    #[inline]
    pub fn finish_scan(&mut self) {
        let mut non_mod_in_scan = false;
        let mut scan_is_reportable = false;
        for state in &mut self.key_state {
            match state.last {
                Keybind::NoOP => {}
                Keybind::Key(k) => {
                    scan_is_reportable |= state.changed;
                    if state.pressed {
                        non_mod_in_scan = true;
                        let _ = self.curr_scan.add(k);
                    }
                }
                Keybind::Mod(m) => {
                    scan_is_reportable |= state.changed;
                    if state.pressed {
                        let _ = self.curr_scan.add(m.to_key());
                    }
                }
                Keybind::Combo(m, k) => {
                    scan_is_reportable |= state.changed;
                    if state.pressed {
                        non_mod_in_scan = true;
                        let _ = self.curr_scan.add(m.to_key());
                        let _ = self.curr_scan.add(k);
                    }
                }
                Keybind::OneShot(m) => {
                    scan_is_reportable |= state.changed;
                    if state.pressed {
                        self.one_shots.add(m.to_byte());
                        let _ = self.curr_scan.add(m.to_key());
                    } else if state.changed {
                        self.one_shots.remove_if_used(m.to_byte());
                    }
                }
                Keybind::Layer(l) => {
                    if !state.changed {
                        continue;
                    }
                    if state.pressed {
                        self.layer_stack.add(l as usize);
                    } else {
                        self.layer_stack.remove(l as usize);
                    }
                }
            }
            state.changed = false;
        }
        if !scan_is_reportable {
            return;
        }
        if non_mod_in_scan && self.one_shots.any() {
            self.one_shots.apply(&mut self.curr_scan);
        }
        let _ = self.scans.push(self.curr_scan);
    }

    #[inline]
    #[must_use]
    pub fn get_scan(&mut self) -> Option<&Seq<Keyboard, SEQUENCE_LIMIT>> {
        self.scans.peek()
    }

    #[inline]
    pub fn clear_last_scan(&mut self) {
        let _ = self.scans.pop();
    }
}

macro_rules! impl_mods {
    ($(($alias:tt, $mod:tt),)*) => {
        paste::paste! {
            $(
                #[macro_export]
                macro_rules! $alias {
                    () => {
                        $crate::keymap::Keybind::Mod($crate::keymap::Mod::$mod)
                    };
                    ($_key:tt) => {
                        $crate::keymap::Keybind::Combo($crate::keymap::Mod::$mod, usbd_human_interface_device::page::Keyboard::$_key)
                    };
                }

                #[macro_export]
                macro_rules! [< OS _ $alias >] {
                    () => {
                        $crate::keymap::Keybind::OneShot($crate::keymap::Mod::$mod)
                    };
                }
            )*
        }
    };
}

impl_mods!(
    (LC, LeftControl),
    (LS, LeftShift),
    (LA, LeftAlt),
    (LG, LeftGUI),
    (RC, RightControl),
    (RS, RightShift),
    (RA, RightAlt),
    (RG, RightGUI),
);

#[macro_export]
macro_rules! KC {
    ($key:tt) => {
        $crate::keymap::Keybind::Key(usbd_human_interface_device::page::Keyboard::$key)
    };
}

#[macro_export]
macro_rules! MO {
    ($layer:tt) => {
        $crate::keymap::Keybind::Layer($layer)
    };
}

#[macro_export]
macro_rules! NA {
    () => {
        $crate::keymap::Keybind::NoOP
    };
}
