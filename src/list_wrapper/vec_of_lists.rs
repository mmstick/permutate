use ListWrapper;

// implementation for lists of lists
impl<T> ListWrapper<Vec<T>> for Vec<&[T]>
where
    T: ?Sized + Copy,
{
    fn wrapper_len(&self) -> usize {
        let len = self.len();
        debug_assert!(len != 0);
        len
    }
    fn lens(&self) -> Vec<usize> {
        self.iter()
            .map(|list| {
                let len = list.len();
                debug_assert!(len != 0);
                len
            })
            .collect::<Vec<usize>>()
    }
    fn next_item(&self, indexes: &Vec<usize>) -> Vec<T> {
        // We are using `get_unchecked()` here because the incrementing
        // algorithim prohibits values from being out of bounds.
        indexes
            .iter()
            .enumerate()
            .map(|(list, value)| unsafe { *self.get_unchecked(list).get_unchecked(*value) })
            .collect::<Vec<T>>()
    }

    fn next_with_buffer(&self, indexes: &Vec<usize>, buffer: &mut Vec<T>) -> () {
        debug_assert!(
            buffer.len() >= self.wrapper_len(),
            "buffer is not large enough to contain the permutation"
        );

        let mut index = 0;
        // We are using `get_unchecked()` here because the incrementing
        // algorithim prohibits values from being out of bounds.
        unsafe {
            for outer_value in indexes
                .iter()
                .enumerate()
                .map(|(list, value)| *self.get_unchecked(list).get_unchecked(*value))
            {
                *buffer.get_unchecked_mut(index) = outer_value;
                index += 1;
            }
        };
    }
}
