use std::marker::PhantomData;

mod specializations;

/// may be used as type annotation to indicate specialization
pub use specializations::OneSized;

/// Abstract the outermost wrapper behaviour
pub trait ListWrapper<'a, Item>
where
    Item: 'a + ?Sized,
{
    fn wrapper_len(&'a self) -> usize;
    fn lens(&'a self) -> Vec<usize>;
    fn next_item(&'a self, indexes: &Vec<usize>) -> Item;
}

#[derive(Clone, Debug)]
pub struct Permutator<'a, ListWrap, Item>
where
    ListWrap: ListWrapper<'a, Item> + ?Sized,
    Item: 'a + ?Sized,
{
    indexes: IndexCounters,
    lists: &'a ListWrap,
    nlists: usize,
    _list_item: PhantomData<Item>,
}

impl<'a, ListWrap, Item> Permutator<'a, ListWrap, Item>
where
    ListWrap: ListWrapper<'a, Item> + 'a + ?Sized,
    Item: 'a + ?Sized,
{
    pub fn new(lists: &'a ListWrap) -> Permutator<'a, ListWrap, Item> {
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
            lists: lists,
            nlists: nlists,
            _list_item: PhantomData,
        }
    }

    pub fn set_index(&mut self, iter_no: usize, indexes: Vec<usize>) {
        debug_assert!(
            indexes.len() == self.indexes.lens.len(),
            "indexes have an invalid length"
        );
        self.indexes.indexes = indexes;
        self.indexes.curr_iter = iter_no;
    }

    /// Obtains the current iteration number and the index counter's indexes.
    pub fn get_index(&self) -> (usize, Vec<usize>) {
        (self.indexes.curr_iter, self.indexes.indexes.clone())
    }

    /// Returns the total number of permutations possible
    pub fn max_permutations(&self) -> usize {
        self.indexes.max_iters
    }

    /// Resets the internal state of the `Permutator` to allow you to start permutating again.
    pub fn reset(&mut self) {
        self.indexes.reset();
        self.indexes.curr_iter = 0;
    }
}

impl<'a, ListWrap, NextItem> Iterator for Permutator<'a, ListWrap, NextItem>
where
    ListWrap: ListWrapper<'a, NextItem> + 'a + ?Sized,
    NextItem: 'a + Sized,
{
    type Item = NextItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.indexes.max_iters != 0 {
            if self.indexes.curr_iter == self.indexes.max_iters {
                return None;
            }
        }

        self.indexes.curr_iter += 1;
        let output = ListWrapper::next_item(self.lists, &self.indexes.indexes);

        // Increment the indexes to point towards the next set of values.
        self.indexes.increment(self.nlists - 1);

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
