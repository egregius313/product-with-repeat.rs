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

/// Provides a `ProductWithRepeat` implementation for which the desired repeat
/// size is known.
///
/// For example:
///
/// ```rust
/// use product_with_repeat::known_size::ProductWithRepeat;
///
/// let potential_baseball_teams: [&Player; 26] = players.product_with_repeat::<26>().collect();
/// ```
pub mod known_size {
    use std::iter::FusedIterator;

    /// Create a cartesian product of a given length from a given iterable.
    pub trait ProductWithRepeat<T> {
        /// `REPEAT` - the length of the desired n-tuples.
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

/// Generate the Cartesian product of `N_ITS` collections.
///
/// The resulting iterator yields arrays containing items from the Cartesian
/// product.
pub fn product<'a, T, A: AsRef<[T]>, const N_ITS: usize>(
    items: &'a [A; N_ITS],
) -> Product<'a, T, A, N_ITS> {
    Product {
        items,
        state: [0; N_ITS],
        completed: false,
        _t: std::marker::PhantomData,
    }
}

/// Iterator representing the state of calculating a Cartesian product
///
/// Created by using the [`product`] function.
///
/// A `Product<'a, T, A: AsRef<[T]>, N_ITS>` will yield `[&'a T; N]` as an
/// iterator.
pub struct Product<'a, T, A: AsRef<[T]>, const N_ITS: usize> {
    /// The iterators being iterated over to generate the cartesian product
    items: &'a [A; N_ITS],
    /// Indices for each iterable
    state: [usize; N_ITS],
    /// Whether or not the iterator has completed
    completed: bool,
    /// Necessary for indicating that the `&'a T` references will live long
    /// enough.
    _t: std::marker::PhantomData<&'a T>,
}

impl<'a, T, A: AsRef<[T]>, const N_ITS: usize> Iterator for Product<'a, T, A, N_ITS> {
    type Item = [&'a T; N_ITS];

    fn next(&mut self) -> Option<Self::Item> {
        if self.completed {
            return None;
        }

        // For the state:  [i, j, k, ...]
        // Yield the item: [&items[0][i], &items[1][j], &items[2][k], ...]
        let item = std::array::from_fn(|i| &self.items[i].as_ref()[self.state[i]]);

        // Had to inline the function due to lifetime issues :|
        #[allow(unused_labels)]
        'increment_state: {
            let mut carry = true;
            for (i, r) in self.state.iter_mut().enumerate().rev() {
                // Increment current index
                *r += 1;
                if *r >= self.items[i].as_ref().len() {
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

        Some(item)
    }
}

impl<'a, T, A: AsRef<[T]>, const N_ITS: usize> FusedIterator for Product<'a, T, A, N_ITS> {}
