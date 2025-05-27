use alloc::string::{String, ToString};
use alloc::vec::Vec;
use ckb_std::ckb_types::bytes::Bytes;
use core::f64;
use serde::{Deserialize, Serialize};

// IPC definition, it can be shared between client and server
#[ckb_script_ipc::service]
pub trait World {
    fn hello(name: String) -> Result<String, u64>;
    fn get_data() -> usize;
}

#[ckb_script_ipc::service]
pub trait TestLargeData {
    fn test_large_data_handling(data: String) -> String;
    fn test_large_data_handling2(data: Vec<u8>) -> Vec<u8>;
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct BoundaryStruct {
    pub usize_data: usize,
    pub u128_data: u128,
    pub u64_data: u64,
    pub u32_data: u32,
    pub u16_data: u16,
    pub u8_data: u8,
    pub isize_data: isize,
    pub i128_data: i128,
    pub i64_data: i64,
    pub i32_data: i32,
    pub i16_data: i16,
    pub i8_data: i8,
    pub bool_data: bool,
    pub char_data: char,
    pub f32_data: f32,
    pub f64_data: f64,
    pub str_data: String,
}

impl BoundaryStruct {
    fn new() -> Self {
        BoundaryStruct {
            usize_data: 0,
            u128_data: 0,
            u64_data: 0,
            u32_data: 0,
            u16_data: 0,
            u8_data: 0,
            isize_data: 0,
            i128_data: 0,
            i64_data: 0,
            i32_data: 0,
            i16_data: 0,
            i8_data: 0,
            bool_data: false,
            char_data: 'a',
            f32_data: 0.0,
            f64_data: 0.0,
            str_data: "".to_string(),
        }
    }

    pub fn min_value() -> Self {
        BoundaryStruct {
            usize_data: usize::MIN,
            u128_data: u128::MIN,
            u64_data: u64::MIN,
            u32_data: u32::MIN,
            u16_data: u16::MIN,
            u8_data: u8::MIN,
            isize_data: isize::MIN,
            i128_data: i128::MIN,
            i64_data: i64::MIN,
            i32_data: i32::MIN,
            i16_data: i16::MIN,
            i8_data: i8::MIN,
            bool_data: false,
            char_data: ' ',
            f32_data: f32::MIN,
            f64_data: f64::MIN,
            str_data: "".to_string(),
        }
    }

    pub fn max_value() -> Self {
        BoundaryStruct {
            usize_data: usize::MAX,
            u128_data: u128::MAX,
            u64_data: u64::MAX,
            u32_data: u32::MAX,
            u16_data: u16::MAX,
            u8_data: u8::MAX,
            isize_data: 0,
            i128_data: i128::MAX,
            i64_data: i64::MAX,
            i32_data: i32::MAX,
            i16_data: i16::MAX,
            i8_data: i8::MAX,
            bool_data: true,
            char_data: '0',
            f32_data: f32::MAX,
            f64_data: f64::MAX,
            str_data: "max".to_string(),
        }
    }
}

#[ckb_script_ipc::service]
pub trait TestBoundary {
    fn test(vec: Vec<BoundaryStruct>) -> Vec<BoundaryStruct>;
}

#[ckb_script_ipc::service]
pub trait TestLinkedCall {
    fn test_linked_call() -> usize;
    fn test_linked_call_self(count: usize) -> usize;

    fn get_data() -> usize;
}
