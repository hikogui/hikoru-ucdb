// This file was generated by the cargo-build script.

const RADICAL_CHUNK_SIZE : usize = 128;
const RADICAL_COLUMN_BITS : usize = 1;
const RADICAL_INDEX_LEN : usize = 97;
const RADICAL_INDEX_BITS : usize = 2;

const RADICAL_INDEX_BYTE_OFFSET : usize = 64;

const RADICAL_DATA: [u8; 90] = [
    // Column table
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,255,255,255,251,255,255,255,255,255,255,255,255,255,255, 15,  0,
    255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255, 63,  0,  0,  0,  0,  0,
    // Index table
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,228,  0,
    // Padding to handle unaligned word reads.
      0,
];

/// Get the Radical attribute for a Unicode code-point.
///
/// # Arguments
///  - `code_point` A code-point in the form of a rust `char`.
///
/// # Returns
/// bool value
#[must_use] pub const fn get_radical(code_point: char) -> bool
{
    const INDEX_MASK : usize = (1 << RADICAL_INDEX_BITS) - 1;
    const COLUMN_MASK : usize = 1;

    let code_point_value = code_point as usize;
    let code_point_lo = code_point_value % RADICAL_CHUNK_SIZE;
    let mut code_point_hi = code_point_value / RADICAL_CHUNK_SIZE;
    if code_point_hi > RADICAL_INDEX_LEN - 1 {
        code_point_hi = RADICAL_INDEX_LEN - 1;
    }

    let index_offset = code_point_hi * RADICAL_INDEX_BITS;
    let index_byte_offset = index_offset / 8;
    let index_bit_offset = index_offset % 8;
    let mut index: usize = 0;
    index |= (RADICAL_DATA[RADICAL_INDEX_BYTE_OFFSET + index_byte_offset + 1] as usize) << 8;
    index |= (RADICAL_DATA[RADICAL_INDEX_BYTE_OFFSET + index_byte_offset + 0] as usize) << 0;
    index >>= index_bit_offset;
    index &= INDEX_MASK;

    let column_offset = (index * RADICAL_CHUNK_SIZE + code_point_lo) * RADICAL_COLUMN_BITS;
    let column_byte_offset = column_offset / 8;
    let column_bit_offset = column_offset % 8;

    let mut value: usize = 0;
    value |= (RADICAL_DATA[column_byte_offset + 0] as usize) << 0;
    value >>= column_bit_offset;
    value &= COLUMN_MASK;

    return match value {
        0 => false,
        _ => true,
    };
}

