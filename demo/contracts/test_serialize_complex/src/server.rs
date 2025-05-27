#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

pub mod def;
pub mod error;

use ckb_std::{log::info};
use crate::def::{World, Point, Color};
use alloc::{format, string::String, vec::Vec, collections::BTreeMap, rc::Rc, sync::Arc, boxed::Box};
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
    
    fn test_f32(&mut self, val: f32) -> Result<f32, u64> { Ok(val) }
    fn test_f64(&mut self, val: f64) -> Result<f64, u64> { Ok(val) }
    
    fn test_bool(&mut self, val: bool) -> Result<bool, u64> { Ok(val) }
    
    fn test_char(&mut self, val: char) -> Result<char, u64> { Ok(val) }
    
    fn test_struct(&mut self, point: Point) -> Result<Point, u64> { Ok(point) }
    
    fn test_enum(&mut self, color: Color) -> Result<Color, u64> { Ok(color) }
    
    fn test_tuple(&mut self, data: (i32, String, bool)) -> Result<(i32, String, bool), u64> { Ok(data) }
    
    fn test_array(&mut self, data: [i32; 5]) -> Result<[i32; 5], u64> { Ok(data) }
    
    fn test_vec(&mut self, data: Vec<i32>) -> Result<Vec<i32>, u64> { Ok(data) }
    
    fn test_option(&mut self, data: Option<String>) -> Result<Option<String>, u64> { Ok(data) }
    
    fn test_nested(&mut self, data: Vec<Option<Point>>) -> Result<Vec<Option<Point>>, u64> { Ok(data) }
    
    fn test_map(&mut self, data: BTreeMap<String, i32>) -> Result<BTreeMap<String, i32>, u64> { Ok(data) }
    
    fn test_box(&mut self, data: Box<i32>) -> Result<Box<i32>, u64> { Ok(data) }
    fn test_box_struct(&mut self, data: Box<Point>) -> Result<Box<Point>, u64> { Ok(data) }
    
    fn test_rc(&mut self, data: Rc<String>) -> Result<Rc<String>, u64> { Ok(data) }
    
    fn test_arc(&mut self, data: Arc<Vec<i32>>) -> Result<Arc<Vec<i32>>, u64> { Ok(data) }
    
    fn hello(&mut self, name: String) -> Result<String, u64> {
        self.data += 1;
        if name == "error" {
            Err(1)
        } else {
            Ok(format!("hello, {}", name))
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