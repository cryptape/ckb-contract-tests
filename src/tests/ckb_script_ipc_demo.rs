use ckb_testtool::ckb_types::core::TransactionBuilder;
use syn::parse_str;
use crate::cells::demo::Demo;
use crate::ContractUtil;
use crate::prelude::ContextExt;

#[test]
fn test_script_ipc_demo() {
    let input_token_cell = Demo::default();
    let mut ct = ContractUtil::new();
    let lock_contract = ct.deploy_contract("../../demo/build/release/ckb-script-ipc-demo");
    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input(tx, lock_contract.clone(), None, &input_token_cell, 100);
    tx = ct.context.complete_tx(tx);
    let ret1 = ct.context.should_be_passed(&tx, 1000000);
    println!("ret:{:?}", ret1);
}


#[test]
fn test_parse_method_with_return_type() {

    let input_token_cell = Demo::default();
    let mut ct = ContractUtil::new();
    let serve_contract = ct.deploy_contract("../../demo/build/release/ckb-script-ipc-test-serve");
    let client_contract = ct.deploy_contract("../../demo/build/release/ckb-script-ipc-test-client");

    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input(tx, client_contract.clone(), None, &input_token_cell, 100);
    tx = ct.add_contract_cell_dep(tx,&serve_contract);
    tx = ct.context.complete_tx(tx);
    let ret1 = ct.context.should_be_passed(&tx, 1000000);
    println!("ret:{:?}", ret1);
}


#[test]
fn test_invalid_request() {

    let input_token_cell = Demo::default();
    let mut ct = ContractUtil::new();
    let serve_contract = ct.deploy_contract("../../demo/build/release/test_invalid_request-serve");
    let client_contract = ct.deploy_contract("../../demo/build/release/test_invalid_request-client");

    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input(tx, client_contract.clone(), None, &input_token_cell, 100);
    tx = ct.add_contract_cell_dep(tx,&serve_contract);
    tx = ct.context.complete_tx(tx);
    let ret1 = ct.context.should_be_passed(&tx, 10000000);
    println!("ret:{:?}", ret1);
}


#[test]
fn test_forever_loop() {

    let input_token_cell = Demo::default();
    let mut ct = ContractUtil::new();
    let serve_contract = ct.deploy_contract("../../demo/build/release/test_loop_request-serve");
    let client_contract = ct.deploy_contract("../../demo/build/release/test_loop_request-client");

    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input(tx, client_contract.clone(), None, &input_token_cell, 100);
    tx = ct.add_contract_cell_dep(tx,&serve_contract);
    tx = ct.context.complete_tx(tx);
    let ret1 = ct.context.should_be_failed(&tx, 1000000000);
    println!("ret:{:?}", ret1);
}

#[test]
fn test_single_forever_loop() {

    let input_token_cell = Demo::default();
    let mut ct = ContractUtil::new();
    let serve_contract = ct.deploy_contract("../../demo/build/release/test_single_loop_request-serve");
    let client_contract = ct.deploy_contract("../../demo/build/release/test_single_loop_request-client");

    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input(tx, client_contract.clone(), None, &input_token_cell, 100);
    tx = ct.add_contract_cell_dep(tx,&serve_contract);
    tx = ct.context.complete_tx(tx);
    let ret1 = ct.context.should_be_failed(&tx, 1000000000);
    println!("ret:{:?}", ret1);
}

#[test]
fn test_GeneralIoError() {

    let input_token_cell = Demo::default();
    let mut ct = ContractUtil::new();
    let serve_contract = ct.deploy_contract("../../demo/build/release/test_single_loop_request-client");
    let client_contract = ct.deploy_contract("../../demo/build/release/test_single_loop_request-client");

    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input(tx, client_contract.clone(), None, &input_token_cell, 100);
    tx = ct.add_contract_cell_dep(tx,&serve_contract);
    tx = ct.context.complete_tx(tx);
    let ret1 = ct.context.should_be_failed(&tx, 1000000000);
    println!("ret:{:?}", ret1);
}

#[test]
fn test_large_data_handling() {

    let input_token_cell = Demo::default();
    let mut ct = ContractUtil::new();
    let serve_contract = ct.deploy_contract("../../demo/build/release/test_large_data_request-serve");
    let client_contract = ct.deploy_contract("../../demo/build/release/test_large_data_request-client");

    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input(tx, client_contract.clone(), None, &input_token_cell, 100);
    tx = ct.add_contract_cell_dep(tx,&serve_contract);
    tx = ct.context.complete_tx(tx);
    let ret1 = ct.context.should_be_failed(&tx, 1000000000);
    println!("ret:{:?}", ret1);
}