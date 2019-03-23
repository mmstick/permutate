use ListWrapper;

// single tuple (for reference)
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
