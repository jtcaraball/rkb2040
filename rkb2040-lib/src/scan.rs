pub type KC = usbd_human_interface_device::page::Keyboard;

pub const MAX_SCAN_SIZE: usize = 12;

pub struct ScanErr;

pub type Result<T> = core::result::Result<T, ScanErr>;

#[derive(Default, Copy, Clone)]
pub struct Scan {
    pub count: usize,
    keys: [KC; MAX_SCAN_SIZE],
}

impl Scan {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            count: 0,
            keys: [KC::NoEventIndicated; MAX_SCAN_SIZE],
        }
    }

    /// # Errors
    ///
    /// Returns `ScanErr` if the current scan is full.
    #[inline]
    pub fn add_key(&mut self, key: KC) -> Result<()> {
        if self.count >= MAX_SCAN_SIZE {
            return Err(ScanErr);
        }
        self.keys[self.count] = key;
        self.count += 1;
        Ok(())
    }

    #[inline]
    pub fn reset(&mut self) {
        self.count = 0;
    }

    #[expect(dead_code)]
    fn iter(&self) -> ScanIterator {
        <&Self as IntoIterator>::into_iter(self)
    }
}

impl<'a> IntoIterator for &'a Scan {
    type Item = KC;
    type IntoIter = ScanIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ScanIterator {
            index: 0,
            scan: self,
        }
    }
}

impl IntoIterator for Scan {
    type Item = KC;
    type IntoIter = ScanToIterator;

    fn into_iter(self) -> Self::IntoIter {
        ScanToIterator {
            index: 0,
            scan: self,
        }
    }
}

pub struct ScanIterator<'a> {
    index: usize,
    scan: &'a Scan,
}

impl Iterator for ScanIterator<'_> {
    type Item = KC;

    fn next(&mut self) -> core::option::Option<Self::Item> {
        if self.index >= self.scan.count {
            return None;
        }
        self.index += 1;
        Some(self.scan.keys[self.index - 1])
    }
}

pub struct ScanToIterator {
    index: usize,
    scan: Scan,
}

impl Iterator for ScanToIterator {
    type Item = KC;

    fn next(&mut self) -> core::option::Option<Self::Item> {
        if self.index >= self.scan.count {
            return None;
        }
        self.index += 1;
        Some(self.scan.keys[self.index - 1])
    }
}
