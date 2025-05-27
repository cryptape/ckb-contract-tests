#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

use ckb_std::{debug, syscalls};
use ckb_std::high_level::{load_cell_data, QueryIter};


use core::u64;

use alloc::vec::Vec;
use ckb_std::ckb_constants::{Source, SYS_EXEC};
#[cfg(not(test))]
use ckb_std::default_alloc;

#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();

fn exec_test_limit() {
    if syscalls::current_cycles() > 99000 {
        return;
    }
    let mb = [0x66 as i8; 1024 * 1024];
    let kb = [0x66 as i8; 1024];
    let bt = [0x66 as i8; 1];

    // s[3145727] = 0;
    unsafe {
        // let size = collect_outputs_amount();
        // let argv = [s.as_ptr() as *const i8; (1024*512)/8];
        // 读指针的时候每次加 8 会越界的
        //
        let (mb_size, kb_size, byte_size) = get_data(collect_outputs_amount().unwrap() as usize);
        let mut argv: Vec<*const i8> = Vec::with_capacity(mb_size as usize);
        for _ in 0..mb_size {
            argv.push(mb.as_ptr());
        }
        for _ in 0..kb_size {
            argv.push(kb.as_ptr());
        }
        for _ in 0..byte_size {
            argv.push(bt.as_ptr());
        }
        let argc = argv.len();
        syscall(
            0,
            3,
            0,
            0,
            argc as u64,
            argv.as_ptr() as u64,
            0,
            SYS_EXEC,
        );
    }
}

pub fn get_data(u128: usize) -> (usize, usize, usize) {
    debug!("u128:{}",u128);
    debug!("mb:{},kb:{},byte:{}",u128 / 100000000, u128 % 100000000 / 10000,u128 % 10000);
    return (u128 / 100000000, u128 % 100000000 / 10000, u128 % 10000);
}
pub fn program_entry() -> i8 {
    exec_test_limit();
    0
}

const UDT_LEN: usize = 16;

fn collect_outputs_amount() -> Result<u128, u8> {
    // With the sum of all input UDT tokens gathered, let's now iterate through
    // output cells to grab the sum of all output UDT tokens.
    let mut buf = [0u8; UDT_LEN];
    debug!("QueryIter:{:?}",QueryIter::new(load_cell_data, Source::GroupOutput).count());
    let udt_list = QueryIter::new(load_cell_data, Source::GroupOutput)
        .map(|data| {
            if data.len() == UDT_LEN {
                buf.copy_from_slice(&data);
                // u128 is 16 bytes
                Ok(u128::from_le_bytes(buf))
            } else {
                Err(9)
            }
        }).collect::<Result<Vec<_>, u8>>()?;
    Ok(udt_list.into_iter().sum::<u128>())
}


#[cfg(target_arch = "riscv64")]
use core::arch::asm;

// #[cfg(target_arch = "riscv64")]
unsafe fn syscall(
    mut a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
) -> u64 {
    asm!(
    "ecall",
    inout("a0") a0,
    in("a1") a1,
    in("a2") a2,
    in("a3") a3,
    in("a4") a4,
    in("a5") a5,
    in("a6") a6,
    in("a7") a7
    );
    a0
}