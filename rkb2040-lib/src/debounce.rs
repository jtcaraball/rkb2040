use rp2040_hal::timer::Instant;

const QUARANTINE_MICROS: u64 = 10_000;

pub struct PinState {
    pub pressed: bool,
    pub debounce: PinDebouncer,
}

impl PinState {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            pressed: false,
            debounce: PinDebouncer::new(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct PinDebouncer {
    last_touch: Option<Instant>,
    quarentined: Option<bool>,
}

impl PinDebouncer {
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
        if diff.to_micros() < QUARANTINE_MICROS {
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
