# Mutation

Use the power of nightly Rust to write code that's generic over mutation!

# Features

- Zero-cost (at runtime)!

- Highly likely to trigger Internal Compiler Errors (ICE)!

- A horrific type astronomy hack!

- Would work even better if Rust had proper higher-kinded types instead of just GATs!

- Kind of works with iterators

**Highly Experimental**

Are you frequently frustrated by the need to write near-identical functions that
differ only by the occurrence of the `mut` keyword?

Do you wish you that you didn't have to maintain two independent versions of
what effectively amount to the same function?

Well, your dreams probably aren't going to come true. This might be the next
best thing though, assuming you're a normal person that thinks that macros are
silly.

## Example

```rust
use mutation::prelude::*;

struct MyVec<T>(Vec<T>);

impl<T> MyVec<T> {
    /// This function 'glues' our generic-over-mutation API to the underlying
    /// non-generic API of `Vec`.
    pub fn get<'a, R: Ref<'a, Self>>(self: R, idx: usize) -> Option<RefMap<'a, R, T>> {
        /// The underlying `Vec` is not generic over mutation, so here we
        /// specialize our function with the corresponding function of `Vec`.
        self.map::<Option<GenRef<T>>, _, _>(
            |this: &Self| this.0.get(idx),
            |this: &mut Self| this.0.get_mut(idx),
        )
    }

    /// This function 'glues' our generic-over-mutation API to the underlying
    /// non-generic API of `Vec`.
    pub fn iter<'a, R: Ref<'a, Self>>(self: R) -> impl Iterator<Item = RefMap<'a, R, T>> where T: 'a {
        /// The underlying `Vec` is not generic over mutation, so here we
        /// specialize our function with the corresponding function of `Vec`.
        self.map::<SliceIter<T>, _, _>(
            |this: &Self| this.0.iter(),
            |this: &mut Self| this.0.iter_mut(),
        )
    }

    /// And now for some functions that are truly generic over mutation using
    /// the API above...

    /// This function works for both mutable and immutable references!
    pub fn get_expect<'a, R: Ref<'a, Self>>(self: R, idx: usize) -> RefMap<'a, R, T> {
        self.get(idx).unwrap()
    }

    /// This function works for both mutable and immutable references!
    pub fn iter_positive<'a, R: Ref<'a, Self>>(self: R) -> impl Iterator<Item = RefMap<'a, R, T>>
        where T: PartialOrd<i32> + 'a
    {
        self.iter().filter(|x| **x >= 0)
    }
}

/// Example usage

let mut my_vec = MyVec(vec![1, 2, 3]);

assert_eq!((&my_vec).iter().copied().sum::<i32>(), 8);

// Iterate elements immutably
assert_eq!((&my_vec).iter_positive().copied().sum::<i32>(), 9);

// Mutate the second element
*(&mut my_vec).get_expect(1) = 4;

// Immutably access the second element
assert_eq!(my_vec.get_expect(1), Some(&4));

// Iterate elements mutably, giving each a value of 1
(&mut my_vec).iter_positive().for_each(|x| *x = 1);

assert_eq!(my_vec.0, vec![-1, 1, 1, 1]);
```
