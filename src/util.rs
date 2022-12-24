use std::mem::size_of;

use num_traits::{ops::wrapping::WrappingSub, PrimInt, Unsigned, WrappingAdd};

// Return result and a bitmap of which bits carried
pub fn add_and_get_carries<T: WrappingAdd + PrimInt + Unsigned>(lhs: T, rhs: T) -> (T, T) {
    let sum = lhs.wrapping_add(&rhs);
    // Derived from truth table. Map which digits passed a carry to their significant neighbor
    let carries = (!lhs & rhs & !sum) | (lhs & !rhs & !sum) | (lhs & rhs);
    (sum, carries)
}

// Return result and a bitmap of which bits borrowed
pub fn subtract_and_get_borrows<T: WrappingSub + PrimInt + Unsigned>(lhs: T, rhs: T) -> (T, T) {
    let diff = lhs.wrapping_sub(&rhs);
    // Derived from truth table. Map which digits borrowed from their significant neighbor
    let borrows = (!lhs & !rhs & diff) | (!lhs & rhs & !diff) | (rhs & diff);
    (diff, borrows)
}

pub fn index_bitmap<T: PrimInt + Into<usize>>(bitmap: T, index: usize) -> bool {
    debug_assert!((size_of::<T>() * 8) > index);
    (bitmap.into() & (0x01 << index)) != 0
}
