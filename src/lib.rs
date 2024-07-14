use std::iter::FusedIterator;

/// Create a cartesian product of a given length from a given iterable.
pub trait ProductWithRepeat<T> {
    fn product_with_repeat(&self, repeat: usize) -> Iter<T>;
}

impl<T, A: AsRef<[T]>> ProductWithRepeat<T> for A {
    fn product_with_repeat(&self, repeat: usize) -> Iter<T> {
        Iter {
            items: self.as_ref(),
            state: vec![0; repeat],
            completed: false,
        }
    }
}

/// Iterator for the Cartesian product
pub struct Iter<'a, T> {
    /// The items used for the product
    items: &'a [T],
    /// The list of indices for the items being iterated over.
    state: Vec<usize>,
    /// Whether or not all items have been iterated over.
    completed: bool,
}

impl<T> Iter<'_, T> {
    fn item_len(&self) -> usize {
        self.state.len()
    }

    #[inline]
    fn increment_state(&mut self) {
        let mut carry = true;
        for r in self.state.iter_mut().rev() {
            // Increment current index
            *r += 1;
            if *r >= self.items.len() {
                *r = 0;
            } else {
                carry = false;
                break;
            }
        }
        // If you would still need to carry, you have overflowed
        let overflowed = carry;
        self.completed = overflowed;
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.completed {
            return None;
        }

        let mut item = Vec::with_capacity(self.item_len());

        for &i in &self.state {
            item.push(&self.items[i]);
        }

        self.increment_state();

        Some(item)
    }
}

impl<T> FusedIterator for Iter<'_, T> {}

pub mod known_size {
    use std::iter::FusedIterator;

    /// Create a cartesian product of a given length from a given iterable.
    pub trait ProductWithRepeat<T> {
        fn product_with_repeat<const REPEAT: usize>(&self) -> Iter<T, REPEAT>;
    }

    impl<T, A: AsRef<[T]>> ProductWithRepeat<T> for A {
        fn product_with_repeat<const REPEAT: usize>(&self) -> Iter<T, REPEAT> {
            Iter {
                items: self.as_ref(),
                state: [0; REPEAT],
                completed: false,
            }
        }
    }

    /// Iterator for the Cartesian product
    pub struct Iter<'a, T, const REPEAT: usize> {
        /// The items used for the product
        items: &'a [T],
        /// The list of indices for the items being iterated over.
        state: [usize; REPEAT],
        /// Whether or not all items have been iterated over.
        completed: bool,
    }

    impl<T, const R: usize> Iter<'_, T, R> {
        #[inline]
        fn increment_state(&mut self) {
            let mut carry = true;
            for r in self.state.iter_mut().rev() {
                // Increment current index
                *r += 1;
                if *r >= self.items.len() {
                    *r = 0;
                } else {
                    carry = false;
                    break;
                }
            }
            // If you would still need to carry, you have overflowed
            let overflowed = carry;
            self.completed = overflowed;
        }
    }

    impl<'a, T, const REPEAT: usize> Iterator for Iter<'a, T, REPEAT> {
        type Item = [&'a T; REPEAT];

        fn next(&mut self) -> Option<Self::Item> {
            if self.completed {
                return None;
            }

            let item = std::array::from_fn(|i| &self.items[self.state[i]]);

            self.increment_state();

            Some(item)
        }
    }

    impl<T, const R: usize> FusedIterator for Iter<'_, T, R> {}
}
