//! A boilerplate library that provides traits [TryIndex] and [TryIndexMut].
//! Unlike the ones in [try_traits](https://docs.rs/try-traits/latest/try_traits/),
//! this gives explicit implementations for all standard library collections,
//! which can actually fail. In exchange, no blanket implementation is provided.
//!
//! The idea is to provide a single definition of these traits
//! that other crates may use to interface with each other in a generic way,
//! while carrying no further code with it.
//!
//! This crate has a default `std` feature that may be disabled for `no_std` support,
//! as well as an `alloc` feature that enables `no_std` `alloc` container types.
//!
//! ## Examples
//! ### Basic usage
//! The methods in this crate return `None` on failure rather than panicking.
//!
//! ```rust
//! use try_index::*;
//!
//! let foo = vec![4, 3, 6, 2];
//! assert_eq!(foo.try_index(2), Some(&6));
//! assert_eq!(foo.try_index(4), None);
//! assert_eq!(foo.try_index(1..=2), Some(&[3, 6][..]));
//! ```
//!
//! ### Implementing on custom type
//! With this, you can provide fallible indexing for your own types.
//!
//! ```rust
//! use try_index::*;
//!
//! struct YourType {
//! }
//!
//! impl TryIndex<usize> for YourType {
//!     type Output = u8;
//!     
//!     fn try_index(&self, index: usize) -> Option<&Self::Output> {
//!         todo!() // your implementation
//!     }
//! }
//!
//! let foo = YourType {};
//! // now, simply call foo.try_index()
//! ```
//!
//! ### Implementing on third-party type
//! This allows you to glue together libraries not providing [TryIndex]
//! and others requiring it, within your code.
//!
//! ```rust
//! use try_index::*;
//!
//! // defined elsewhere
//! pub struct NotYourType {
//! }
//!
//! // can't implement directly, so we use a wrapper
//! #[repr(transparent)]
//! struct YourWrapper<'a> {
//!     inner: &'a NotYourType,
//! }
//!
//! impl<'a> TryIndex<usize> for YourWrapper<'a> {
//!     type Output = u8;
//!     
//!     fn try_index(&self, index: usize) -> Option<&Self::Output> {
//!         todo!() // your implementation
//!     }
//! }
//!
//! let foo = NotYourType {};
//! let wrapper = YourWrapper {inner: &foo};
//! // now, call wrapper.try_index() or pass wrapper around
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(not(feature = "std"))]
use core::{ffi::CStr, ops::RangeFrom, slice, slice::SliceIndex};
#[cfg(feature = "std")]
use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashMap, VecDeque},
    ffi::{CStr, CString, OsStr, OsString},
    hash::Hash,
    ops::{RangeFrom, RangeFull},
    slice,
    slice::SliceIndex,
};
#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::{
    borrow::Borrow,
    collections::{btree_map::BTreeMap, vec_deque::VecDeque},
    ffi::CString,
    string::String,
    vec::Vec,
};
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use core::ops::RangeFull;

/*
 * Traits
 */
/// A fallible version of [Index](https://doc.rust-lang.org/std/ops/trait.Index.html)
/// that will return `None` if the indexing operation fails.
pub trait TryIndex<Idx>
where
    Idx: ?Sized,
{
    type Output: ?Sized;

    fn try_index(&self, index: Idx) -> Option<&Self::Output>;
}

/// A fallible version of [IndexMut](https://doc.rust-lang.org/std/ops/trait.IndexMut.html)
/// that will return `None` if the indexing operation fails.
pub trait TryIndexMut<Idx>
where
    Idx: ?Sized,
{
    type Output: ?Sized;

    fn try_index_mut(&mut self, index: Idx) -> Option<&mut Self::Output>;
}

/*
 * Slices
 */
impl<T, I> TryIndex<I> for [T]
where
    I: SliceIndex<[T]>,
{
    type Output = <I as SliceIndex<[T]>>::Output;

    fn try_index(&self, index: I) -> Option<&Self::Output> {
        self.get(index)
    }
}

impl<T, I> TryIndexMut<I> for [T]
where
    I: SliceIndex<[T]>,
{
    type Output = <I as SliceIndex<[T]>>::Output;

    fn try_index_mut(&mut self, index: I) -> Option<&mut Self::Output> {
        self.get_mut(index)
    }
}

/*
 * Arrays
 */
impl<T, I, const N: usize> TryIndex<I> for [T; N]
where
    [T]: TryIndex<I, Output = T>,
{
    type Output = T;

    fn try_index(&self, index: I) -> Option<&Self::Output> {
        self.as_slice().try_index(index)
    }
}

impl<T, I, const N: usize> TryIndexMut<I> for [T; N]
where
    [T]: TryIndexMut<I, Output = T>,
{
    type Output = T;

    fn try_index_mut(&mut self, index: I) -> Option<&mut Self::Output> {
        self.as_mut_slice().try_index_mut(index)
    }
}

/*
 * Vec
 */
#[cfg(any(feature = "std", feature = "alloc"))]
impl<T, I> TryIndex<I> for Vec<T>
where
    I: SliceIndex<[T]>,
{
    type Output = <I as SliceIndex<[T]>>::Output;

    fn try_index(&self, index: I) -> Option<&Self::Output> {
        self.get(index)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T, I> TryIndexMut<I> for Vec<T>
where
    I: SliceIndex<[T]>,
{
    type Output = <I as SliceIndex<[T]>>::Output;

    fn try_index_mut(&mut self, index: I) -> Option<&mut Self::Output> {
        self.get_mut(index)
    }
}

/*
 * VecDeque
 */
#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> TryIndex<usize> for VecDeque<T> {
    type Output = T;

    fn try_index(&self, index: usize) -> Option<&Self::Output> {
        self.get(index)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T> TryIndexMut<usize> for VecDeque<T> {
    type Output = T;

    fn try_index_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        self.get_mut(index)
    }
}

/*
 * HashMap
 */
#[cfg(feature = "std")]
impl<K, Q, V> TryIndex<&Q> for HashMap<K, V>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash + ?Sized,
{
    type Output = V;

    fn try_index(&self, index: &Q) -> Option<&Self::Output> {
        self.get(index)
    }
}

#[cfg(feature = "std")]
impl<K, Q, V> TryIndexMut<&Q> for HashMap<K, V>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash + ?Sized,
{
    type Output = V;

    fn try_index_mut(&mut self, index: &Q) -> Option<&mut Self::Output> {
        self.get_mut(index)
    }
}

/*
 * BTreeMap
 */
#[cfg(any(feature = "std", feature = "alloc"))]
impl<K, Q, V> TryIndex<&Q> for BTreeMap<K, V>
where
    K: Borrow<Q> + Ord,
    Q: Ord + ?Sized,
{
    type Output = V;

    fn try_index(&self, index: &Q) -> Option<&Self::Output> {
        self.get(index)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<K, Q, V> TryIndexMut<&Q> for BTreeMap<K, V>
where
    K: Borrow<Q> + Ord,
    Q: Ord + ?Sized,
{
    type Output = V;

    fn try_index_mut(&mut self, index: &Q) -> Option<&mut Self::Output> {
        self.get_mut(index)
    }
}

/*
 * str
 */
#[cfg(any(feature = "std", feature = "alloc"))]
impl<I> TryIndex<I> for str
where
    I: SliceIndex<str>,
{
    type Output = <I as SliceIndex<str>>::Output;

    fn try_index(&self, index: I) -> Option<&Self::Output> {
        self.get(index)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<I> TryIndexMut<I> for str
where
    I: SliceIndex<str>,
{
    type Output = <I as SliceIndex<str>>::Output;

    fn try_index_mut(&mut self, index: I) -> Option<&mut Self::Output> {
        self.get_mut(index)
    }
}

/*
 * String
 */
#[cfg(any(feature = "std", feature = "alloc"))]
impl<I> TryIndex<I> for String
where
    I: SliceIndex<str>,
{
    type Output = <I as SliceIndex<str>>::Output;

    fn try_index(&self, index: I) -> Option<&Self::Output> {
        self.get(index)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<I> TryIndexMut<I> for String
where
    I: SliceIndex<str>,
{
    type Output = <I as SliceIndex<str>>::Output;

    fn try_index_mut(&mut self, index: I) -> Option<&mut Self::Output> {
        self.get_mut(index)
    }
}

/*
 * CStr
 */
impl TryIndex<RangeFrom<usize>> for CStr {
    type Output = CStr;

    fn try_index(&self, index: RangeFrom<usize>) -> Option<&Self::Output> {
        let l = self.count_bytes();
        match index.start <= l {
            true => Some(unsafe {
                CStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(
                    &*(self.as_ptr() as *const u8).add(index.start),
                    l - index.start + 1,
                ))
            }),
            false => None,
        }
    }
}

#[test]
fn cstr_test() {
    let string = c"Test";
    assert_eq!(string.try_index(2..), Some(c"st"));
    assert_eq!(string.try_index(4..), Some(c""));
    assert_eq!(string.try_index(5..), None);
}

/*
 * CString
 */
#[cfg(any(feature = "std", feature = "alloc"))]
impl TryIndex<RangeFull> for CString {
    type Output = CStr;

    fn try_index(&self, index: RangeFull) -> Option<&Self::Output> {
        Some(&self[index])
    }
}

/*
 * OsString
 */
#[cfg(feature = "std")]
impl TryIndex<RangeFull> for OsString {
    type Output = OsStr;

    fn try_index(&self, index: RangeFull) -> Option<&Self::Output> {
        Some(&self[index])
    }
}
