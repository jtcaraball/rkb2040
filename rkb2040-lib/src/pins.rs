use rp2040_hal::gpio::{FunctionSio, Pin, PinId, PullUp, SioInput};

use crate::debounce::PinState;

pub type KeyPin<Id> = Pin<Id, FunctionSio<SioInput>, PullUp>;

pub struct DirectKeyPin<Id: PinId> {
    pub pin: KeyPin<Id>,
    pub state: PinState,
}

impl<Id: PinId> DirectKeyPin<Id> {
    pub const fn new(pin: KeyPin<Id>) -> Self {
        Self {
            pin,
            state: PinState::new(),
        }
    }
}
