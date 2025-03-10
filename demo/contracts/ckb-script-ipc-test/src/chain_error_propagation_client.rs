#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;
use crate::def::WorldClient;

pub mod def;
pub mod error;

use alloc::ffi::CString;
use ckb_script_ipc_common::spawn::spawn_server;
use ckb_std::{ckb_constants::Source, debug, log::info};

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
    match client_run() {
        Ok(_) => 0,
        Err(e) => e as i8,
    }
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
    info!("call hello");

    // invoke
    // let ret = client.hello("world \0\n\\n\\\\\'''''   ///@@".into()).unwrap();
    let ret = match client.hello("world".into()) {
        Ok(ok) => ok,
        Err(err) => {
            debug!("error: {:?}", err);
            return Err(Error::CkbSysError);
        }
    };

    Ok(())
}
