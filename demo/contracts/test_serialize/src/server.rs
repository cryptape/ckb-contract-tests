#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

pub mod def;
pub mod error;

use ckb_std::{log::info};

use crate::def::World;
use alloc::{format, string::String};
use ckb_script_ipc_common::spawn::run_server;

#[cfg(not(test))]
use ckb_std::default_alloc;
use ckb_std::logger;
use crate::error::Error;

#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();

pub fn program_entry() -> i8 {
    drop(logger::init());
    match server_run() {
        Ok(_) => 0,
        Err(e) => e as i8,
    }
}

struct WorldServer {
    pub data: usize,
}

impl WorldServer {
    fn new() -> Self {
        WorldServer {
            data: 2
        }
    }
}

impl World for WorldServer {
    // 整数类型测试实现
    fn test_i8(&mut self, val: i8) -> Result<i8, u64> { Ok(val) }
    fn test_i16(&mut self, val: i16) -> Result<i16, u64> { Ok(val) }
    fn test_i32(&mut self, val: i32) -> Result<i32, u64> { Ok(val) }
    fn test_i64(&mut self, val: i64) -> Result<i64, u64> { Ok(val) }
    fn test_i128(&mut self, val: i128) -> Result<i128, u64> { Ok(val) }
    
    fn test_u8(&mut self, val: u8) -> Result<u8, u64> { Ok(val) }
    fn test_u16(&mut self, val: u16) -> Result<u16, u64> { Ok(val) }
    fn test_u32(&mut self, val: u32) -> Result<u32, u64> { Ok(val) }
    fn test_u64(&mut self, val: u64) -> Result<u64, u64> { Ok(val) }
    fn test_u128(&mut self, val: u128) -> Result<u128, u64> { Ok(val) }
    
    // 浮点类型测试实现
    fn test_f32(&mut self, val: f32) -> Result<f32, u64> { Ok(val) }
    fn test_f64(&mut self, val: f64) -> Result<f64, u64> { Ok(val) }
    
    // 布尔类型测试实现
    fn test_bool(&mut self, val: bool) -> Result<bool, u64> { Ok(val) }
    
    // 字符类型测试实现
    fn test_char(&mut self, val: char) -> Result<char, u64> { Ok(val) }
    
    // 原有方法实现
    fn hello(&mut self, name: String) -> Result<String, u64> {
        self.data += 1;
        if name == "error" {
            Err(1)
        } else if name == "server_error" {
            Err(Error::ServerError as u64)
        } else if name == "ckb_sys_error" {
            Err(Error::CkbSysError as u64)
        } else if name == "unknown" {
            Err(Error::Unknown as u64)
        } else {
            Ok(format!("处理成功，数据大小: {} 字节", name.len()))
        }
    }

    fn get_data(&mut self) -> usize {
        return self.data;
    }
}

pub fn server_run() -> Result<(), Error> {
    let world = WorldServer::new();
    run_server(world.server()).map_err(|_| Error::ServerError)
}