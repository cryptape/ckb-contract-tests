#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

pub mod def;
pub mod error;

use ckb_std::log::info;

use crate::def::World;
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

struct WorldServer {
    pub data: usize,
}

impl WorldServer {
    fn new() -> Self {
        WorldServer { data: 2 }
    }
}

impl World for WorldServer {
    // method implementation
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
