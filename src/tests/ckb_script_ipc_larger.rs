use ckb_testtool::ckb_types::core::TransactionBuilder;
use ckb_testtool::ckb_types::prelude::Builder;
use crate::cells::xudt_data::{XUDTData, XUDTDataCell};
use crate::ContractUtil;
use crate::prelude::ContextExt;

/// Test large data handling
///
/// max str length: 65439
#[test]
fn test_large_data_handling() {
    let mut input_token_cell = XUDTDataCell::new([1; 32], XUDTData { amount: 65439 });
    let mut ct = ContractUtil::new();
    let server_contract = ct.deploy_contract("../../demo/build/release/large_data_server");
    let client_contract = ct.deploy_contract("../../demo/build/release/large_data_client");

    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input(tx, ct.alway_contract.clone(), Some(client_contract.clone()), &input_token_cell, 100);
    tx = ct.add_outpoint(tx, ct.alway_contract.clone(), Some(client_contract.clone()), &input_token_cell, 100);
    tx = ct.add_contract_cell_dep(tx, &server_contract);
    tx = ct.context.complete_tx(tx);
    ct.context.should_be_passed(&tx, 100000000);

    let mut tx = TransactionBuilder::default().build();
    input_token_cell.data.amount = 65440;
    tx = ct.add_input(tx, ct.alway_contract.clone(), Some(client_contract.clone()), &input_token_cell, 100);
    tx = ct.add_outpoint(tx, ct.alway_contract.clone(), Some(client_contract.clone()), &input_token_cell, 100);
    tx = ct.add_contract_cell_dep(tx, &server_contract);
    tx = ct.context.complete_tx(tx);
    ct.context.should_be_failed(&tx, 100000000);
}


#[test]
fn test_large_data_handling_2() {
    let mut input_token_cell = XUDTDataCell::new([1; 32], XUDTData { amount: 24545 });
    let mut ct = ContractUtil::new();
    let server_contract = ct.deploy_contract("../../demo/build/release/large_data_server");
    let client_contract = ct.deploy_contract("../../demo/build/release/large_data_client2");

    // for i in 0..100 {
    //     input_token_cell.data.amount = input_token_cell.data.amount + i * 1;
    //     println!("input_token_cell.data.amount:{}", input_token_cell.data.amount);
    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input(tx, ct.alway_contract.clone(), Some(client_contract.clone()), &input_token_cell, 100);
    tx = ct.add_outpoint(tx, ct.alway_contract.clone(), Some(client_contract.clone()), &input_token_cell, 100);
    tx = ct.add_contract_cell_dep(tx, &server_contract);
    tx = ct.context.complete_tx(tx);
    ct.context.should_be_passed(&tx, 100000000);
    // }

    let mut tx = TransactionBuilder::default().build();
    input_token_cell.data.amount = 24551;
    tx = ct.add_input(tx, ct.alway_contract.clone(), Some(client_contract.clone()), &input_token_cell, 100);
    tx = ct.add_outpoint(tx, ct.alway_contract.clone(), Some(client_contract.clone()), &input_token_cell, 100);
    tx = ct.add_contract_cell_dep(tx, &server_contract);
    tx = ct.context.complete_tx(tx);
    ct.context.should_be_failed(&tx, 100000000);
}
