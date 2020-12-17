#![feature(generic_associated_types, arbitrary_self_types, trait_alias)]

use core::{
    marker::PhantomData,
    ops::Deref,
};

pub trait IsRef<'a> {
    type Inner;
    type Mut: Mut;
    type Ref<'b, U> where U: 'b;

    fn specialize<'b, U: Specialization, F, G>(self, f: F, g: G) -> U::Ty<Self::Mut>
    where
        'a: 'b,
        Self::Inner: 'b,
        F: FnOnce(<Immutable as Mut>::Ref<'b, Self::Inner>) -> U::Ty<Immutable>,
        G: FnOnce(<Mutable as Mut>::Ref<'b, Self::Inner>) -> U::Ty<Mutable>;
}

impl<'a, T> IsRef<'a> for &'a T {
    type Inner = T;
    type Mut = Immutable;
    type Ref<'b, U> where U: 'b = &'b U;

    fn specialize<'b, U: Specialization, F, G>(self, f: F, g: G) -> U::Ty<Self::Mut>
    where
        'a: 'b,
        Self::Inner: 'b,
        F: FnOnce(<Immutable as Mut>::Ref<'b, Self::Inner>) -> U::Ty<Immutable>,
        G: FnOnce(<Mutable as Mut>::Ref<'b, Self::Inner>) -> U::Ty<Mutable>,
    {
        f(self)
    }
}

impl<'a, T> IsRef<'a> for &'a mut T {
    type Inner = T;
    type Mut = Mutable;
    type Ref<'b, U> where U: 'b = &'b mut U;

    fn specialize<'b, U: Specialization, F, G>(self, f: F, g: G) -> U::Ty<Self::Mut>
    where
        'a: 'b,
        Self::Inner: 'b,
        F: FnOnce(<Immutable as Mut>::Ref<'b, Self::Inner>) -> U::Ty<Immutable>,
        G: FnOnce(<Mutable as Mut>::Ref<'b, Self::Inner>) -> U::Ty<Mutable>,
    {
        g(self)
    }
}





pub trait Mut: Sized {
    type Ref<'a, T>: Sized + Deref<Target = T> where T: 'a;
    type Choose<'a, A, B> where A: 'a, B: 'a;
}

pub struct Immutable;

impl Mut for Immutable {
    type Ref<'a, T> where T: 'a = &'a T;
    type Choose<'a, A, B> where A: 'a, B: 'a = A;
}

pub struct Mutable;

impl Mut for Mutable {
    type Ref<'a, T> where T: 'a = &'a mut T;
    type Choose<'a, A, B> where A: 'a, B: 'a = B;
}

pub trait Specialization {
    type Ty<M: Mut>;
}

pub type RefMap<'a, R, T> = <<R as IsRef<'a>>::Mut as Mut>::Ref<'a, T>;

pub trait Ref<'a, T> = IsRef<'a, Inner = T> + Deref<Target = T>;

pub mod prelude {
    use super::{Ref, RefMap, Mut, Specialization};
}

#[test]
fn basic() {
    struct MyVec<T>(Vec<T>);

    impl<T> MyVec<T> {
        /*
        pub fn iter<'a, R: Ref<'a, Self>>(self: R) -> impl Iterator<Item = RefMap<'a, R, T>> {
            struct Iter<'a, T>(&'a T);
            impl<'a, T> Specialization for Iter<'a, T> {
                type Ty<M: Mut> = M::Choose<
                    'a,
                    std::slice::Iter<'a, T>,
                    std::slice::IterMut<'a, T>,
                >;
            }

            R::specialize::<Iter<_>, _, _>(
                self,
                |this| this.0.iter(),
                |this| this.0.iter_mut(),
            )
        }
        */

        pub fn get<'a, R: Ref<'a, Self>>(self: R, idx: usize) -> Option<RefMap<'a, R, T>> {
            struct OptionRef<'a, T>(&'a T);

            impl<'a, T> Specialization for OptionRef<'a, T> {
                type Ty<M: Mut> = Option<M::Ref<'a, T>>;
            }

            R::specialize::<OptionRef<_>, _, _>(
                self,
                |this| this.0.get(idx),
                |this| this.0.get_mut(idx),
            )
        }

        pub fn get_expect<'a, R: Ref<'a, Self>>(self: R, idx: usize) -> RefMap<'a, R, T> {
            self.get(idx).unwrap()
        }
    }

    let mut x = MyVec(vec![1, 2, 3, 4, 5]);

    assert_eq!(x.get(2), Some(&3));
    *(&mut x).get(2).unwrap() = 4;
    assert_eq!(x.get(2), Some(&4));
}
