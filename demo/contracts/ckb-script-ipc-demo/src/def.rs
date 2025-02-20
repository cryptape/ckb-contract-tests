use alloc::string::String;

// IPC definition, it can be shared between client and server
#[ckb_script_ipc::service]
pub trait World {
    fn hello(name: String) -> Result<String, u64>;
}

#[ckb_script_ipc::service]
pub trait World11 {
    // 1. 无参数、无返回值
    fn foo();
    // fn greet(name: &str);

    fn hello(name: String) -> Result<String, u64>;
}
