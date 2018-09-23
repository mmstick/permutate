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
            }).collect::<Vec<usize>>()
    }
    fn next_item(&'a self, indexes: &Vec<usize>) -> Vec<&'a T> {
        indexes
            .iter()
            .enumerate()
            .map(|(list, value)| unsafe { *self.get_unchecked(list).get_unchecked(*value) })
            .collect::<Vec<&T>>()
    }
}

// same, but with option (WIP)
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

// single tuple
// impl<'a, A> ListWrapper<'a, (&'a A)> for (&'a [&'a A],)
// where
//     A: 'a + ?Sized,
//     &'a Self: Sized,
// {
//     fn wrapper_len(&'a self) -> usize {
//         1
//     }
//     fn lens(&self) -> Vec<usize> {
//         debug_assert!(self.len() != 0);
//         vec![self.len()]
//     }
//     fn next_item(&'a self, indexes: &Vec<usize>) -> (&'a A) {
//         unsafe {
//             (
//                 *self.get_unchecked(indexes[0])
//             )
//         }
//     }
// }

// double tuples (for reference)
// impl<'a, A, B> ListWrapper<'a, (&'a A, &'a B)> for (&'a [&'a A], &'a [&'a B])
// where
//     A: 'a + ?Sized,
//     B: 'a + ?Sized,
//     &'a Self: Sized,
// {
//     fn wrapper_len(&'a self) -> usize {
//         2
//     }
//     fn lens(&self) -> Vec<usize> {
//         let ls = vec![self.0.len(), self.1.len()];
//         ls.iter().for_each(|l| debug_assert!(*l != 0));
//         ls
//     }
//     fn next_item(&'a self, indexes: &Vec<usize>) -> (&'a A, &'a B) {
//         unsafe {
//             (
//                 *self.0.get_unchecked(indexes[0]),
//                 *self.1.get_unchecked(indexes[1])
//             )
//         }
//     }
// }

// reference: https://doc.rust-lang.org/src/core/tuple.rs.html
macro_rules! tuple_impls {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(
            impl<'a, $($T),+> ListWrapper<'a, ($(&'a $T),+)> for ($(&'a [&'a $T],)+)
            where
                $($T: 'a + ?Sized,)+
                &'a Self: Sized,
            {
                fn wrapper_len(&'a self) -> usize {
                    tuple_impls!(@last_idx $($idx,)+) + 1
                }
                fn lens(&self) -> Vec<usize> {
                    let ls = vec![$(self.$idx.len()),+];
                    ls.iter().for_each(|l| debug_assert!(*l != 0));
                    ls
                }
                fn next_item(&'a self, indexes: &Vec<usize>) -> ($(&'a $T),+) {
                    unsafe {
                        (
                            $(*self.$idx.get_unchecked(indexes[$idx])),+
                        )
                    }
                }
            }


        )+
    };

    // this returns the last idx from a list of idx
    (@last_idx $a:tt,) => { $a };
    (@last_idx $a:tt, $($rest_a:tt,)+) => { tuple_impls!(@last_idx $($rest_a,)+) };
    // additional reference: https://danielkeep.github.io/tlborm/book/pat-internal-rules.html
}


tuple_impls! {
    Tuple1 {
        (0) -> A
    }
    Tuple2 {
        (0) -> A
        (1) -> B
    }
    Tuple3 {
        (0) -> A
        (1) -> B
        (2) -> C
    }
    Tuple4 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
    }
    Tuple5 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
    }
    Tuple6 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
    }
    Tuple7 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
    }
    Tuple8 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
    }
    Tuple9 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
    }
    Tuple10 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
    }
    Tuple11 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
    }
    Tuple12 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
        (11) -> L
    }
}
