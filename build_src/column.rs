

use crate::build_src::CodePointDescription;


/// Copy a chunk from the source-chunk to the destination-chunk.
/// 
/// # Arguments
///  - `column`: The column to deduplicate, this column will be mutated by this function.
///  - `dst_chunk`: The index to the chunk where the data will be copied to.
///  - `src_chunk`: The index to the chunk where the data will be copied from.
///  - `chunk_size`: The size of the chunks.
///
fn copy_chunk(src: &Vec<u32>, src_chunk: usize, dst: &mut Vec<u32>, dst_chunk: usize, chunk_size: usize)
{
    let dst_offset = dst_chunk * chunk_size;
    let src_offset = src_chunk * chunk_size;

    let dst_size = dst_offset + chunk_size;
    if dst.len() < dst_size {
        dst.resize(dst_size, 0);
    }

    for i in 0..chunk_size {
        dst[dst_offset + i] = src[src_offset + i];
    }
}

fn test_chunk(src: &Vec<u32>, src_chunk: usize, dst: &Vec<u32>, dst_chunk: usize, chunk_size: usize) -> bool
{
    let src_offset = src_chunk * chunk_size;
    let dst_offset = dst_chunk * chunk_size;

    for i in 0..chunk_size {
        if dst[dst_offset + i] != src[src_offset + i] {
            return false;
        }
    }
    return true;
}

/// Test if any of the chunks previous from the `dst_chunk` are equal to the
/// `src_chunk`.
/// 
/// # Arguments
///  - `src': The data source.
///  - `src_chunk`: The chunk-index to the source-chunk.
///  - `dst`: The deduplicated table.
///  - `dst_chunk`: The chunk-index to the destination-chunk where the
///                 source-chunk will be copied to; if all chunks from
///                 `0..dst_chunk` are unequal to source-chunk.
///  - `chunk_size`: The size of the chunks.
/// 
/// # Returns
/// The index to the first chunk that is equal to the chunk pointed to by
/// `src_chunk`, or `dst_chunk` if no such chunk exists.
/// 
/// The `dst_chunk` is returned so that you can directly copy the source-chunk
/// to the correct position.
/// 
fn test_chunks(src: &Vec<u32>, src_chunk: usize, dst: &Vec<u32>, dst_chunk: usize, chunk_size: usize) -> usize
{
    for i in 0..dst_chunk {
        if test_chunk(src, src_chunk, dst, i, chunk_size) {
            return i;
        }
    }
    return dst_chunk;
}

/// Deduplicate a column.
/// 
/// # Arguments
///  - `column` : The column to deduplicate. The column is modified in-place.
///  - `chunk_size` : The size of the chunks.
/// 
/// # Returns
/// (dedupped-data-table, index-table).
/// 
/// To find a specific entry `i` in the deduplicated column:
///  - `chunk_nr = min(i / chunk_size, index_table.len())`
///  - `offset = index_table[chunk_nr] * chunk_size + i % chunk_size`
///  - `entry = column[offset]`
/// 
pub fn dedup(column: &Vec<u32>, chunk_size: usize) -> (Vec<u32>, Vec<u32>)
{
    let num_chunks = 0x110000 / chunk_size;
    let mut index_table = Vec::<u32>::new();
    let mut dedup_table = Vec::<u32>::new();

    // Deduplicating the column table and create an index table.
    let mut dst_chunk = 0;
    for src_chunk in 0..num_chunks {
        let found_chunk = test_chunks(column, src_chunk, &dedup_table, dst_chunk, chunk_size);
        index_table.push(found_chunk as u32);
        if found_chunk == dst_chunk {
            copy_chunk(column, src_chunk, &mut dedup_table, dst_chunk, chunk_size);
            dst_chunk += 1;
        }
    }

    // Truncate the index_table, so that the last value is not repeating.
    if let Some(&last_value) = index_table.last() {
        loop {
            match index_table.last() {
                None => break,
                Some(&x) => {
                    if x == last_value {
                        index_table.pop();
                    } else {
                        break;
                    }
                },
            }
        }
        index_table.push(last_value);
    }

    return (dedup_table, index_table);
}

pub fn dedup_best_fit(column: &Vec<u32>) -> (Vec<u32>, usize, Vec<u32>, usize, usize)
{
    let chunk_sizes = vec![
        32 as usize,
        64 as usize,
        128 as usize,
        256 as usize,
        512 as usize,
    ];

    let mut best_chunk_size: usize = 0;
    let mut best_dedup = Vec::<u32>::new();
    let mut best_index = Vec::<u32>::new();
    let mut best_dedup_bits: usize = 0;
    let mut best_index_bits: usize = 0;
    let mut best_byte_len: usize = 0;
    for chunk_size in chunk_sizes {
        let (dedup, index) = dedup(&column, chunk_size);
        let dedup_bits = get_width(&dedup);
        let index_bits = get_width(&index);

        let byte_len = dedup.len() * dedup_bits + index.len() * index_bits;
        if best_byte_len == 0 || byte_len < best_byte_len {
            best_chunk_size = chunk_size;
            best_dedup = dedup;
            best_index = index;
            best_dedup_bits = dedup_bits;
            best_index_bits = index_bits;
            best_byte_len = byte_len;
        }
    }

    return (best_dedup, best_dedup_bits, best_index, best_index_bits, best_chunk_size);
}

pub fn map_str_to_int<'a>(order: &mut Vec<String>, descriptions: &'a Vec<CodePointDescription>, op: impl Fn(&'a CodePointDescription) -> &'a String) -> Vec<u32>
{
    let mut r = Vec::<u32>::with_capacity(0x110000);
    r.resize(0x110000, 0);

    for cp in 0..0x110000 {
        let str_val = op(&descriptions[cp]);

        if let Some(x) = order.iter().position(|x| x == str_val) {
            r[cp] = x as u32;
        } else {
            r[cp] = order.len() as u32;
            order.push(str_val.to_string()); 
        }
    }

    return r;
}

pub fn map_char_to_int<'a>(descriptions: &'a Vec<CodePointDescription>, op: impl Fn(&'a CodePointDescription) -> &'a Option<char>) -> Vec<u32>
{
    let mut r = Vec::<u32>::with_capacity(0x110000);
    r.resize(0x110000, 0);

    for cp in 0..0x110000 {
        if let Some(c) = op(&descriptions[cp]) {
            assert!(*c != '\0');
            r[cp] = *c as u32;
            assert!(r[cp] != 0x1ffff);
        } else {
            r[cp] = 0;
        }
    }

    return r;
}

pub fn map_bool_to_int<'a>(descriptions: &'a Vec<CodePointDescription>, op: impl Fn(&'a CodePointDescription) -> bool) -> Vec<u32>
{
    let mut r = Vec::<u32>::with_capacity(0x110000);
    r.resize(0x110000, 0);

    for cp in 0..0x110000 {
        if op(&descriptions[cp]) {
            r[cp] = 1;
        } else {
            r[cp] = 0;
        }
    }

    return r;
}

fn compress_insert_value(bytes: &mut Vec<u8>, offset : usize, mut value : u32)
{
    let mut byte_offset = offset / 8;
    let bit_offset = offset % 8;

    value <<= bit_offset;
    while value != 0 {
        bytes[byte_offset] |= (value & 255) as u8;
        value >>= 8;
        byte_offset += 1;
    }
}

/// Compress and integer tables into tightly packed bytes.
///
pub fn compress(input: &Vec<u32>, num_bits: usize) -> Vec<u8>
{
    let total_num_bits = num_bits * input.len();
    let total_num_bytes = (total_num_bits + 7) / 8;

    let mut r = Vec::<u8>::with_capacity(total_num_bytes);
    r.resize(total_num_bytes, 0);

    let mut offset : usize = 0;
    for x in input {
        compress_insert_value(&mut r, offset, *x);
        offset += num_bits;
    }

    return r;
}

pub fn get_width(input: &Vec<u32>) -> usize
{
    let max = *input.iter().max().unwrap_or(&0);
    return (max + 1).next_power_of_two().trailing_zeros() as usize;
}

