use core::mem::MaybeUninit;

pub struct QueueErr;

pub type Result<T> = core::result::Result<T, QueueErr>;

pub struct Queue<T, const N: usize> {
    head: usize,
    tail: usize,
    buf: [MaybeUninit<T>; N],
}

impl<T, const N: usize> Queue<T, N> {
    const NULL: MaybeUninit<T> = MaybeUninit::uninit();

    /// # Errors
    ///
    /// Returs `QueueErr` if the queue is full.
    #[inline]
    pub fn push(&mut self, val: T) -> Result<()> {
        let next = (self.tail + 1) % N;
        if next == self.head {
            return Err(QueueErr);
        }
        unsafe {
            self.buf
                .get_unchecked_mut(self.tail)
                .as_mut_ptr()
                .write(val);
        }
        self.tail = next;
        Ok(())
    }

    #[inline]
    pub fn peek(&self) -> Option<&T> {
        if self.head == self.tail {
            return None;
        }
        unsafe { self.buf.get_unchecked(self.head).as_ptr().as_ref() }
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.head == self.tail {
            return None;
        }
        let val = unsafe { self.buf.get_unchecked_mut(self.head).as_ptr().read() };
        self.head = (self.head + 1) % N;
        Some(val)
    }

    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            buf: [Self::NULL; N],
            head: 0,
            tail: 0,
        }
    }
}
