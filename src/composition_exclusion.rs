// This file was generated by the cargo-build script.

const COMPOSITION_EXCLUSION_CHUNK_SIZE : usize = 256;
const COMPOSITION_EXCLUSION_COLUMN_BITS : usize = 1;
const COMPOSITION_EXCLUSION_INDEX_LEN : usize = 467;
const COMPOSITION_EXCLUSION_INDEX_BITS : usize = 3;

const COMPOSITION_EXCLUSION_INDEX_BYTE_OFFSET : usize = 256;

const COMPOSITION_EXCLUSION_DATA: [u8; 433] = [
    // Column table
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,255,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,176,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0, 72,  0,  0,  0,  0, 78,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 48,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  8, 32,132, 16,  0,  2, 64,  1,  0,  0,  8, 32,132, 16,  0,  2,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 16,  0,  0,  0,  0,
      0,  0,  0,160,  0,252,127, 95,219,127,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,192, 31,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,248,  1,  0,  0,  0,  0,  0,  0,  0,
    // Index table
      0,  0,  0,136,  6,128,  0,  0,  0,  0,  0,  0,  0,  0,  0, 64,  1,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 12,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 56,  0,
    // Padding to handle unaligned word reads.
      0,
];

/// Get the CompositionExclusion attribute for a Unicode code-point.
///
/// # Arguments
///  - `code_point` A code-point in the form of a rust `char`.
///
/// # Returns
/// bool value
#[must_use] pub const fn get_composition_exclusion(code_point: char) -> bool
{
    const INDEX_MASK : usize = (1 << COMPOSITION_EXCLUSION_INDEX_BITS) - 1;
    const COLUMN_MASK : usize = 1;

    let code_point_value = code_point as usize;
    let code_point_lo = code_point_value % COMPOSITION_EXCLUSION_CHUNK_SIZE;
    let mut code_point_hi = code_point_value / COMPOSITION_EXCLUSION_CHUNK_SIZE;
    if code_point_hi > COMPOSITION_EXCLUSION_INDEX_LEN - 1 {
        code_point_hi = COMPOSITION_EXCLUSION_INDEX_LEN - 1;
    }

    let index_offset = code_point_hi * COMPOSITION_EXCLUSION_INDEX_BITS;
    let index_byte_offset = index_offset / 8;
    let index_bit_offset = index_offset % 8;
    let mut index: usize = 0;
    index |= (COMPOSITION_EXCLUSION_DATA[COMPOSITION_EXCLUSION_INDEX_BYTE_OFFSET + index_byte_offset + 1] as usize) << 8;
    index |= (COMPOSITION_EXCLUSION_DATA[COMPOSITION_EXCLUSION_INDEX_BYTE_OFFSET + index_byte_offset + 0] as usize) << 0;
    index >>= index_bit_offset;
    index &= INDEX_MASK;

    let column_offset = (index * COMPOSITION_EXCLUSION_CHUNK_SIZE + code_point_lo) * COMPOSITION_EXCLUSION_COLUMN_BITS;
    let column_byte_offset = column_offset / 8;
    let column_bit_offset = column_offset % 8;

    let mut value: usize = 0;
    value |= (COMPOSITION_EXCLUSION_DATA[column_byte_offset + 0] as usize) << 0;
    value >>= column_bit_offset;
    value &= COLUMN_MASK;

    return match value {
        0 => false,
        _ => true,
    };
}

