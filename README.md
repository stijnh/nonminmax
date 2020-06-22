# Integers types which cannot be their minimum/maximum value.

The standard library contains a collection of `std::num::NonZeroX` types: integer types which
cannot be zero. This crate extends this idea further by providing `NonMinX`/`NonMaxX`: integer
types which cannot be their minimum/maximum value.

```Rust
// Create a regular NonMinU32
let x = 123 as i32;
let y = NonMinI32::new(x).unwrap();
assert_eq!(y.get(), 123);

// -2147483648 is the minimum value for a 32-bit integer.
let z = NonMinI32::new(-2147483648);
assert_eq!(z, None);
```

# Memory optimization
Similar to `NonZeroX` types, these `NonMinX`/`NonMaxX` types allow for the niche filling
optimization. This means that types such as `Option<NonMinX>`/`Option<NonMaxX>` takes up the
same amount of space as `X`, while a regular `Option<X>` takes up twice the size of `X` due to
the need of storing the variant tag.

```Rust
// Option<u32> is larger than a regular u32
assert!(size_of::<Option<u32>>() == 2 * size_of::<u32>());

// Option<NonMinU32>/Option<NonMaxU32> is the same size as a regular u32.
assert!(size_of::<Option<NonMinU32>>() == size_of::<u32>());
assert!(size_of::<Option<NonMaxU32>>() == size_of::<u32>());
```

While this may seem like a micro-optimization, it becomes important when frequently passing an
`Option<X>` around or when creating a large array of `Option<X>`.

```Rust
// 1000 x u32 takes up 4000 bytes
assert!(size_of::<[u32; 1000]>() == 4000);

// 1000 x Option<u32> takes up 8000 bytes, ouch
assert!(size_of::<[Option<u32>; 1000]>() == 8000);

// 1000 x Option<NonMaxU32> takes up only 4000 bytes
assert!(size_of::<[Option<NonMaxU32>; 1000]>() == 4000);
```

# Internal details
Internally, these types work by wrapping the existing `NonZeroX` types and xor-ing with a mask when
accessing the inner value. This means that there is the cost of a single `xor` instruction each
time `get` is called.

# Supported types
The following types are supported
- `i8`/`u8`
- `i16`/`u16`
- `i32`/`u32`
- `i64`/`u64`
- `i128` / `u128`
- `isize` / `usize`


