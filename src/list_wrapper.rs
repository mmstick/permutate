mod single_list;
mod tuple_of_lists;
mod vec_of_lists;

// re-export
pub use self::single_list::Repeated;

/// Abstract the outermost slice(s) wrapper behaviour.  
/// eg. Accesses on `Vec` slice wrappers are different from `tuple` slice wrappers.
pub trait ListWrapper<ItemWrap>
where
    ItemWrap: Sized,
{
    /// The total number of slices that are being permutated with.
    fn wrapper_len(&self) -> usize;
    /// The length of each slices that are being permutated with.
    fn lens(&self) -> Vec<usize>;
    /// Get values according to the given `indexes`.
    fn next_item(&self, indexes: &Vec<usize>) -> ItemWrap;
    /// Same as `next_item`, but doesn't allocates for the returning `ItemWrap`.
    fn next_with_buffer(&self, indexes: &Vec<usize>, buffer: &mut ItemWrap) -> ();
}
