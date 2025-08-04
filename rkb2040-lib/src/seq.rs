pub struct SeqErr;

pub type Result<T> = core::result::Result<T, SeqErr>;

#[derive(Copy, Clone)]
pub struct Seq<T, const N: usize>
where
    T: Default + Copy,
{
    pub count: usize,
    values: [T; N],
}

impl<T, const N: usize> Default for Seq<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Seq::new()
    }
}

impl<T, const N: usize> Seq<T, N>
where
    T: Default + Copy,
{
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            count: 0,
            values: core::array::from_fn(|_| T::default()),
        }
    }

    /// # Errors
    ///
    /// Returns `SeqErr` if the current scan is full.
    #[inline]
    pub fn add(&mut self, val: T) -> Result<()> {
        if self.count >= N {
            return Err(SeqErr);
        }
        self.values[self.count] = val;
        self.count += 1;
        Ok(())
    }

    #[inline]
    pub fn reset(&mut self) {
        self.count = 0;
    }

    #[expect(dead_code)]
    fn iter(&self) -> SeqIterator<T, N> {
        <&Self as IntoIterator>::into_iter(self)
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a Seq<T, N>
where
    T: Default + Copy,
{
    type Item = T;
    type IntoIter = SeqIterator<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        SeqIterator {
            index: 0,
            scan: self,
        }
    }
}

impl<T, const N: usize> IntoIterator for Seq<T, N>
where
    T: Default + Copy,
{
    type Item = T;
    type IntoIter = SeqToIterator<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        SeqToIterator {
            index: 0,
            scan: self,
        }
    }
}

pub struct SeqIterator<'a, T, const N: usize>
where
    T: Default + Copy,
{
    index: usize,
    scan: &'a Seq<T, N>,
}

impl<T, const N: usize> Iterator for SeqIterator<'_, T, N>
where
    T: Default + Copy,
{
    type Item = T;

    fn next(&mut self) -> core::option::Option<Self::Item> {
        if self.index >= self.scan.count {
            return None;
        }
        self.index += 1;
        Some(self.scan.values[self.index - 1])
    }
}

pub struct SeqToIterator<T, const N: usize>
where
    T: Default + Copy,
{
    index: usize,
    scan: Seq<T, N>,
}

impl<T, const N: usize> Iterator for SeqToIterator<T, N>
where
    T: Default + Copy,
{
    type Item = T;

    fn next(&mut self) -> core::option::Option<Self::Item> {
        if self.index >= self.scan.count {
            return None;
        }
        self.index += 1;
        Some(self.scan.values[self.index - 1])
    }
}
