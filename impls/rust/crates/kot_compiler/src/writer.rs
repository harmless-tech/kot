#![allow(dead_code)]

// Writes a header of length + padding.
pub fn header(file: &mut Vec<u8>, header: &str, padding: usize) {
    let buffer = &[header.as_bytes(), &vec![0_u8; padding]].concat();
    file.extend_from_slice(buffer);
}

//<editor-fold desc="Primitive writing functions">

fn write_char(file: &mut Vec<u8>, data: char) {
    let buffer = &(data as u32).to_be_bytes();
    file.extend_from_slice(buffer);
}

fn write_string(file: &mut Vec<u8>, data: &String) {
    let buffer = data.as_bytes();

    write_u64(file, data.len() as u64);
    file.extend_from_slice(buffer);
}

fn write_i8(file: &mut Vec<u8>, data: i8) {
    let buffer = &(data).to_be_bytes();
    file.extend_from_slice(buffer);
}

fn write_i16(file: &mut Vec<u8>, data: i16) {
    let buffer = &(data).to_be_bytes();
    file.extend_from_slice(buffer);
}

fn write_i32(file: &mut Vec<u8>, data: i32) {
    let buffer = &(data).to_be_bytes();
    file.extend_from_slice(buffer);
}

fn write_i64(file: &mut Vec<u8>, data: i64) {
    let buffer = &(data).to_be_bytes();
    file.extend_from_slice(buffer);
}

fn write_i128(file: &mut Vec<u8>, data: i128) {
    let buffer = &(data).to_be_bytes();
    file.extend_from_slice(buffer);
}

fn write_u8(file: &mut Vec<u8>, data: u8) {
    let buffer = &(data).to_be_bytes();
    file.extend_from_slice(buffer);
}

fn write_u16(file: &mut Vec<u8>, data: u16) {
    let buffer = &(data).to_be_bytes();
    file.extend_from_slice(buffer);
}

fn write_u32(file: &mut Vec<u8>, data: u32) {
    let buffer = &(data).to_be_bytes();
    file.extend_from_slice(buffer);
}

fn write_u64(file: &mut Vec<u8>, data: u64) {
    let buffer = &(data).to_be_bytes();
    file.extend_from_slice(buffer);
}

fn write_u128(file: &mut Vec<u8>, data: u128) {
    let buffer = &(data).to_be_bytes();
    file.extend_from_slice(buffer);
}

fn write_f32(file: &mut Vec<u8>, data: f32) {
    let buffer = &(data).to_be_bytes();
    file.extend_from_slice(buffer);
}

fn write_f64(file: &mut Vec<u8>, data: f64) {
    let buffer = &(data).to_be_bytes();
    file.extend_from_slice(buffer);
}

fn write_bool(file: &mut Vec<u8>, data: bool) {
    let buffer = &(data as u8).to_be_bytes();
    file.extend_from_slice(buffer);
}

//</editor-fold>

//<editor-fold desc="OLD">
// #![allow(dead_code)] //TODO Remove!
//
// use hta_shared::{
//     hfs, traits::EnumWithU8, DataType, DebugData, Instructions, MetaData, Program, Tag, TagMap,
// };
// use std::{collections::HashMap, fs::File, io::Write};
//
// pub fn header(file: &mut File, header: &str) -> Result<(), String> {
//     if header.len() <= 8 {
//         let null_buffer = vec![0_u8; 8 - header.len()];
//         let buffer = &[header.as_bytes(), &null_buffer].concat();
//
//         hfs::error(file.by_ref().write_all(buffer))?;
//     }
//
//     Err("Header too long! Must only be 8 bytes.".to_string())
// }
//
// pub fn version(file: &mut File, v: (u64, u64, u64)) -> Result<(), String> {
//     write_u64(file, v.0)?;
//     write_u64(file, v.1)?;
//     write_u64(file, v.2)
// }
//
// pub fn debug_data(file: &mut File, data: &DebugData) -> Result<(), String> {
//     // native_lib_mappings
//     write_u64(file, data.native_lib_mappings.len() as u64)?;
//     for (name, mapping) in data.native_lib_mappings.iter() {
//         write_u64(file, *name)?;
//         write_string(file, mapping)?;
//     }
//
//     // call_function_mappings
//     write_u64(file, data.call_function_mappings.len() as u64)?;
//     for (name, mapping) in data.call_function_mappings.iter() {
//         write_u64(file, *name)?;
//         write_string(file, mapping)?;
//     }
//
//     // variable_name_mappings
//     write_u64(file, data.variable_name_mappings.len() as u64)?;
//     for (name, mapping) in data.variable_name_mappings.iter() {
//         write_u64(file, *name)?;
//         write_string(file, mapping)?;
//     }
//
//     // tag_name_mappings
//     write_u64(file, data.tag_name_mappings.len() as u64)?;
//     for (name, mapping) in data.tag_name_mappings.iter() {
//         write_u64(file, *name)?;
//         write_string(file, mapping)?;
//     }
//
//     // line_mappings
//     write_u64(file, data.line_mappings.len() as u64)?;
//     for (name, mapping) in data.line_mappings.iter() {
//         write_u64(file, name.0)?;
//         write_u64(file, name.1)?;
//         write_string(file, mapping)?;
//     }
//
//     Ok(())
// }
//
// pub fn metadata(file: &mut File, data: &MetaData) -> Result<(), String> {
//     write_string(file, &data.name)?; // name
//     write_string(file, &data.authors.join(", "))?; // authors
//     write_string(file, &data.version)?; // version
//     write_string(file, &data.website)?; // website
//     write_string(file, &data.git)?; // git
//     write_string(file, &data.license)?; // license
//
//     // natives
//     write_u64(file, data.natives.len() as u64)?; // Amount of natives.
//     for native in data.natives.iter() {
//         write_string(file, &native)?;
//     }
//
//     Ok(())
// }
//
// pub fn program(file: &mut File, data: &Program) -> Result<(), String> {
//     tags(file, &data.tags)?;
//     instructions(file, &data.instructions)
// }
//
// fn tags(file: &mut File, data: &HashMap<Tag, TagMap>) -> Result<(), String> {
//     write_u64(file, data.len() as u64)?; // Amount of tags.
//     for (tag, loc) in data.iter() {
//         write_u64(file, *tag)?;
//         write_u64(file, loc.0)?;
//         write_u64(file, loc.1)?;
//     }
//
//     Ok(())
// }
//
// fn instructions(file: &mut File, data: &Vec<Vec<Instructions>>) -> Result<(), String> {
//     write_u64(file, data.len() as u64)?; // Amount of frames.
//
//     for frame in data.iter() {
//         write_u64(file, frame.len() as u64)?; // Amount of instructions.
//
//         for instruction in frame.iter() {
//             write_u8(file, instruction.to_u8())?;
//
//             match instruction {
//                 Instructions::Alloc(var, dat) => {
//                     write_u64(file, *var)?;
//                     data_type(file, dat)?;
//                 }
//                 Instructions::DeAlloc(var) => write_u64(file, *var)?,
//                 Instructions::SetVar(var, dat) => {
//                     write_u64(file, *var)?;
//                     data_type(file, dat)?;
//                 }
//                 Instructions::RegVar(var) => write_u64(file, *var)?,
//                 Instructions::SetReg(reg, dat) => {
//                     write_u8(file, reg.to_u8())?;
//                     data_type(file, dat)?;
//                 }
//                 Instructions::VarReg(var, reg) => {
//                     write_u64(file, *var)?;
//                     write_u8(file, reg.to_u8())?;
//                 }
//                 Instructions::CpyReg(reg1, reg2) => {
//                     write_u8(file, reg1.to_u8())?;
//                     write_u8(file, reg2.to_u8())?;
//                 }
//                 Instructions::Op(op) => write_u8(file, op.to_u8())?,
//                 Instructions::Jmp(tag) => write_u64(file, *tag)?,
//                 Instructions::PushJmp(tag) => write_u64(file, *tag)?,
//                 Instructions::PopJmp => {}
//                 Instructions::Cast(dat) => data_type(file, dat)?,
//                 Instructions::Call(nat) => write_u64(file, *nat)?,
//                 Instructions::Exit(code) => write_i32(file, *code)?,
//                 Instructions::Assert(_op_dat) => {
//                     return Err("The Instruction Assert is not supported yet!".to_string())
//                 }
//             }
//         }
//     }
//
//     Ok(())
// }
//
// fn data_type(file: &mut File, data: &DataType) -> Result<(), String> {
//     write_u8(file, data.to_u8())?;
//
//     match data {
//         DataType::Char(chr) => write_char(file, *chr)?,
//         DataType::String(str) => write_string(file, str)?,
//         DataType::Int8(int) => write_i8(file, *int)?,
//         DataType::Int16(int) => write_i16(file, *int)?,
//         DataType::Int32(int) => write_i32(file, *int)?,
//         DataType::Int64(int) => write_i64(file, *int)?,
//         DataType::Int128(int) => write_i128(file, *int)?,
//         DataType::UInt8(uint) => write_u8(file, *uint)?,
//         DataType::UInt16(uint) => write_u16(file, *uint)?,
//         DataType::UInt32(uint) => write_u32(file, *uint)?,
//         DataType::UInt64(uint) => write_u64(file, *uint)?,
//         DataType::UInt128(uint) => write_u128(file, *uint)?,
//         DataType::Float32(float) => write_f32(file, *float)?,
//         DataType::Float64(float) => write_f64(file, *float)?,
//         DataType::Boolean(boolean) => write_bool(file, *boolean)?,
//     }
//
//     Ok(())
// }
//
// // Primitive writing functions.
//
// fn write_char(file: &mut File, data: char) -> Result<(), String> {
//     let buffer = &(data as u32).to_be_bytes();
//     hfs::error(file.by_ref().write_all(buffer))
// }
//
// fn write_string(file: &mut File, data: &String) -> Result<(), String> {
//     let buffer = data.as_bytes();
//
//     write_u64(file, data.len() as u64)?;
//     hfs::error(file.by_ref().write_all(buffer))
// }
//
// fn write_i8(file: &mut File, data: i8) -> Result<(), String> {
//     let buffer = &(data).to_be_bytes();
//     hfs::error(file.by_ref().write_all(buffer))
// }
//
// fn write_i16(file: &mut File, data: i16) -> Result<(), String> {
//     let buffer = &(data).to_be_bytes();
//     hfs::error(file.by_ref().write_all(buffer))
// }
//
// fn write_i32(file: &mut File, data: i32) -> Result<(), String> {
//     let buffer = &(data).to_be_bytes();
//     hfs::error(file.by_ref().write_all(buffer))
// }
//
// fn write_i64(file: &mut File, data: i64) -> Result<(), String> {
//     let buffer = &(data).to_be_bytes();
//     hfs::error(file.by_ref().write_all(buffer))
// }
//
// fn write_i128(file: &mut File, data: i128) -> Result<(), String> {
//     let buffer = &(data).to_be_bytes();
//     hfs::error(file.by_ref().write_all(buffer))
// }
//
// fn write_u8(file: &mut File, data: u8) -> Result<(), String> {
//     let buffer = &(data).to_be_bytes();
//     hfs::error(file.by_ref().write_all(buffer))
// }
//
// fn write_u16(file: &mut File, data: u16) -> Result<(), String> {
//     let buffer = &(data).to_be_bytes();
//     hfs::error(file.by_ref().write_all(buffer))
// }
//
// fn write_u32(file: &mut File, data: u32) -> Result<(), String> {
//     let buffer = &(data).to_be_bytes();
//     hfs::error(file.by_ref().write_all(buffer))
// }
//
// fn write_u64(file: &mut File, data: u64) -> Result<(), String> {
//     let buffer = &(data).to_be_bytes();
//     hfs::error(file.by_ref().write_all(buffer))
// }
//
// fn write_u128(file: &mut File, data: u128) -> Result<(), String> {
//     let buffer = &(data).to_be_bytes();
//     hfs::error(file.by_ref().write_all(buffer))
// }
//
// fn write_f32(file: &mut File, data: f32) -> Result<(), String> {
//     let buffer = &(data).to_be_bytes();
//     hfs::error(file.by_ref().write_all(buffer))
// }
//
// fn write_f64(file: &mut File, data: f64) -> Result<(), String> {
//     let buffer = &(data).to_be_bytes();
//     hfs::error(file.by_ref().write_all(buffer))
// }
//
// fn write_bool(file: &mut File, data: bool) -> Result<(), String> {
//     let buffer = &(data as u8).to_be_bytes();
//     hfs::error(file.by_ref().write_all(buffer))
// }
//</editor-fold>
