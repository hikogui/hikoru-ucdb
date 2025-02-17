// This file was generated by the cargo-build script.

const ID_COMPAT_MATH_START_CHUNK_SIZE : usize = 256;
const ID_COMPAT_MATH_START_COLUMN_BITS : usize = 1;
const ID_COMPAT_MATH_START_INDEX_LEN : usize = 473;
const ID_COMPAT_MATH_START_INDEX_BITS : usize = 2;

const ID_COMPAT_MATH_START_INDEX_BYTE_OFFSET : usize = 128;

const ID_COMPAT_MATH_START_DATA: [u8; 248] = [
    // Column table
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    132,  0,  0, 64,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  2,  0,  0,  8,  0,  0,  0,  8,
      0,  0, 32,  0,  0,  0, 32,  0,  0,128,  0,  0,  0,128,  0,  0,  0,  2,  0,  0,  0,  2,  0,  0,  8,  0,  0,  0,  0,  0,  0,  0,
    // Index table
      0,  0,  0,  0,  0,  0,  0,  0, 16,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,224,  0,
    // Padding to handle unaligned word reads.
      0,
];

/// Get the IdCompatMathStart attribute for a Unicode code-point.
///
/// # Arguments
///  - `code_point` A code-point in the form of a rust `char`.
///
/// # Returns
/// bool value
#[must_use] pub const fn get_id_compat_math_start(code_point: char) -> bool
{
    const INDEX_MASK : usize = (1 << ID_COMPAT_MATH_START_INDEX_BITS) - 1;
    const COLUMN_MASK : usize = 1;

    let code_point_value = code_point as usize;
    let code_point_lo = code_point_value % ID_COMPAT_MATH_START_CHUNK_SIZE;
    let mut code_point_hi = code_point_value / ID_COMPAT_MATH_START_CHUNK_SIZE;
    if code_point_hi > ID_COMPAT_MATH_START_INDEX_LEN - 1 {
        code_point_hi = ID_COMPAT_MATH_START_INDEX_LEN - 1;
    }

    let index_offset = code_point_hi * ID_COMPAT_MATH_START_INDEX_BITS;
    let index_byte_offset = index_offset / 8;
    let index_bit_offset = index_offset % 8;
    let mut index: usize = 0;
    index |= (ID_COMPAT_MATH_START_DATA[ID_COMPAT_MATH_START_INDEX_BYTE_OFFSET + index_byte_offset + 1] as usize) << 8;
    index |= (ID_COMPAT_MATH_START_DATA[ID_COMPAT_MATH_START_INDEX_BYTE_OFFSET + index_byte_offset + 0] as usize) << 0;
    index >>= index_bit_offset;
    index &= INDEX_MASK;

    let column_offset = (index * ID_COMPAT_MATH_START_CHUNK_SIZE + code_point_lo) * ID_COMPAT_MATH_START_COLUMN_BITS;
    let column_byte_offset = column_offset / 8;
    let column_bit_offset = column_offset % 8;

    let mut value: usize = 0;
    value |= (ID_COMPAT_MATH_START_DATA[column_byte_offset + 0] as usize) << 0;
    value >>= column_bit_offset;
    value &= COLUMN_MASK;

    return match value {
        0 => false,
        _ => true,
    };
}

