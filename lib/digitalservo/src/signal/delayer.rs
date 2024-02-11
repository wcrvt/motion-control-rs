pub struct Delayer<T: Sized + Default + Copy, const N: usize> {
    buffer: [T; N],
    index: usize,
}

impl<T: Sized + Default + Copy, const N: usize> Delayer<T, N> {
    pub fn new() -> Self {
        Self {
            buffer: [T::default(); N],
            index: 0,
        }
    }

    pub fn output(&mut self, u: T) -> T {
        let out: T = self.buffer[self.index];
        self.buffer[self.index] = u;
        self.index = if self.index < (N - 1) {
            self.index + 1
        } else {
            0
        };

        out
    }

    pub fn reset(&mut self) {
        self.buffer = [T::default(); N];
        self.index = 0;
    }
}
