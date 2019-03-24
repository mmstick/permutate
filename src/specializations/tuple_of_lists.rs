use ListWrapper;

// reference: https://doc.rust-lang.org/src/core/tuple.rs.html
macro_rules! tuple_impls {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(
            impl<'a, $($T),+> ListWrapper<($($T,)+)> for ($(&'a [$T],)+)
            where
                $($T: ?Sized + Copy,)+
                // &'a Self: Sized,
            {
                fn wrapper_len(&self) -> usize {
                    tuple_impls!(@last_idx $($idx,)+) + 1
                }
                fn lens(&self) -> Vec<usize> {
                    let ls = vec![$(self.$idx.len()),+];
                    ls.iter().for_each(|l| debug_assert!(*l != 0));
                    ls
                }
                fn next_item(&self, indexes: &Vec<usize>) -> ($($T,)+) {
                    unsafe {
                        (
                            $(*self.$idx.get_unchecked(indexes[$idx]),)+
                        )
                    }
                }


                fn next_with_buffer(
                    &self,
                    indexes: &Vec<usize>,
                    buffer: &mut ($($T,)+),
                ) -> () {
                    // `nlists` verification is unnecessary because it's verified
                    // at compile-time

                    unsafe {
                        $(
                            buffer.$idx = *self.$idx.get_unchecked(indexes[$idx]);
                        )+
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
