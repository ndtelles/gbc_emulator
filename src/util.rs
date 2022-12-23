use std::ops::{BitAnd, BitOr, Not};

use num_traits::{ops::wrapping::WrappingSub, Unsigned, WrappingAdd};

// Return result and a bitmap of which bits carried
pub fn add_and_get_carries<
    T: WrappingAdd + BitAnd<Output = T> + BitOr<Output = T> + Not<Output = T> + Unsigned + Copy,
>(
    operand1: T,
    operand2: T,
) -> (T, T) {
    let result = operand1.wrapping_add(&operand2);
    // Generated from truth table
    let carries =
        (!operand1 & operand2 & !result) | (operand1 & !operand2 & !result) | (operand1 & operand2);
    (result, carries)
}

// Return result and a bitmap of which bits borrowed
pub fn subtract_and_get_borrows<T: WrappingSub + BitAnd<Output = T> + Unsigned + Copy>(
    minuend: T,
    subtrahend: T,
) -> (T, T) {
    let result = minuend.wrapping_sub(&subtrahend);
    // Generated from truth table
    let borrows = subtrahend & result;
    (result, borrows)
}
