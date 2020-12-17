# Mutation

Use the power of nightly Rust to write code that's generic over mutation!

# Features

- Zero-cost (at runtime)!

- Highly likely to trigger Internal Compiler Errors (ICE)!

- A horrific type astronomy hack!

- Would work even better if Rust had proper higher-kinded types instead of just GATs!

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
        /// This function returns an `Option`! We need to tell Mutation how to
        /// specialise an `Option` over mutation.
        struct OptionRef<'a, T>(&'a T);
        impl<'a, T> Specialization for OptionRef<'a, T> {
            type Ty<M: Mut> = Option<M::Ref<'a, T>>;
        }

        /// The underlying `Vec` is not generic over mutation, so here we
        /// specialize our function with the corresponding function of `Vec`.
        R::specialize::<OptionRef<_>, _, _>(
            self,
            |this| this.0.get(idx),
            |this| this.0.get_mut(idx),
        )
    }

    /// And now for some functions that are truly generic over mutation using
    /// the API above...

    /// This function works for both mutable and immutable references!
    pub fn get_expect<'a, R: Ref<'a, Self>>(self: R, idx: usize) -> RefMap<'a, R, T> {
        self.get(idx).unwrap()
    }
}

/// Example usage

let mut my_vec = MyVec(vec![1, 2, 3]);

// Mutate the second element
*(&mut my_vec).get_expect(1) = 4;

// Immutably access the second element
assert_eq!(my_vec.get_expect(1), Some(&4));
```
