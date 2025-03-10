#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

pub mod def;
pub mod error;

use ckb_std::{debug, syscalls};

use crate::def::{TestLinkedCall, TestLinkedCallClient};
use crate::error::Error;
use alloc::ffi::CString;
use ckb_script_ipc_common::pipe::Pipe;
use ckb_script_ipc_common::spawn::{run_server, spawn_server};
use ckb_std::ckb_constants::Source;
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

pub struct LinkedCallServer {
    pub data: u8,
}

impl LinkedCallServer {
    fn new() -> Self {
        debug!("LinkedCallServer new,{}", syscalls::process_id());
        if syscalls::process_id() > 1 {
            return LinkedCallServer {
                data: syscalls::process_id() as u8,
            };
        }
        let (read_pipe, write_pipe) = match spawn_server(
            0,
            Source::CellDep,
            &[CString::new("demo").unwrap().as_ref()],
        )
        .map_err(|_| Error::CkbSysError)
        {
            Ok(ret) => ret,
            Err(_) => {
                debug!("{}:spawn server failed", syscalls::process_id());
                panic!("spawn server failed");
            }
        };

        // new client
        let mut client = TestLinkedCallClient::new(read_pipe, write_pipe);
        let ret = client.get_data();
        return LinkedCallServer { data: ret as u8 };
    }
}

impl TestLinkedCall for LinkedCallServer {
    // method implementation

    fn test_linked_call(&mut self) -> usize {
        return 0;
    }
    fn test_linked_call_self(&mut self, count: usize) -> usize {
        debug!("LinkedCallServer new,{}", syscalls::process_id());
        if count == 0 {
            return 0;
        }
        let (read_pipe, write_pipe) = match spawn_server(
            0,
            Source::CellDep,
            &[CString::new("demo").unwrap().as_ref()],
        )
        .map_err(|_| Error::CkbSysError)
        {
            Ok(ret) => ret,
            Err(_) => {
                debug!("{}:spawn server failed", syscalls::process_id());
                panic!("spawn server failed");
            }
        };
        // new client
        let mut client = TestLinkedCallClient::new(read_pipe, write_pipe);
        return count + client.test_linked_call_self(count - 1);
    }

    fn get_data(&mut self) -> usize {
        return syscalls::process_id() as usize;
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
    let world = LinkedCallServer::new();
    run_server(world.server()).map_err(|_| Error::ServerError)
}
