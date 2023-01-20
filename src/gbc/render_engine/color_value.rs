// Map 5bit color to 8 bit color
const RGB555_TO_RGB888_LOOKUP: [u8; 32] = [
    0, 8, 16, 25, 33, 41, 49, 58, 66, 74, 82, 90, 99, 107, 115, 123, 132, 140, 148, 156, 165, 173,
    181, 189, 197, 206, 214, 222, 230, 239, 247, 255,
];

pub(super) fn rgb555_to_rgb888(rgb555: &[u8; 3]) -> [u8; 3] {
    rgb555.map(|val| RGB555_TO_RGB888_LOOKUP[val as usize])
}
