use rp2040_hal::gpio::{FunctionSio, Pin, PinId, PullUp, SioInput};
use rp2040_hal::timer::Instant;

pub struct PinState<const D: u64> {
    pub pressed: bool,
    pub debounce: PinDebouncer<D>,
}

impl<const D: u64> PinState<D> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            pressed: false,
            debounce: PinDebouncer::new(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct PinDebouncer<const D: u64> {
    last_touch: Option<Instant>,
    quarentined: Option<bool>,
}

impl<const D: u64> PinDebouncer<D> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            last_touch: None,
            quarentined: None,
        }
    }

    pub fn update(&mut self, now: Instant, state: bool) -> bool {
        let Some(earlier) = self.last_touch else {
            self.last_touch = Some(now);
            return true;
        };
        let Some(diff) = now.checked_duration_since(earlier) else {
            self.last_touch = Some(now);
            return true;
        };
        if diff.to_micros() < D {
            if self.quarentined != Some(state) {
                self.quarentined = Some(state);
                self.last_touch = Some(now);
            }
            return false;
        }

        self.last_touch = Some(now);
        self.quarentined.take();
        true
    }
}

pub type KeyPin<Id> = Pin<Id, FunctionSio<SioInput>, PullUp>;

pub struct DirectKeyPin<Id: PinId, const D: u64> {
    pub pin: KeyPin<Id>,
    pub state: PinState<D>,
}

impl<Id: PinId, const D: u64> DirectKeyPin<Id, D> {
    pub const fn new(pin: KeyPin<Id>) -> Self {
        Self {
            pin,
            state: PinState::new(),
        }
    }
}
