use std::ops;
use {Rank, Select0, Select1, UnsignedInt};

pub trait Dict<T>: ops::Index<T>
where
    <Self as ops::Index<T>>::Output: PartialEq<Self::Item>,
{
    /// Associated items to this dictionary.
    type Item;

    /// Result type of `select`.
    type Index;

    /// Result type of `rank`.
    type Count;

    /// Returns count of `Item` in `0...i`.
    fn rank(&self, item: &Self::Item, i: Self::Index) -> Self::Count;

    /// Returns the position of the `c+1`-th appearance of `Item`.
    fn select(&self, item: &Self::Item, c: Self::Count) -> Option<Self::Index>;
}

pub trait BitDict<T: UnsignedInt>
    : ops::Index<T, Output = bool> + Rank<T> + Select0<T, Index = T> + Select1<T, Index = T>
    {
    fn rank0(&self, T) -> Self::Count;
    fn rank1(&self, T) -> Self::Count;
    fn select0(&self, c: T) -> Option<<Self as Select0<T>>::Index>;
    fn select1(&self, c: T) -> Option<<Self as Select1<T>>::Index>;
}

impl<T, U> BitDict<T> for U
where
    T: UnsignedInt,
    U: ops::Index<T, Output = bool> + Rank<T> + Select0<T, Index = T> + Select1<T, Index = T>,
{
    fn rank0(&self, i: T) -> Self::Count {
        <Self as Rank<T>>::rank0(self, i)
    }
    fn rank1(&self, i: T) -> Self::Count {
        <Self as Rank<T>>::rank1(self, i)
    }

    fn select0(&self, c: T) -> Option<<Self as Select0<T>>::Index> {
        <Self as Select0<T>>::select0(self, c)
    }
    fn select1(&self, c: T) -> Option<<Self as Select1<T>>::Index> {
        <Self as Select1<T>>::select1(self, c)
    }
}

// impl<T, U> Dict<T> for U
// where
//     T: UnsignedInt,
//     U: BitDict<T>,
// {
//     type Item = bool;

//     type Index = U::Index;
//     type Count = U::Count;

//     // fn size(&self) -> Self::Rank {<Self as Rank<T>>::SIZE}

//     fn rank(&self, item: &Self::Item, i: Self::Index) -> Self::Count {
//         if *item {
//             BitDict::rank1(self, i)
//         } else {
//             BitDict::rank0(self, i)
//         }
//     }

//     fn select(&self, item: &Self::Item, c: Self::Count) -> Option<Self::Index> {
//         if *item {
//             BitDict::select1(self, c)
//         } else {
//             BitDict::select0(self, c)
//         }
//     }
// }
