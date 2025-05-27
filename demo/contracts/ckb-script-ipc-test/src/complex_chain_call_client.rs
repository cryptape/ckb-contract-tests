#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

pub mod error;
use ckb_std::debug;

use crate::error::Error;
use alloc::ffi::CString;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use ckb_script_ipc_common::spawn::spawn_server;
#[cfg(not(test))]
use ckb_std::default_alloc;
use ckb_std::high_level::{load_cell_data, QueryIter};
use ckb_std::logger;
use ckb_std::syscalls::current_cycles;
use ckb_std::{ckb_constants::Source, log::info};

pub mod def;
use crate::def::{
    BoundaryStruct, TestBoundaryClient, TestLargeDataClient, TestLinkedCallClient, WorldClient,
};

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

pub fn client_run() -> Result<(), Error> {
    info!("client run started");

    // linked_server.rs
    let (read_pipe, write_pipe) = spawn_server(
        0,
        Source::CellDep,
        &[CString::new("demo").unwrap().as_ref()],
    )
    .map_err(|_| Error::CkbSysError)?;
    let mut linked_client = TestLinkedCallClient::new(read_pipe, write_pipe);

    // boundary_server.rs
    let (read_pipe, write_pipe) = spawn_server(
        1,
        Source::CellDep,
        &[CString::new("demo").unwrap().as_ref()],
    )
    .map_err(|_| Error::CkbSysError)?;
    // new client
    let mut boundary_client = TestBoundaryClient::new(read_pipe, write_pipe);

    // large_data_server.rs
    let (read_pipe, write_pipe) = spawn_server(
        2,
        Source::CellDep,
        &[CString::new("demo").unwrap().as_ref()],
    )
    .map_err(|_| Error::CkbSysError)?;

    let mut large_client = TestLargeDataClient::new(read_pipe, write_pipe);

    let data_length = collect_outputs_amount().unwrap();

    // server.rs
    let (read_pipe, write_pipe) = spawn_server(
        3,
        Source::CellDep,
        &[CString::new("demo").unwrap().as_ref()],
    )
    .map_err(|_| Error::CkbSysError)?;

    let (read_pipe1, write_pipe1) = spawn_server(
        3,
        Source::CellDep,
        &[CString::new("demo").unwrap().as_ref()],
    )
    .map_err(|_| Error::CkbSysError)?;
    // new client
    let mut client = WorldClient::new(read_pipe, write_pipe);
    let mut client2 = WorldClient::new(read_pipe1, write_pipe1);
    let ret = linked_client.test_linked_call_self(data_length as usize);
    // invoke
    let expected_ret = (1 + data_length) * data_length / 2;
    info!("IPC response: {:?}", ret);
    assert_eq!(ret, expected_ret as usize);
    for i in 0..100 {
        // invoke
        let ret = client
            .hello("world \0\n\\n\\\\\'''''   ///@@".into())
            .unwrap();
        info!("IPC response: {:?}", ret);
        let data = client.get_data();
        info!("data:{:?}", data);
        // invoke again, should return error
        let ret = client.hello("error".into());
        info!("IPC response: {:?}", ret);
        let data = client.get_data();
        info!("data:{:?}", data);
        let data = client2.get_data();
        info!("data:{:?}", data);

        let data_length = collect_outputs_amount().unwrap();
        let ret = boundary_client.test(vec![
            BoundaryStruct::min_value(),
            BoundaryStruct::max_value(),
        ]);
        info!("IPC response: {:?}", ret);
        assert_eq!(ret.len(), 2);

        let test_str = generate_rand_str(data_length as usize);
        let ret = large_client.test_large_data_handling(test_str.clone().into());
        info!("IPC response: {:?}", ret);
    }
    Ok(())
}
const UDT_LEN: usize = 16;

fn generate_rand_str(len: usize) -> String {
    let mut s = String::new();
    for _ in 0..len {
        s.push((current_cycles() as u8 % 26 + 97) as char);
    }
    s
}

fn collect_outputs_amount() -> Result<u128, u8> {
    // With the sum of all input UDT tokens gathered, let's now iterate through
    // output cells to grab the sum of all output UDT tokens.
    let mut buf = [0u8; UDT_LEN];
    debug!(
        "QueryIter:{:?}",
        QueryIter::new(load_cell_data, Source::GroupOutput).count()
    );
    let udt_list = QueryIter::new(load_cell_data, Source::GroupOutput)
        .map(|data| {
            if data.len() == UDT_LEN {
                buf.copy_from_slice(&data);
                // u128 is 16 bytes
                Ok(u128::from_le_bytes(buf))
            } else {
                Err(9)
            }
        })
        .collect::<Result<Vec<_>, u8>>()?;
    Ok(udt_list.into_iter().sum::<u128>())
}
