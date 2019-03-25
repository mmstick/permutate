use std::marker::PhantomData;

mod specializations;

pub use specializations::Repeated;

pub trait PermutatorWrapper<ListWrap, ItemWrap>
where
    ListWrap: ListWrapper<ItemWrap>,
    ListWrap: ListWrapper<ItemWrap> + ?Sized + Clone,
{
    fn new(lists: &ListWrap) -> Permutator<ListWrap, ItemWrap>;
    fn set_index(&mut self, iter_no: usize, indexes: Vec<usize>);
    fn get_index(&self) -> (usize, Vec<usize>);
    fn max_permutations(&self) -> usize;
    fn reset(&mut self);
    fn next_with_buffer<'b>(&mut self, buffer: &'b mut ItemWrap) -> Option<&'b mut ItemWrap>;
}

#[derive(Clone, Debug)]
pub struct Permutator<ListWrap, ItemWrap>
where
    ListWrap: ListWrapper<ItemWrap>,
{
    indexes: IndexCounters,
    lists: ListWrap,
    _list_item_wrapper: PhantomData<ItemWrap>,
}

/// Abstract the outermost wrapper behaviour
pub trait ListWrapper<ItemWrap>
where
    ItemWrap: ?Sized,
{
    fn wrapper_len(&self) -> usize;
    fn lens(&self) -> Vec<usize>;
    fn next_item(&self, indexes: &Vec<usize>) -> ItemWrap;
    fn next_with_buffer(&self, indexes: &Vec<usize>, buffer: &mut ItemWrap) -> ();
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
        if self.indexes.max_iters != 0 && self.indexes.curr_iter == self.indexes.max_iters {
            return None;
        }

        self.indexes.curr_iter += 1;
        let output = ListWrap::next_item(&self.lists, &self.indexes.indexes);

        // Increment the indexes to point towards the next set of values.
        self.indexes.increment(self.lists.wrapper_len() - 1);

        // Return the collected permutation
        Some(output)
    }
}

#[derive(Clone, Debug)]
/// Tracks the state of the indexes of each list.
pub struct IndexCounters {
    /// The current state of the indexes
    indexes: Vec<usize>,
    /// The lengths of the wrapped lists
    lens: Vec<usize>,
    /// The current iteration position
    curr_iter: usize,
    /// The maximum number of iterations to perform
    max_iters: usize,
}

impl IndexCounters {
    /// Increments & resets index indexes according to their maximum values.
    fn increment(&mut self, mut nlists: usize) {
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

    fn reset(&mut self) {
        for value in self.indexes.iter_mut() {
            *value = 0;
        }
    }
}
