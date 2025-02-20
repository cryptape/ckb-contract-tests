use crate::cells::demo::Demo;
use crate::prelude::ContextExt;
use crate::ContractUtil;
use ckb_testtool::ckb_types::core::TransactionBuilder;
use syn::parse_str;

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
    tx = ct.add_contract_cell_dep(tx, &serve_contract);
    tx = ct.context.complete_tx(tx);
    let ret1 = ct.context.should_be_passed(&tx, 1000000);
    println!("ret:{:?}", ret1);
}
