#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

pub mod def;
pub mod error;

use alloc::ffi::CString;
use ckb_script_ipc_common::spawn::spawn_server;
use ckb_std::{ckb_constants::Source, log::info};

#[cfg(not(test))]
use ckb_std::default_alloc;
use ckb_std::logger;
use crate::def::WorldClient;
use crate::error::Error;
use alloc::string::String;

#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();

pub fn program_entry() -> i8 {
    drop(logger::init());
    match client_run() {
        Ok(_) => 0,
        Err(e) => e as i8,
    }
}

// 生成大数据字符串
fn generate_large_data(size_mb: usize) -> String {
    let base_str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let chars_per_mb = 1024 * 1024; // 1MB的字符数
    let total_chars = size_mb * chars_per_mb;
    
    // 重复基础字符串来创建大数据
    let mut result = String::with_capacity(total_chars);
    let mut remaining_chars = total_chars;
    
    while remaining_chars > 0 {
        let chunk_size = if remaining_chars > base_str.len() {
            base_str.len()
        } else {
            remaining_chars
        };
        result.push_str(&base_str[..chunk_size]);
        remaining_chars = remaining_chars.saturating_sub(chunk_size);
    }
    
    result
}

pub fn client_run() -> Result<(), Error> {
    info!("client run started");

    // server can be spawned by any process which wants to start it.
    // here it is invoked by client
    let (read_pipe, write_pipe) = spawn_server(
        0,
        Source::CellDep,
        &[CString::new("demo").unwrap().as_ref()],
    )
        .map_err(|_| Error::CkbSysError)?;

    // new client
    let mut client = WorldClient::new(read_pipe, write_pipe);

    // 测试整数类型
    info!("测试整数类型");
    info!("i8: {:?}", client.test_i8(-8));
    info!("i16: {:?}", client.test_i16(-16));
    info!("i32: {:?}", client.test_i32(-32));
    info!("i64: {:?}", client.test_i64(-64));
    info!("i128: {:?}", client.test_i128(-128));
    
    info!("u8: {:?}", client.test_u8(8));
    info!("u16: {:?}", client.test_u16(16));
    info!("u32: {:?}", client.test_u32(32));
    info!("u64: {:?}", client.test_u64(64));
    info!("u128: {:?}", client.test_u128(128));

    // 测试浮点类型
    info!("测试浮点类型");
    info!("f32: {:?}", client.test_f32(3.14));
    info!("f64: {:?}", client.test_f64(6.28));

    // 测试布尔类型
    info!("测试布尔类型");
    info!("bool true: {:?}", client.test_bool(true));
    info!("bool false: {:?}", client.test_bool(false));

    // 测试字符类型
    info!("测试字符类型");
    info!("char: {:?}", client.test_char('你'));

    // // 测试字符串类型（原有功能）
    // let large_data = generate_large_data(1); // 使用1MB数据测试
    // info!("测试字符串类型，数据大小: {} bytes", large_data.len());
    // let ret = client.hello(large_data);
    // info!("字符串响应: {:?}", ret);

    Ok(())
}
