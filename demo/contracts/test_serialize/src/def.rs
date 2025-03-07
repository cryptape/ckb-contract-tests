use alloc::string::String;

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
    
    // 原有方法保持不变
    fn hello(name: String) -> Result<String, u64>;
    fn get_data() -> usize;
}

