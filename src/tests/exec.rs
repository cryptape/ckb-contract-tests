use crate::ContractUtil;
use ckb_testtool::ckb_types::core::TransactionBuilder;
use crate::cells::demo::Demo;
use crate::cells::xudt_data::{XUDTData, XUDTDataCell};
use crate::prelude::ContextExt;

#[test]
fn test_exec_arg_length() {
    let mut ct = ContractUtil::new();
    let type_contract = ct.deploy_contract("../../demo/build/release/exec-arg-length");
    let mut tx = TransactionBuilder::default().build();
    let input_token_cell = XUDTDataCell::new([1; 32], XUDTData { amount: 110240000 });
    tx = ct.add_input(tx, ct.alway_contract.clone(), Some(type_contract.clone()), &input_token_cell, 100);
    tx = ct.add_outpoint(tx, ct.alway_contract.clone(), Some(type_contract.clone()), &input_token_cell, 100);
    tx = ct.context.complete_tx(tx);
    let ret1 = ct.context.should_be_failed(&tx, 1000000);
    println!("ret:{:?}", ret1);
}