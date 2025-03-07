#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

pub mod def;
pub mod error;

use alloc::ffi::CString;
use alloc::vec;
use ckb_script_ipc_common::spawn::spawn_server;
use ckb_std::{ckb_constants::Source, log::info};

#[cfg(not(test))]
use ckb_std::default_alloc;
use ckb_std::logger;
use crate::def::{WorldClient, Point, Color};
use crate::error::Error;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::sync::Arc;
use alloc::boxed::Box;

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

    // 测试结构体
    info!("测试结构体");
    let point = Point { x: 10, y: 20 };
    info!("struct Point: {:?}", client.test_struct(point));

    // 测试枚举
    info!("测试枚举");
    info!("enum Color::Red: {:?}", client.test_enum(Color::Red));
    info!("enum Color::Custom: {:?}", client.test_enum(Color::Custom(255, 128, 0)));

    // 测试元组
    info!("测试元组");
    let tuple_data = (42, String::from("test"), true);
    info!("tuple: {:?}", client.test_tuple(tuple_data));

    // 测试固定大小数组
    info!("测试固定大小数组");
    let array_data = [1, 2, 3, 4, 5];
    info!("array: {:?}", client.test_array(array_data));

    // 测试向量
    info!("测试向量");
    let vec_data = vec![1, 2, 3, 4, 5];
    info!("vector: {:?}", client.test_vec(vec_data));

    // 测试Option
    info!("测试Option");
    info!("Some: {:?}", client.test_option(Some(String::from("test"))));
    info!("None: {:?}", client.test_option(None));

    // 测试嵌套结构
    info!("测试嵌套结构");
    let nested_data = vec![Some(Point { x: 1, y: 2 }), None, Some(Point { x: 3, y: 4 })];
    info!("nested: {:?}", client.test_nested(nested_data));

    // 测试映射类型
    info!("测试映射类型");
    let mut map_data = BTreeMap::new();
    map_data.insert(String::from("key1"), 1);
    map_data.insert(String::from("key2"), 2);
    info!("map: {:?}", client.test_map(map_data));

    // 测试Box智能指针
    info!("测试Box智能指针");
    let boxed_int = Box::new(42);
    info!("boxed int: {:?}", client.test_box(boxed_int));
    let boxed_point = Box::new(Point { x: 100, y: 200 });
    info!("boxed point: {:?}", client.test_box_struct(boxed_point));

    // 测试Rc引用计数智能指针
    info!("测试Rc引用计数智能指针");
    let rc_string = Rc::new(String::from("test rc"));
    info!("rc string: {:?}", client.test_rc(rc_string));

    // 测试Arc原子引用计数智能指针
    info!("测试Arc原子引用计数智能指针");
    let arc_vec = Arc::new(vec![1, 2, 3, 4, 5]);
    info!("arc vector: {:?}", client.test_arc(arc_vec));

    // 测试原有功能
    info!("测试原有功能");
    info!("hello: {:?}", client.hello(String::from("world")));
    info!("get_data: {}", client.get_data());

    Ok(())
}
