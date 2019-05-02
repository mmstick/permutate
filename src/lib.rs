#![feature(external_doc)]
#![doc(include = "Readme.md")]
#![deny(missing_docs)]

/// Bin utilities.
pub mod bin;

use std::marker::PhantomData;

mod index_counters;
mod list_wrapper;

use index_counters::IndexCounters;
pub use list_wrapper::{ListWrapper, Repeated};

/// The `PermutatorWrapper` contains the methods (creation, etc) which any Permutator should
/// implement.
pub trait PermutatorWrapper<ListWrap, ItemWrap>
where
    ListWrap: ListWrapper<ItemWrap>,
    ListWrap: ListWrapper<ItemWrap> + ?Sized + Clone,
{
    /// Initialize a new `Permutator` with the vec/tuple of input slices to permutate with.
    /// The input may be provided as either multiple lists via a vec/tuple of slices, or a single
    /// list as an slice within an array.
    fn new(lists: &ListWrap) -> Permutator<ListWrap, ItemWrap>;

    /// Sets the internal index counter's values to a specific state, which you will
    /// typically obtain when using the `get_index()` method. The `iter_no` parameter
    /// will specify what the iteration's position should be. If, for example, you set
    /// this value to `0`, then it will iterate through all possible permutations,
    /// including looping around back to the beginning and generating permutations
    /// for all possible values before the supplied state.
    ///
    /// # Panics
    /// This method will panic if the supplied indexes vector is not the correct length
    fn set_index(&mut self, iter_no: usize, indexes: Vec<usize>);

    /// Obtains the current iteration number and the index counter's indexes.
    fn get_index(&self) -> (usize, Vec<usize>);

    /// Returns the total number of permutations possible
    fn max_permutations(&self) -> usize;

    /// Resets the internal state of the `Permutator` to allow you to start permutating again.
    fn reset(&mut self);

    /// Provides similar functionality as the `Iterator` traits `next` method, but allows the
    /// ability to either supply your own buffer or re-use the `Vec`/`tuple` created by a prior
    /// `next` in order to avoid extra allocations.
    ///
    /// If there were more methods to compute, then the buffer mutable borrow is returned.  
    /// Otherwise, None is returned.
    ///
    /// # Panics
    /// This method will panic if the supplied buffer's length is invalid.
    fn next_with_buffer<'b>(&mut self, buffer: &'b mut ItemWrap) -> Option<&'b mut ItemWrap>;
}

/// The `Permutator` contains the state of the iterator as well as the owned values and/or
/// references of inputs that are being permutated. The input should be provided as a vector or
/// tuple of slices of values (or references).
#[derive(Clone, Debug)]
pub struct Permutator<ListWrap, ItemWrap>
where
    ListWrap: ListWrapper<ItemWrap>,
{
    /// The indexes is used to point to the next permutation sequence.
    indexes: IndexCounters,
    /// The internal data that the permutator is permutating against.
    lists: ListWrap,
    /// Phantom type, so that when implementing the `Iterator` for this structure,
    /// `ItemWrap` type may be used as `Iterator::Item`.
    _list_item_wrapper: PhantomData<ItemWrap>,
}

impl<ListWrap, ItemWrap> PermutatorWrapper<ListWrap, ItemWrap> for Permutator<ListWrap, ItemWrap>
where
    ListWrap: ListWrapper<ItemWrap> + ?Sized + Clone,
{
    fn new(lists: &ListWrap) -> Permutator<ListWrap, ItemWrap> {
        let nlists = lists.wrapper_len();
        let nvalues = lists.lens();
        let max_iters = nvalues.iter().product();

        Permutator {
            indexes: IndexCounters {
                indexes: vec![0; nlists],
                lens: nvalues,
                curr_iter: 0,
                max_iters: max_iters,
            },
            lists: lists.clone(),
            _list_item_wrapper: PhantomData,
        }
    }

    fn set_index(&mut self, iter_no: usize, indexes: Vec<usize>) {
        debug_assert!(
            indexes.len() == self.indexes.lens.len(),
            "indexes have an invalid length"
        );
        self.indexes.indexes = indexes;
        self.indexes.curr_iter = iter_no;
    }

    /// Obtains the current iteration number and the index counter's indexes.
    fn get_index(&self) -> (usize, Vec<usize>) {
        (self.indexes.curr_iter, self.indexes.indexes.clone())
    }

    /// Returns the total number of permutations possible
    fn max_permutations(&self) -> usize {
        self.indexes.max_iters
    }

    /// Resets the internal state of the `Permutator` to allow you to start permutating again.
    fn reset(&mut self) {
        self.indexes.reset();
        self.indexes.curr_iter = 0;
    }

    fn next_with_buffer<'b>(&mut self, buffer: &'b mut ItemWrap) -> Option<&'b mut ItemWrap> {
        if self.indexes.max_iters != 0 && self.indexes.curr_iter == self.indexes.max_iters {
            return None;
        }

        self.indexes.curr_iter += 1;
        let self_lists: &mut _ = &mut self.lists;
        ListWrap::next_with_buffer(self_lists, &self.indexes.indexes, buffer);
        self.indexes.increment(self_lists.wrapper_len() - 1);
        Some(buffer)
    }
}

impl<ListWrap, ItemWrap> Iterator for Permutator<ListWrap, ItemWrap>
where
    ListWrap: ListWrapper<ItemWrap>,
{
    type Item = ItemWrap;

    fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
        loop {
            if self.indexes.max_iters != 0 && self.indexes.curr_iter == self.indexes.max_iters {
                return None;
            }

            self.indexes.curr_iter += 1;

            if let _should_skip @ true = n != 0 {
                self.indexes.increment(&self.lists.wrapper_len() - 1);
                n -= 1;
            } else {
                let output = ListWrap::next_item(&self.lists, &self.indexes.indexes);
                self.indexes.increment(&self.lists.wrapper_len() - 1);
                return Some(output);
            }
        }
    }

    fn next(&mut self) -> Option<Self::Item> {
        // Without this check, the permutator would cycle forever and never return `None`
        // because my incrementing algorithim prohibits it.
        if self.indexes.max_iters != 0 && self.indexes.curr_iter == self.indexes.max_iters {
            return None;
        }

        self.indexes.curr_iter += 1;
        // Generates the next permutation sequence using the current indexes.
        let output = ListWrap::next_item(&self.lists, &self.indexes.indexes);

        // Increment the indexes to point towards the next set of values.
        self.indexes.increment(self.lists.wrapper_len() - 1);

        // Return the collected permutation
        Some(output)
    }
}
