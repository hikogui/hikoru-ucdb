use crate::build_src::column;
use convert_case::{Case, Casing};
use std::io::Write;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to write")]
    IO(#[from] std::io::Error),
    #[error("Failed to format")]
    Formatting(#[from] std::fmt::Error),
}

pub fn generate_enum_table(
    code_dir: &std::path::Path,
    name: &str,
    enum_values: &Vec<String>,
    column: &Vec<usize>,
    column_bits: usize,
    index: &Vec<usize>,
    index_bits: usize,
    chunk_size: usize,
) -> Result<(), Error> {
    let upper_name = name.to_case(Case::Constant);
    let camel_name = name.to_case(Case::Pascal);

    let column_bytes = column::compress(column, column_bits);
    let index_bytes = column::compress(index, index_bits);

    // These are the number of bytes to read to read a value in a single read instruction.
    let index_bytes_to_read = ((index_bits + 7) / 8 + 1).next_power_of_two();
    let column_bytes_to_read = ((column_bits + 7) / 8 + 1).next_power_of_two();

    let code_path = code_dir.join(format!("{}.rs", &name));
    let mut fd = std::fs::File::create(&code_path)?;

    write!(
        fd,
        "// This file was generated by the cargo-build script.\n\n"
    )?;

    write!(
        fd,
        "const {}_CHUNK_SIZE : usize = {};\n",
        upper_name, chunk_size
    )?;
    write!(
        fd,
        "const {}_COLUMN_BITS : usize = {};\n",
        upper_name, column_bits
    )?;
    write!(
        fd,
        "const {}_INDEX_LEN : usize = {};\n",
        upper_name,
        index.len()
    )?;
    write!(
        fd,
        "const {}_INDEX_BITS : usize = {};\n\n",
        upper_name, index_bits
    )?;
    write!(
        fd,
        "const {}_INDEX_BYTE_OFFSET : usize = {};\n\n",
        upper_name,
        column_bytes.len()
    )?;

    let data_bytes_len = column_bytes.len() + index_bytes.len() + index_bytes_to_read - 1;
    write!(
        fd,
        "const {}_DATA: [u8; {}] = [\n",
        upper_name, data_bytes_len
    )?;
    write!(fd, "    // Column table")?;
    for (i, v) in column_bytes.iter().enumerate() {
        if i % 32 == 0 {
            write!(fd, "\n    ")?;
        }
        write!(fd, "{:3},", v)?;
    }
    write!(fd, "\n    // Index table")?;
    for (i, v) in index_bytes.iter().enumerate() {
        if i % 32 == 0 {
            write!(fd, "\n    ")?;
        }
        write!(fd, "{:3},", v)?;
    }
    write!(fd, "\n    // Padding to handle unaligned word reads.\n    ")?;
    for _ in 1..index_bytes_to_read {
        write!(fd, "{:3},", 0)?;
    }
    write!(fd, "\n];\n\n")?;

    write!(
        fd,
        "/// The {} attribute for Unicode code-points.\n",
        camel_name
    )?;
    write!(fd, "#[derive(Debug,Clone,Copy,PartialEq)]\n")?;
    write!(fd, "pub enum {} {{\n", camel_name)?;
    for (i, v) in enum_values.iter().enumerate() {
        write!(fd, "    {} = {},\n", v.to_case(Case::Pascal), i)?;
    }
    write!(fd, "}}\n\n")?;

    write!(
        fd,
        "/// Get the {} attribute for a Unicode code-point.\n",
        camel_name
    )?;
    write!(fd, "///\n")?;
    write!(fd, "/// # Arguments\n")?;
    write!(
        fd,
        "///  - `code_point` A code-point in the form of a rust `char`.\n"
    )?;
    write!(fd, "///\n")?;
    write!(fd, "/// # Returns\n")?;
    write!(
        fd,
        "/// A {} attribute of the Unicode code-point.\n",
        camel_name
    )?;
    write!(
        fd,
        "#[must_use] pub const fn get_{}(code_point: char) -> {}\n",
        name, camel_name
    )?;
    write!(fd, "{{\n")?;
    write!(
        fd,
        "    const INDEX_MASK : usize = (1 << {}_INDEX_BITS) - 1;\n",
        upper_name
    )?;
    write!(
        fd,
        "    const COLUMN_MASK : usize = (1 << {}_COLUMN_BITS) - 1;\n\n",
        upper_name
    )?;

    write!(fd, "    let code_point_value = code_point as usize;\n")?;
    write!(
        fd,
        "    let code_point_lo = code_point_value % {}_CHUNK_SIZE;\n",
        upper_name
    )?;
    write!(
        fd,
        "    let mut code_point_hi = code_point_value / {}_CHUNK_SIZE;\n",
        upper_name
    )?;
    write!(
        fd,
        "    if code_point_hi > {}_INDEX_LEN - 1 {{\n",
        upper_name
    )?;
    write!(
        fd,
        "        code_point_hi = {}_INDEX_LEN - 1;\n",
        upper_name
    )?;
    write!(fd, "    }}\n\n")?;

    write!(
        fd,
        "    let index_offset = code_point_hi * {}_INDEX_BITS;\n",
        upper_name
    )?;
    write!(fd, "    let index_byte_offset = index_offset / 8;\n")?;
    write!(fd, "    let index_bit_offset = index_offset % 8;\n")?;
    write!(fd, "    let mut index: usize = 0;\n")?;
    for i in (0..index_bytes_to_read).rev() {
        write!(fd, "    index |= ({}_DATA[{}_INDEX_BYTE_OFFSET + index_byte_offset + {}] as usize) << {};\n", upper_name, upper_name, i, i * 8)?;
    }
    write!(fd, "    index >>= index_bit_offset;\n")?;
    write!(fd, "    index &= INDEX_MASK;\n\n")?;

    write!(
        fd,
        "    let column_offset = (index * {}_CHUNK_SIZE + code_point_lo) * {}_COLUMN_BITS;\n",
        upper_name, upper_name
    )?;
    write!(fd, "    let column_byte_offset = column_offset / 8;\n")?;
    write!(fd, "    let column_bit_offset = column_offset % 8;\n\n")?;

    write!(fd, "    let mut value: usize = 0;\n")?;
    for i in (0..column_bytes_to_read).rev() {
        write!(
            fd,
            "    value |= ({}_DATA[column_byte_offset + {}] as usize) << {};\n",
            upper_name,
            i,
            i * 8
        )?;
    }
    write!(fd, "    value >>= column_bit_offset;\n")?;
    write!(fd, "    value &= COLUMN_MASK;\n\n")?;

    write!(fd, "    return match value {{\n")?;
    for (i, v) in enum_values.iter().enumerate() {
        write!(fd, "        {} => {}::{},\n", i, camel_name, v.to_case(Case::Pascal))?;
    }
    write!(fd, "        _ => {}::{},\n", camel_name, enum_values[0].to_case(Case::Pascal))?;
    write!(fd, "    }};\n")?;
    write!(fd, "}}\n\n")?;

    return Ok(());
}

pub fn generate_bool_table(
    code_dir: &std::path::Path,
    name: &str,
    column: &Vec<usize>,
    index: &Vec<usize>,
    index_bits: usize,
    chunk_size: usize,
) -> Result<(), Error> {
    let upper_name = name.to_case(Case::Constant);
    let camel_name = name.to_case(Case::Pascal);

    let column_bytes = column::compress(column, 1);
    let index_bytes = column::compress(index, index_bits);

    // These are the number of bytes to read to read a value in a single read instruction.
    let index_bytes_to_read = ((index_bits + 7) / 8 + 1).next_power_of_two();
    let column_bytes_to_read = 1;

    let code_path = code_dir.join(format!("{}.rs", &name));
    let mut fd = std::fs::File::create(&code_path)?;

    write!(
        fd,
        "// This file was generated by the cargo-build script.\n\n"
    )?;

    write!(
        fd,
        "const {}_CHUNK_SIZE : usize = {};\n",
        upper_name, chunk_size
    )?;
    write!(
        fd,
        "const {}_COLUMN_BITS : usize = 1;\n",
        upper_name
    )?;
    write!(
        fd,
        "const {}_INDEX_LEN : usize = {};\n",
        upper_name,
        index.len()
    )?;
    write!(
        fd,
        "const {}_INDEX_BITS : usize = {};\n\n",
        upper_name, index_bits
    )?;
    write!(
        fd,
        "const {}_INDEX_BYTE_OFFSET : usize = {};\n\n",
        upper_name,
        column_bytes.len()
    )?;

    let data_bytes_len = column_bytes.len() + index_bytes.len() + index_bytes_to_read - 1;
    write!(
        fd,
        "const {}_DATA: [u8; {}] = [\n",
        upper_name, data_bytes_len
    )?;
    write!(fd, "    // Column table")?;
    for (i, v) in column_bytes.iter().enumerate() {
        if i % 32 == 0 {
            write!(fd, "\n    ")?;
        }
        write!(fd, "{:3},", v)?;
    }
    write!(fd, "\n    // Index table")?;
    for (i, v) in index_bytes.iter().enumerate() {
        if i % 32 == 0 {
            write!(fd, "\n    ")?;
        }
        write!(fd, "{:3},", v)?;
    }
    write!(fd, "\n    // Padding to handle unaligned word reads.\n    ")?;
    for _ in 1..index_bytes_to_read {
        write!(fd, "{:3},", 0)?;
    }
    write!(fd, "\n];\n\n")?;

    write!(
        fd,
        "/// Get the {} attribute for a Unicode code-point.\n",
        camel_name
    )?;
    write!(fd, "///\n")?;
    write!(fd, "/// # Arguments\n")?;
    write!(
        fd,
        "///  - `code_point` A code-point in the form of a rust `char`.\n"
    )?;
    write!(fd, "///\n")?;
    write!(fd, "/// # Returns\n")?;
    write!(fd, "/// bool value\n")?;
    write!(
        fd,
        "#[must_use] pub const fn get_{}(code_point: char) -> bool\n",
        name
    )?;
    write!(fd, "{{\n")?;
    write!(
        fd,
        "    const INDEX_MASK : usize = (1 << {}_INDEX_BITS) - 1;\n",
        upper_name
    )?;
    write!(
        fd,
        "    const COLUMN_MASK : usize = 1;\n\n"
    )?;

    write!(fd, "    let code_point_value = code_point as usize;\n")?;
    write!(
        fd,
        "    let code_point_lo = code_point_value % {}_CHUNK_SIZE;\n",
        upper_name
    )?;
    write!(
        fd,
        "    let mut code_point_hi = code_point_value / {}_CHUNK_SIZE;\n",
        upper_name
    )?;
    write!(
        fd,
        "    if code_point_hi > {}_INDEX_LEN - 1 {{\n",
        upper_name
    )?;
    write!(
        fd,
        "        code_point_hi = {}_INDEX_LEN - 1;\n",
        upper_name
    )?;
    write!(fd, "    }}\n\n")?;

    write!(
        fd,
        "    let index_offset = code_point_hi * {}_INDEX_BITS;\n",
        upper_name
    )?;
    write!(fd, "    let index_byte_offset = index_offset / 8;\n")?;
    write!(fd, "    let index_bit_offset = index_offset % 8;\n")?;
    write!(fd, "    let mut index: usize = 0;\n")?;
    for i in (0..index_bytes_to_read).rev() {
        write!(fd, "    index |= ({}_DATA[{}_INDEX_BYTE_OFFSET + index_byte_offset + {}] as usize) << {};\n", upper_name, upper_name, i, i * 8)?;
    }
    write!(fd, "    index >>= index_bit_offset;\n")?;
    write!(fd, "    index &= INDEX_MASK;\n\n")?;

    write!(
        fd,
        "    let column_offset = (index * {}_CHUNK_SIZE + code_point_lo) * {}_COLUMN_BITS;\n",
        upper_name, upper_name
    )?;
    write!(fd, "    let column_byte_offset = column_offset / 8;\n")?;
    write!(fd, "    let column_bit_offset = column_offset % 8;\n\n")?;

    write!(fd, "    let mut value: usize = 0;\n")?;
    for i in (0..column_bytes_to_read).rev() {
        write!(
            fd,
            "    value |= ({}_DATA[column_byte_offset + {}] as usize) << {};\n",
            upper_name,
            i,
            i * 8
        )?;
    }
    write!(fd, "    value >>= column_bit_offset;\n")?;
    write!(fd, "    value &= COLUMN_MASK;\n\n")?;

    write!(fd, "    return match value {{\n")?;
    write!(fd, "        0 => false,\n")?;
    write!(fd, "        _ => true,\n")?;
    write!(fd, "    }};\n")?;
    write!(fd, "}}\n\n")?;

    return Ok(());
}
