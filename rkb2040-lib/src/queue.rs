pub struct QueueErr;

pub type Result<T> = core::result::Result<T, QueueErr>;

pub struct Queue<T, const N: usize>
where
    T: Default + Copy,
{
    head: usize,
    tail: usize,
    buf: [T; N],
}

impl<T, const N: usize> Queue<T, N>
where
    T: Default + Copy,
{
    /// # Errors
    ///
    /// Returs `QueueErr` if the queue is full.
    #[inline]
    pub fn push(&mut self, val: T) -> Result<()> {
        let next = (self.tail + 1) % N;
        if next == self.head {
            return Err(QueueErr);
        }
        self.buf[self.tail] = val;
        self.tail = next;
        Ok(())
    }

    #[inline]
    pub fn peek(&self) -> Option<&T> {
        if self.head == self.tail {
            return None;
        }
        Some(&self.buf[self.head])
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.head == self.tail {
            return None;
        }
        let val = self.buf[self.head];
        self.head = (self.head + 1) % N;
        Some(val)
    }

    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            buf: [T::default(); N],
            head: 0,
            tail: 0,
        }
    }
}
