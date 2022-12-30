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

pub fn set_bit(val: u8, bit: usize) -> u8 {
    debug_assert!((size_of::<u8>() * 8) > bit);
    val | (0x1u8 << bit)
}

pub fn reset_bit(val: u8, bit: usize) -> u8 {
    debug_assert!((size_of::<u8>() * 8) > bit);
    val & (!(0x1u8 << bit))
}

pub fn add_i8_to_u16(lhs: u16, rhs: i8) -> (u16, u16) {
    if rhs.is_negative() {
        // Sign extend operand to i16 then multiply by -1 to make it positive.
        // Finally convert positive value to u16. We need to cast to i16 before
        // multiply so value 128 doesn't overflow in i8. Yaaay Rust -_-
        subtract_and_get_borrows(lhs, ((rhs as i16) * -1) as u16)
    } else {
        add_and_get_carries(lhs, rhs as u16)
    }
}

pub trait Bytes {
    fn high(self) -> u8;
    fn low(self) -> u8;
}

impl Bytes for u16 {
    fn high(self) -> u8 {
        (self >> 8) as u8
    }

    fn low(self) -> u8 {
        self as u8
    }
}
