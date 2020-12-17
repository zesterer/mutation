use core::ops::Deref;

pub trait IsRef<'a> {
    type Inner: 'a;
    type Mut: Mut;
    type Map<B> where B: 'a;

    fn map<B: GenMut<'a>, F, G>(
        self,
        f: F,
        g: G,
    ) -> B::Ty<'a, Self::Mut>
    where
        F: FnOnce(&'a Self::Inner) -> B::Ty<'a, Immutable>,
        G: FnOnce(&'a mut Self::Inner) -> B::Ty<'a, Mutable>;
}

impl<'a, T> IsRef<'a> for &'a T {
    type Inner = T;
    type Mut = Immutable;
    type Map<B> where B: 'a = &'a B;

    fn map<B: GenMut<'a>, F, G>(
        self,
        f: F,
        g: G,
    ) -> B::Ty<'a, Self::Mut>
    where
        F: FnOnce(&'a Self::Inner) -> B::Ty<'a, Immutable>,
        G: FnOnce(&'a mut Self::Inner) -> B::Ty<'a, Mutable>
    {
        f(self)
    }
}

impl<'a, T> IsRef<'a> for &'a mut T {
    type Inner = T;
    type Mut = Mutable;
    type Map<B> where B: 'a = &'a mut B;

    fn map<B: GenMut<'a>, F, G>(
        self,
        f: F,
        g: G,
    ) -> B::Ty<'a, Self::Mut>
    where
        F: FnOnce(&'a Self::Inner) -> B::Ty<'a, Immutable>,
        G: FnOnce(&'a mut Self::Inner) -> B::Ty<'a, Mutable>
    {
        g(self)
    }
}

pub trait GenMut<'g> {
    type Ty<'a, M: Mut> where 'g: 'a;
}

pub struct GenRef<'a, T>(&'a T);

impl<'g, T: 'g> GenMut<'g> for GenRef<'g, T> {
    type Ty<'b, M: Mut> where 'g: 'b = M::Ref<'b, T>;
}

impl<'g, T: GenMut<'g>> GenMut<'g> for Option<T> {
    type Ty<'b, M: Mut> where 'g: 'b = Option<T::Ty<'b, M>>;
}

pub struct SliceIter<'a, T>(&'a T);

impl<'g, T: 'g> GenMut<'g> for SliceIter<'g, T> {
    type Ty<'b, M: Mut> where 'g: 'b = M::SliceIter<'b, T>;
}

pub trait Mut: Sized {
    type Ref<'a, T>: Deref<Target = T> where T: 'a;
    type SliceIter<'a, T>: Iterator<Item = Self::Ref<'a, T>> where T: 'a;
}

pub struct Immutable;
impl Mut for Immutable {
    type Ref<'a, T> where T: 'a = &'a T;
    type SliceIter<'a, T> where T: 'a = core::slice::Iter<'a, T>;
}

pub struct Mutable;
impl Mut for Mutable {
    type Ref<'a, T> where T: 'a = &'a mut T;
    type SliceIter<'a, T> where T: 'a = core::slice::IterMut<'a, T>;
}

pub trait Ref<'a, T> = IsRef<'a, Inner = T> + Deref<Target = T>;

pub type RefMap<'a, R, T> = <GenRef<'a, T> as GenMut<'a>>::Ty<'a, <R as IsRef<'a>>::Mut>;

pub struct MyVec<T>(Vec<T>);

impl<T: 'static> MyVec<T> {
    pub fn get<'a, R: Ref<'a, Self>>(self: R, idx: usize) -> Option<RefMap<'a, R, T>> {
        self.map::<Option<GenRef<T>>, _, _>(
            |this: &Self| this.0.get(idx),
            |this: &mut Self| this.0.get_mut(idx),
        )
    }

    pub fn get_expect<'a, R: Ref<'a, Self>>(self: R, idx: usize) -> RefMap<'a, R, T> {
        self.get(idx).unwrap()
    }

    pub fn iter<'a, R: Ref<'a, Self>>(self: R) -> impl Iterator<Item = RefMap<'a, R, T>> {
        self.map::<SliceIter<T>, _, _>(
            |this: &Self| this.0.iter(),
            |this: &mut Self| this.0.iter_mut(),
        )
    }
}

impl MyVec<i32> {
    pub fn iter_positive<'a, R: Ref<'a, Self>>(self: R) -> impl Iterator<Item = RefMap<'a, R, i32>> {
        self.iter().filter(|x| **x >= 0)
    }
}

#[test]
fn basic() {
    let mut x = MyVec(vec![-1, 2, 3, 4]);

    assert_eq!((&x).iter().copied().sum::<i32>(), 8);

    assert_eq!((&x).iter_positive().copied().sum::<i32>(), 9);

    *(&mut x).get(1).unwrap() = 3;
    assert_eq!(*x.get_expect(1), 3);

    (&mut x).iter_positive().for_each(|x| *x = 1);

    assert_eq!(x.0, vec![-1, 1, 1, 1]);
}
