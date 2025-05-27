use std::cmp::{max, min};
use std::io;
use std::io::{copy, BufRead, BufReader, BufWriter, Read, Write};
use ckb_testtool::ckb_types::core::TransactionBuilder;
use crate::cells::demo::Demo;
use crate::ContractUtil;
use crate::prelude::ContextExt;

#[test]
fn test_buffered() {
    let input_token_cell = Demo::new();
    let mut ct = ContractUtil::new();
    let type_contract = ct.deploy_contract("../../demo/build/release/buffered");
    let mut tx = TransactionBuilder::default().build();

    tx = ct.add_input(tx, type_contract.clone(),None, &input_token_cell, 100);
    tx = ct.context.complete_tx(tx);
    let ret1 = ct.context.should_be_passed(&tx, 1000000);
    println!("ret:{:?}", ret1);

}

#[test]
fn test_copy(){
    let input_token_cell = Demo::new();
    let mut ct = ContractUtil::new();
    let type_contract = ct.deploy_contract("../../demo/build/release/copy");
    let mut tx = TransactionBuilder::default().build();

    tx = ct.add_input(tx, type_contract.clone(),None, &input_token_cell, 100);
    tx = ct.context.complete_tx(tx);
    let ret1 = ct.context.should_be_passed(&tx, 10000000);
    println!("ret:{:?}", ret1);

}

#[test]
fn test_cursor(){
    let input_token_cell = Demo::new();
    let mut ct = ContractUtil::new();
    let type_contract = ct.deploy_contract("../../demo/build/release/cursor");
    let mut tx = TransactionBuilder::default().build();

    tx = ct.add_input(tx, type_contract.clone(),None, &input_token_cell, 100);
    tx = ct.context.complete_tx(tx);
    let ret1 = ct.context.should_be_passed(&tx, 1000000);
    println!("ret:{:?}", ret1);
    
}

#[test]
fn test_io_all(){
    let input_token_cell = Demo::new();
    let mut ct = ContractUtil::new();
    let type_contract = ct.deploy_contract("../../demo/build/release/io_all");
    let mut tx = TransactionBuilder::default().build();

    tx = ct.add_input(tx, type_contract.clone(),None, &input_token_cell, 100);
    tx = ct.context.complete_tx(tx);
    let ret1 = ct.context.should_be_passed(&tx, 1000000);
    println!("ret:{:?}", ret1);

}

#[test]
fn test_impls(){
    let input_token_cell = Demo::new();
    let mut ct = ContractUtil::new();
    let type_contract = ct.deploy_contract("../../demo/build/release/impls");
    let mut tx = TransactionBuilder::default().build();

    tx = ct.add_input(tx, type_contract.clone(),None, &input_token_cell, 100);
    tx = ct.context.complete_tx(tx);
    let ret1 = ct.context.should_be_passed(&tx, 1000000);
    println!("ret:{:?}", ret1);
}

#[test]
fn test_util(){
    let input_token_cell = Demo::new();
    let mut ct = ContractUtil::new();
    let type_contract = ct.deploy_contract("../../demo/build/release/util");
    let mut tx = TransactionBuilder::default().build();

    tx = ct.add_input(tx, type_contract.clone(),None, &input_token_cell, 100);
    tx = ct.context.complete_tx(tx);
    let ret1 = ct.context.should_be_passed(&tx, 1000000);
    println!("ret:{:?}", ret1);
}

