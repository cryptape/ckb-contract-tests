use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::sync::Arc;
use alloc::boxed::Box;
use serde_with::serde_as;

// 测试用的复杂类型定义
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

// IPC definition, it can be shared between client and server
#[ckb_script_ipc::service]
pub trait World {
    // 测试整数类型
    fn test_i8(val: i8) -> Result<i8, u64>;
    fn test_i16(val: i16) -> Result<i16, u64>;
    fn test_i32(val: i32) -> Result<i32, u64>;
    fn test_i64(val: i64) -> Result<i64, u64>;
    fn test_i128(val: i128) -> Result<i128, u64>;
    
    fn test_u8(val: u8) -> Result<u8, u64>;
    fn test_u16(val: u16) -> Result<u16, u64>;
    fn test_u32(val: u32) -> Result<u32, u64>;
    fn test_u64(val: u64) -> Result<u64, u64>;
    fn test_u128(val: u128) -> Result<u128, u64>;
    
    // 测试浮点类型
    fn test_f32(val: f32) -> Result<f32, u64>;
    fn test_f64(val: f64) -> Result<f64, u64>;
    
    // 测试布尔类型
    fn test_bool(val: bool) -> Result<bool, u64>;
    
    // 测试字符类型
    fn test_char(val: char) -> Result<char, u64>;
    
    // 测试结构体
    fn test_struct(point: Point) -> Result<Point, u64>;
    
    // 测试枚举
    fn test_enum(color: Color) -> Result<Color, u64>;
    
    // 测试元组
    fn test_tuple(data: (i32, String, bool)) -> Result<(i32, String, bool), u64>;
    
    // 测试固定大小数组
    fn test_array(data: [i32; 5]) -> Result<[i32; 5], u64>;
    
    // 测试向量
    fn test_vec(data: Vec<i32>) -> Result<Vec<i32>, u64>;
    
    // 测试Option
    fn test_option(data: Option<String>) -> Result<Option<String>, u64>;
    
    // 测试嵌套结构
    fn test_nested(data: Vec<Option<Point>>) -> Result<Vec<Option<Point>>, u64>;
    
    // 测试映射类型
    fn test_map(data: BTreeMap<String, i32>) -> Result<BTreeMap<String, i32>, u64>;
    
    // 测试Box智能指针
    fn test_box(data: Box<i32>) -> Result<Box<i32>, u64>;
    fn test_box_struct(data: Box<Point>) -> Result<Box<Point>, u64>;
    
    // 测试Rc引用计数智能指针
    fn test_rc(data: Rc<String>) -> Result<Rc<String>, u64>;
    
    // 测试Arc原子引用计数智能指针
    fn test_arc(data: Arc<Vec<i32>>) -> Result<Arc<Vec<i32>>, u64>;
    
    // 原有方法保持不变
    fn hello(name: String) -> Result<String, u64>;
    fn get_data() -> usize;
}

