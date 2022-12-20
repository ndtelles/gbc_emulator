use enum_map::{Enum};

#[derive(Clone, Copy, Enum)]
pub enum Register {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}
