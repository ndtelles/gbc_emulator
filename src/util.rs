use std::mem::size_of;

use num_traits::{ops::wrapping::WrappingSub, PrimInt, Unsigned, WrappingAdd};

// Return result and a bitmap of which bits carried
pub fn add_and_get_carries<T: WrappingAdd + PrimInt + Unsigned>(
    operand1: T,
    operand2: T,
) -> (T, T) {
    let result = operand1.wrapping_add(&operand2);
    // Derived from truth table. Map which digits passed a carry to their significant neighbor
    let carries =
        (!operand1 & operand2 & !result) | (operand1 & !operand2 & !result) | (operand1 & operand2);
    (result, carries)
}

// Return result and a bitmap of which bits borrowed
pub fn subtract_and_get_borrows<T: WrappingSub + PrimInt + Unsigned>(
    minuend: T,
    subtrahend: T,
) -> (T, T) {
    let result = minuend.wrapping_sub(&subtrahend);
    // Derived from truth table. Map which digits borrowed from their significant neighbor
    let borrows = (!minuend & !subtrahend & result)
        | (!minuend & subtrahend & !result)
        | (subtrahend & result);
    (result, borrows)
}

pub fn index_bitmap<T: PrimInt + Into<usize>>(bitmap: T, index: usize) -> bool {
    debug_assert!((size_of::<T>() * 8) > index);
    (bitmap.into() & (0x01 << index)) != 0
}
