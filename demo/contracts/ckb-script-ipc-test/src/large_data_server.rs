#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

pub mod def;
pub mod error;

use alloc::vec::Vec;
use ckb_std::log::info;

use crate::def::TestLargeData;
use alloc::{format, string::String};
use ckb_script_ipc_common::spawn::run_server;

use crate::error::Error;
#[cfg(not(test))]
use ckb_std::default_alloc;
use ckb_std::logger;

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

pub struct LargeDataServer {
    pub data: usize,
}

impl LargeDataServer {
    fn new() -> Self {
        LargeDataServer { data: 2 }
    }
}

impl TestLargeData for LargeDataServer {
    // method implementation
    fn test_large_data_handling(&mut self, data: String) -> String {
        self.data += 1;
        return format!("hello, {}", data);
    }
    fn test_large_data_handling2(&mut self, data: Vec<u8>) -> Vec<u8> {
        return data;
    }
}

// impl World for LargeDataServer {
//     // method implementation
//     fn hello(&mut self, name: String) -> Result<String, u64> {
//         self.data += 1;
//         if name == "error" {
//             Err(1)
//         } else {
//             Ok(format!("hello, {}", name))
//         }
//     }
//     fn get_data(&mut self) -> usize {
//         return self.data;
//     }
// }

pub fn server_run() -> Result<(), Error> {
    let world = LargeDataServer::new();
    run_server(world.server()).map_err(|_| Error::ServerError)
}
