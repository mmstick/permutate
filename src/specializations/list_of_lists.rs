use ListWrapper;

// implementation for lists of lists
impl<'a, T> ListWrapper<'a, Vec<&'a T>> for [&'a [&'a T]]
where
    T: 'a + ?Sized,
    &'a Self: Sized,
{
    fn wrapper_len(&'a self) -> usize {
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
    fn next_item(&'a self, indexes: &Vec<usize>) -> Vec<&'a T> {
        indexes
            .iter()
            .enumerate()
            .map(|(list, value)| unsafe { *self.get_unchecked(list).get_unchecked(*value) })
            .collect::<Vec<&T>>()
    }
}

// list_of_lists, but with option (WIP)
// impl<'a, T> ListWrapper<'a, Vec<Option<&'a T>>> for [&'a [&'a T]; 1]
// where T: 'a + ?Sized {
//     fn wrapper_len(&self) -> usize {
//         self[0].len()
//     }
//     fn lens(&self) -> Vec<usize> {
//         let nlists = self[0].len();
//         (0..nlists).map(|_| nlists).collect::<Vec<usize>>()
//     }
//     fn next_item(&'a self, indexes: std::slice::Iter<usize>) -> Vec<Option<&'a T>> {
//         indexes
//             .map(|value| self[0].get(*value).and_then(|t| Some(*t)))
//             .collect::<Vec<Option<&T>>>()
//     }
// }
