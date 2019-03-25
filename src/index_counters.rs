#[derive(Clone, Debug)]
/// Tracks the state of the indexes of each list.
pub struct IndexCounters {
    /// The current state of the indexes
    pub indexes: Vec<usize>,
    /// The lengths of the wrapped lists
    pub lens: Vec<usize>,
    /// The current iteration position
    pub curr_iter: usize,
    /// The maximum number of iterations to perform
    pub max_iters: usize,
}

impl IndexCounters {
    /// Increments & resets index indexes according to their maximum values.
    pub fn increment(&mut self, mut nlists: usize) {
        loop {
            let mut increment = false;
            {
                let current = unsafe { self.indexes.get_unchecked_mut(nlists) };
                let max = unsafe { self.lens.get_unchecked(nlists) };
                if *current + 1 >= *max {
                    if nlists != 0 {
                        *current = 0;
                        increment = true;
                    }
                } else {
                    *current += 1;
                }
            }

            if increment {
                nlists -= 1;
            } else {
                break;
            }
        }
    }

    pub fn reset(&mut self) {
        for value in self.indexes.iter_mut() {
            *value = 0;
        }
    }
}
