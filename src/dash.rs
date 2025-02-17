// This file was generated by the cargo-build script.

const DASH_CHUNK_SIZE : usize = 128;
const DASH_COLUMN_BITS : usize = 1;
const DASH_INDEX_LEN : usize = 543;
const DASH_INDEX_BITS : usize = 4;

const DASH_INDEX_BYTE_OFFSET : usize = 224;

const DASH_DATA: [u8; 497] = [
    // Column table
      0,  0,  0,  0,  0, 32,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  4,  0,  0,  0,  0,  0, 64,  0,  0,  0,  0,  0,  0,  0,  0,  1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     64,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 63,  0,  0,  0,  0,  0,  0,  0,  8,  0,  0,  0,  0,  8,
      0,  8,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  4,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,128,  4,  0,  0,  0, 12,  1,  0,  0, 32,  0,  0,  0,  0,  0,  0,  0, 16,  0,  0,  1,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  6,  0,  0,  0,  0,  1,  8,  0,  0,  0,
      0, 32,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 64,  0,  0,
    // Index table
     16, 17, 17, 17, 17, 33, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 19, 17, 17, 17, 20, 17, 17, 17, 17, 17, 17, 17,
    101, 17, 23, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 24, 17,169, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 27, 28,
     17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 29,  1,  1,
    // Padding to handle unaligned word reads.
      0,
];

/// Get the Dash attribute for a Unicode code-point.
///
/// # Arguments
///  - `code_point` A code-point in the form of a rust `char`.
///
/// # Returns
/// bool value
#[must_use] pub const fn get_dash(code_point: char) -> bool
{
    const INDEX_MASK : usize = (1 << DASH_INDEX_BITS) - 1;
    const COLUMN_MASK : usize = 1;

    let code_point_value = code_point as usize;
    let code_point_lo = code_point_value % DASH_CHUNK_SIZE;
    let mut code_point_hi = code_point_value / DASH_CHUNK_SIZE;
    if code_point_hi > DASH_INDEX_LEN - 1 {
        code_point_hi = DASH_INDEX_LEN - 1;
    }

    let index_offset = code_point_hi * DASH_INDEX_BITS;
    let index_byte_offset = index_offset / 8;
    let index_bit_offset = index_offset % 8;
    let mut index: usize = 0;
    index |= (DASH_DATA[DASH_INDEX_BYTE_OFFSET + index_byte_offset + 1] as usize) << 8;
    index |= (DASH_DATA[DASH_INDEX_BYTE_OFFSET + index_byte_offset + 0] as usize) << 0;
    index >>= index_bit_offset;
    index &= INDEX_MASK;

    let column_offset = (index * DASH_CHUNK_SIZE + code_point_lo) * DASH_COLUMN_BITS;
    let column_byte_offset = column_offset / 8;
    let column_bit_offset = column_offset % 8;

    let mut value: usize = 0;
    value |= (DASH_DATA[column_byte_offset + 0] as usize) << 0;
    value >>= column_bit_offset;
    value &= COLUMN_MASK;

    return match value {
        0 => false,
        _ => true,
    };
}

