#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

pub mod def;
pub mod error;

use alloc::ffi::CString;
use ckb_script_ipc_common::spawn::spawn_server;
use ckb_std::{ckb_constants::Source, log::info, syscalls};

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

// 简单的伪随机数生成器
struct FuzzGenerator {
    seed: u64,
}

impl FuzzGenerator {
    fn new() -> Self {
        // 使用当前周期作为随机种子
        let seed = syscalls::current_cycles() as u64;
        Self { seed }
    }

    // 生成下一个随机数
    fn next(&mut self) -> u64 {
        // 简单的线性同余生成器
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.seed
    }

    // 生成指定范围内的随机数
    fn range(&mut self, min: u64, max: u64) -> u64 {
        min + (self.next() % (max - min + 1))
    }

    // 生成随机i8
    fn gen_i8(&mut self) -> i8 {
        self.next() as i8
    }

    // 生成随机i16
    fn gen_i16(&mut self) -> i16 {
        self.next() as i16
    }

    // 生成随机i32
    fn gen_i32(&mut self) -> i32 {
        self.next() as i32
    }

    // 生成随机i64
    fn gen_i64(&mut self) -> i64 {
        self.next() as i64
    }

    // 生成随机i128
    fn gen_i128(&mut self) -> i128 {
        (self.next() as i128) | ((self.next() as i128) << 64)
    }

    // 生成随机u8
    fn gen_u8(&mut self) -> u8 {
        self.next() as u8
    }

    // 生成随机u16
    fn gen_u16(&mut self) -> u16 {
        self.next() as u16
    }

    // 生成随机u32
    fn gen_u32(&mut self) -> u32 {
        self.next() as u32
    }

    // 生成随机u64
    fn gen_u64(&mut self) -> u64 {
        self.next()
    }

    // 生成随机u128
    fn gen_u128(&mut self) -> u128 {
        (self.next() as u128) | ((self.next() as u128) << 64)
    }

    // 生成随机f32
    fn gen_f32(&mut self) -> f32 {
        let bits = self.next() as u32;
        f32::from_bits(bits)
    }

    // 生成随机f64
    fn gen_f64(&mut self) -> f64 {
        let bits = self.next();
        f64::from_bits(bits)
    }

    // 生成随机bool
    fn gen_bool(&mut self) -> bool {
        (self.next() & 1) == 1
    }

    // 生成随机char
    fn gen_char(&mut self) -> char {
        // 生成有效的Unicode字符（基本多文种平面）
        let code = self.range(32, 0xD7FF) as u32;
        char::from_u32(code).unwrap_or('?')
    }

    // 生成随机字符串
    fn gen_string(&mut self, max_len: usize) -> String {
        let len = self.range(1, max_len as u64) as usize;
        let mut result = String::with_capacity(len);
        for _ in 0..len {
            result.push(self.gen_char());
        }
        result
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

    // 创建Fuzz生成器
    let mut fuzz_gen = FuzzGenerator::new();
    
    // 执行标准测试
    run_standard_tests(&mut client)?;
    
    // 执行Fuzz测试
    run_fuzz_tests(&mut client, &mut fuzz_gen, 10)?; // 执行10轮Fuzz测试

    Ok(())
}

// 标准测试函数
fn run_standard_tests<R: ckb_script_ipc_common::io::Read, W: ckb_script_ipc_common::io::Write>(client: &mut WorldClient<R, W>) -> Result<(), Error> {
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

// Fuzz测试函数
fn run_fuzz_tests<R: ckb_script_ipc_common::io::Read, W: ckb_script_ipc_common::io::Write>(client: &mut WorldClient<R, W>, fuzz_gen: &mut FuzzGenerator, rounds: usize) -> Result<(), Error> {
    info!("开始Fuzz测试，执行{}轮", rounds);
    
    for i in 0..rounds {
        info!("Fuzz测试轮次 {}/{}", i+1, rounds);
        
        // 测试整数类型
        let i8_val = fuzz_gen.gen_i8();
        info!("Fuzz i8({}): {:?}", i8_val, client.test_i8(i8_val));
        
        let i16_val = fuzz_gen.gen_i16();
        info!("Fuzz i16({}): {:?}", i16_val, client.test_i16(i16_val));
        
        let i32_val = fuzz_gen.gen_i32();
        info!("Fuzz i32({}): {:?}", i32_val, client.test_i32(i32_val));
        
        let i64_val = fuzz_gen.gen_i64();
        info!("Fuzz i64({}): {:?}", i64_val, client.test_i64(i64_val));
        
        let i128_val = fuzz_gen.gen_i128();
        info!("Fuzz i128({}): {:?}", i128_val, client.test_i128(i128_val));
        
        let u8_val = fuzz_gen.gen_u8();
        info!("Fuzz u8({}): {:?}", u8_val, client.test_u8(u8_val));
        
        let u16_val = fuzz_gen.gen_u16();
        info!("Fuzz u16({}): {:?}", u16_val, client.test_u16(u16_val));
        
        let u32_val = fuzz_gen.gen_u32();
        info!("Fuzz u32({}): {:?}", u32_val, client.test_u32(u32_val));
        
        let u64_val = fuzz_gen.gen_u64();
        info!("Fuzz u64({}): {:?}", u64_val, client.test_u64(u64_val));
        
        let u128_val = fuzz_gen.gen_u128();
        info!("Fuzz u128({}): {:?}", u128_val, client.test_u128(u128_val));
        
        // 测试浮点类型
        let f32_val = fuzz_gen.gen_f32();
        // 过滤掉NaN和无穷大的情况
        if !f32_val.is_nan() && f32_val.is_finite() {
            info!("Fuzz f32({}): {:?}", f32_val, client.test_f32(f32_val));
        }
        
        let f64_val = fuzz_gen.gen_f64();
        // 过滤掉NaN和无穷大的情况
        if !f64_val.is_nan() && f64_val.is_finite() {
            info!("Fuzz f64({}): {:?}", f64_val, client.test_f64(f64_val));
        }
        
        // 测试布尔类型
        let bool_val = fuzz_gen.gen_bool();
        info!("Fuzz bool({}): {:?}", bool_val, client.test_bool(bool_val));
        
        // 测试字符类型
        let char_val = fuzz_gen.gen_char();
        info!("Fuzz char({}): {:?}", char_val, client.test_char(char_val));
        
        // 测试字符串类型
        let string_val = fuzz_gen.gen_string(100); // 最大100个字符
        info!("Fuzz string({}): {:?}", string_val.clone(), client.hello(string_val));
    }
    
    info!("Fuzz测试完成");
    Ok(())
}
