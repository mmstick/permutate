use ListWrapper;

type OneSized<'a, T> = [&'a [T]; 1];

/// Indicates that the permutator will repeated the input and read it as a
/// square-sized matrix.
///
/// # Example
///
/// ```rust
/// # use permutate::{Repeated, Permutator};
/// #
/// // the permutator `pv`, which indicates the `Repeated` type,
/// let v = [&["1", "2", "3"][..]];
/// let pv = Permutator::<Repeated<_>, _>::new(&v);
///
/// // will have the same output as the permutator `pm`,
/// // which does not indicates the `Repeated` type:
/// let m = [&["1", "2", "3"][..], &["1", "2", "3"][..], &["1", "2", "3"][..]];
/// let pm = Permutator::<_, _>::new(&m.to_vec());
///
/// // and such output is:
/// let output = [
///   &["1", "1", "1"][..], &["1", "1", "2"][..], &["1", "1", "3"][..],
///   &["1", "2", "1"][..], &["1", "2", "2"][..], &["1", "2", "3"][..],
///   &["1", "3", "1"][..], &["1", "3", "2"][..], &["1", "3", "3"][..],
///   &["2", "1", "1"][..], &["2", "1", "2"][..], &["2", "1", "3"][..],
///   &["2", "2", "1"][..], &["2", "2", "2"][..], &["2", "2", "3"][..],
///   &["2", "3", "1"][..], &["2", "3", "2"][..], &["2", "3", "3"][..],
///   &["3", "1", "1"][..], &["3", "1", "2"][..], &["3", "1", "3"][..],
///   &["3", "2", "1"][..], &["3", "2", "2"][..], &["3", "2", "3"][..],
///   &["3", "3", "1"][..], &["3", "3", "2"][..], &["3", "3", "3"][..],
/// ];
/// #
/// # pv.zip(pm).zip(output[..].iter()).for_each(|((v, m), o)| {
/// #  assert_eq!(v, m);
/// #  assert_eq!(&v, o);
/// # });
/// ```
///
/// # Math perspective
///
/// This is the same as `(diag(V) x Ones) transposed` math operation,  
/// where   
/// - `V` is a vector `1xn`,  
/// - `diag(V)` is the `DiagonalMatrix` operation
/// which maps each element from `Vj` into a 2d-matrix `nxn` at position `jxj`,  
/// - `Ones` is a all-ones matrix `nxn`,
/// - `transposed` operation transposes the matrix that precedes it.
pub type Repeated<'a, T> = OneSized<'a, T>;

// implementation for when it's a single list
impl<'a, T> ListWrapper<Vec<T>> for OneSized<'a, T>
where
    T: ?Sized + Copy,
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
    fn next_item(&self, indexes: &Vec<usize>) -> Vec<T> {
        indexes
            .iter()
            .map(|value| unsafe { *self[0].get_unchecked(*value) })
            .collect::<Vec<T>>()
    }
    fn next_with_buffer(&self, indexes: &Vec<usize>, buffer: &mut Vec<T>, nlists: usize) -> () {
        debug_assert!(
            buffer.len() >= nlists,
            "buffer ({}) is not large enough to contain the permutation ({})",
            buffer.len(),
            nlists
        );

        let mut index = 0;
        unsafe {
            for outer_value in indexes.iter().map(|value| *self[0].get_unchecked(*value)) {
                *buffer.get_unchecked_mut(index) = outer_value;
                index += 1;
            }
        };
    }
}
