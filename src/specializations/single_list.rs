use ListWrapper;

pub type OneSized<'a, T> = [&'a [&'a T]; 1];

// implementation for when it's a single list
impl<'a, T> ListWrapper<'a, Vec<&'a T>> for OneSized<'a, T>
where
    T: 'a + ?Sized,
{
    fn wrapper_len(&self) -> usize {
        let len = self[0].len();
        debug_assert!(len != 0);
        len
    }
    fn lens(&self) -> Vec<usize> {
        let nlists = self[0].len();
        debug_assert!(nlists != 0);
        (0..nlists).map(|_| nlists).collect::<Vec<usize>>()
    }
    fn next_item(&'a self, indexes: &Vec<usize>) -> Vec<&'a T> {
        indexes
            .iter()
            .map(|value| unsafe { *self[0].get_unchecked(*value) })
            .collect::<Vec<&T>>()
    }
}
