#![no_std]
//! # Integers types which cannot be their minimum/maximum value.
//!
//! The standard library contains a collection of `std::num::NonZeroX` types: integer types which
//! cannot be zero. This crate extends this idea further by providing `NonMinX`/`NonMaxX`: integer
//! types which cannot be their minimum/maximum value.
//!
//! ```
//! # use nonminmax::*;
//! // Create a regular NonMinU32
//! let x = 123 as i32;
//! let y = NonMinI32::new(x).unwrap();
//! assert_eq!(y.get(), 123);
//!
//! // -2147483648 is the minimum value for a 32-bit integer.
//! let z = NonMinI32::new(-2147483648);
//! assert_eq!(z, None);
//! ```
//!
//! # Memory optimization
//! Simlarity to `NonZeroX` types from, these `NonMinX`/`NonMaxX` types allow for the niche filling
//! optimization. This means that types such as `Option<NonMinX>`/`Option<NonMaxX>` takes up the
//! same amount of space as `X`, while a regular `Option<X>` takes up twice the size of `X` due to
//! the need of storing the variant tag.
//!
//! ```
//! # use nonminmax::*;
//! use std::mem::size_of;
//!
//! // Option<u32> is larger than a regular u32
//! assert!(size_of::<Option<u32>>() == 2 * size_of::<u32>());
//!
//! // Option<NonMinU32>/Option<NonMaxU32> is the same size as a regular u32.
//! assert!(size_of::<Option<NonMinU32>>() == size_of::<u32>());
//! assert!(size_of::<Option<NonMaxU32>>() == size_of::<u32>());
//! ```
//!
//! While this may seem like a micro-optimization, it becomes important when frequently passing an
//! `Option<X>` around or when creating a large array of `Option<X>`.
//!
//! ```
//! # use nonminmax::*;
//! # use std::mem::size_of;
//! // 1000 x u32 takes up 4000 bytes
//! assert!(size_of::<[u32; 1000]>() == 4000);
//!
//! // 1000 x Option<u32> takes up 8000 bytes, ouch
//! assert!(size_of::<[Option<u32>; 1000]>() == 8000);
//!
//! // 1000 x Option<NonMaxU32> takes up only 4000 bytes
//! assert!(size_of::<[Option<NonMaxU32>; 1000]>() == 4000);
//! ```
//!
//! # Internal details
//! Internally, these types work by wrapping the existing `NonZeroX` types and xor-ing with a mask when
//! accessing the inner value. This means that there is the cost of a single `xor` instruction each
//! time `get` is called.
//!
//! # Supported types
//! The following types are supported
//! - `i8`/`u8`
//! - `i16`/`u16`
//! - `i32`/`u32`
//! - `i64`/`u64`
//! - `i128` / `u128`
//! - `isize` / `usize`
//!
//!

use core::fmt;
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

macro_rules! doc_comment {
    ($x:expr, $($tt:tt)*) => {
        #[doc=$x]
        $($tt)*
    }
}

/// Testing testing

macro_rules! impl_nontype {
    ($struct:ident, $nonzero:ident, $prim:ident, $mask:expr) => {

        doc_comment! {
            concat!("
            An integer of type `", stringify!($prim),"` which is known to not equal `", stringify!($mask), "`.
            
            
            This type allows for niche filling optimization (similar to the existing `std::num::NonZero*` types) 
            meaning items such as `Option<", stringify!($struct) ,">` and `Result<", stringify!($struct) ,", ()>` take up the same
            amount of space as `", stringify!($prim),"`.
            
            ```
            # use nonminmax::*;
            // Create using `new`, extract value using `get`
            let x = ", stringify!($struct) ,"::new(123).unwrap();
            assert_eq!(x.get(), 123);

            // The value cannot be `", stringify!($mask) ,"`
            let y = ", stringify!($struct) ,"::new(", stringify!($mask) ,");
            assert_eq!(y, None);

            // Niche filling optimization works!
            use std::mem::size_of;
            assert_eq!(size_of::<", stringify!($prim) ,">(), size_of::<", stringify!($struct) ,">());
            assert_eq!(size_of::<", stringify!($prim) ,">(), size_of::<Option<", stringify!($struct) ,">>());
            ```",
            ),
            #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #[repr(transparent)]
            pub struct $struct {
                value: $nonzero,
            }
        }

        impl $struct {
            doc_comment! {
                concat!("Creates an instance of `", stringify!($struct), "` by checking if the value is not `", stringify!($mask), "`."),
                #[inline(always)]
                pub fn new(value: $prim) -> Option<Self> {
                    if value != $mask {
                        unsafe { Some(Self::new_unchecked(value)) }
                    } else {
                        None
                    }
                }
            }

            doc_comment! {
                concat!("Creates an instance of `", stringify!($struct), "` without checking if the value is not `", stringify!($mask), "`.\n",
                " # Safety\n",
                "The value cannot be equal to `", stringify!($mask), "`."),
                #[inline(always)]
                pub unsafe fn new_unchecked(value: $prim) -> Self {
                    let value = $nonzero::new_unchecked(value ^ $mask);

                    Self { value }
                }
            }

            /// Returns the integer value.
            #[inline(always)]
            pub fn get(self) -> $prim {
                self.value.get() ^ $mask
            }
        }

        impl From<$struct> for $prim {
            fn from(nontype: $struct) -> Self {
                nontype.get()
            }
        }

        impl fmt::Debug for $struct {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, concat!(stringify!($struct), "({:?})"), self.get())
            }
        }

        impl fmt::Display for $struct {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                <_ as fmt::Display>::fmt(&self.get(), f)
            }
        }
    }
}

impl_nontype!(NonMaxU8, NonZeroU8, u8, u8::MAX);
impl_nontype!(NonMaxU16, NonZeroU16, u16, u16::MAX);
impl_nontype!(NonMaxU32, NonZeroU32, u32, u32::MAX);
impl_nontype!(NonMaxU64, NonZeroU64, u64, u64::MAX);
impl_nontype!(NonMaxU128, NonZeroU128, u128, u128::MAX);
impl_nontype!(NonMaxUsize, NonZeroUsize, usize, usize::MAX);

impl_nontype!(NonMaxI8, NonZeroI8, i8, i8::MAX);
impl_nontype!(NonMaxI16, NonZeroI16, i16, i16::MAX);
impl_nontype!(NonMaxI32, NonZeroI32, i32, i32::MAX);
impl_nontype!(NonMaxI64, NonZeroI64, i64, i64::MAX);
impl_nontype!(NonMaxI128, NonZeroI128, i128, i128::MAX);
impl_nontype!(NonMaxIsize, NonZeroIsize, isize, isize::MAX);

impl_nontype!(NonMinU8, NonZeroU8, u8, u8::MIN);
impl_nontype!(NonMinU16, NonZeroU16, u16, u16::MIN);
impl_nontype!(NonMinU32, NonZeroU32, u32, u32::MIN);
impl_nontype!(NonMinU64, NonZeroU64, u64, u64::MIN);
impl_nontype!(NonMinU128, NonZeroU128, u128, u128::MIN);
impl_nontype!(NonMinUsize, NonZeroUsize, usize, usize::MIN);

impl_nontype!(NonMinI8, NonZeroI8, i8, i8::MIN);
impl_nontype!(NonMinI16, NonZeroI16, i16, i16::MIN);
impl_nontype!(NonMinI32, NonZeroI32, i32, i32::MIN);
impl_nontype!(NonMinI64, NonZeroI64, i64, i64::MIN);
impl_nontype!(NonMinI128, NonZeroI128, i128, i128::MIN);
impl_nontype!(NonMinIsize, NonZeroIsize, isize, isize::MIN);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_nontype {
        ($test_name:ident, $struct:ident, $prim:ident, $mask:expr) => {
            #[test]
            fn $test_name() {
                // test for arbitrary value.
                let val = 123 as $prim;
                let x = $struct::new(val).unwrap();
                assert_eq!(x.get(), val);

                // test if what happens if value equals mask.
                let y = $struct::new($mask);
                assert_eq!(y, None);

                // test niche filling optimization.
                use core::mem::size_of;
                assert_eq!(size_of::<$struct>(), size_of::<$prim>());
                assert_eq!(size_of::<Option<$struct>>(), size_of::<$prim>());
                assert_eq!(size_of::<Result<$struct, ()>>(), size_of::<$prim>());
            }
        };
    }

    test_nontype!(test_nonmaxu8, NonMaxU8, u8, u8::MAX);
    test_nontype!(test_nonmaxu16, NonMaxU16, u16, u16::MAX);
    test_nontype!(test_nonmaxu32, NonMaxU32, u32, u32::MAX);
    test_nontype!(test_nonmaxu64, NonMaxU64, u64, u64::MAX);
    test_nontype!(test_nonmaxu128, NonMaxU128, u128, u128::MAX);
    test_nontype!(test_nonmaxusize, NonMaxUsize, usize, usize::MAX);

    test_nontype!(test_nonmaxi8, NonMaxI8, i8, i8::MAX);
    test_nontype!(test_nonmaxi16, NonMaxI16, i16, i16::MAX);
    test_nontype!(test_nonmaxi32, NonMaxI32, i32, i32::MAX);
    test_nontype!(test_nonmaxi64, NonMaxI64, i64, i64::MAX);
    test_nontype!(test_nonmaxi128, NonMaxI128, i128, i128::MAX);
    test_nontype!(test_nonmaxisize, NonMaxIsize, isize, isize::MAX);

    test_nontype!(test_nonminu8, NonMinU8, u8, u8::MIN);
    test_nontype!(test_nonminu16, NonMinU16, u16, u16::MIN);
    test_nontype!(test_nonminu32, NonMinU32, u32, u32::MIN);
    test_nontype!(test_nonminu64, NonMinU64, u64, u64::MIN);
    test_nontype!(test_nonminu128, NonMinU128, u128, u128::MIN);
    test_nontype!(test_nonminusize, NonMinUsize, usize, usize::MIN);

    test_nontype!(test_nonmini8, NonMinI8, i8, i8::MIN);
    test_nontype!(test_nonmini16, NonMinI16, i16, i16::MIN);
    test_nontype!(test_nonmini32, NonMinI32, i32, i32::MIN);
    test_nontype!(test_nonmini64, NonMinI64, i64, i64::MIN);
    test_nontype!(test_nonmini128, NonMinI128, i128, i128::MIN);
    test_nontype!(test_nonminisize, NonMinIsize, isize, isize::MIN);
}
