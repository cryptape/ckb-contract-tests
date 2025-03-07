use ckb_testtool::ckb_types::core::TransactionBuilder;
use ckb_testtool::ckb_types::prelude::Builder;
use crate::cells::xudt_data::{XUDTData, XUDTDataCell};
use crate::ContractUtil;
use crate::prelude::ContextExt;


#[test]
fn test_basic_chain_call() {
    let mut input_token_cell = XUDTDataCell::new([1; 32], XUDTData { amount: 13 });
    let mut ct = ContractUtil::new();
    let server_contract = ct.deploy_contract("../../demo/build/release/linked_server");
    let client_contract = ct.deploy_contract("../../demo/build/release/linked_client");

    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input(tx, ct.alway_contract.clone(), Some(client_contract.clone()), &input_token_cell, 100);
    tx = ct.add_outpoint(tx, ct.alway_contract.clone(), Some(client_contract.clone()), &input_token_cell, 100);
    tx = ct.add_contract_cell_dep(tx, &server_contract);
    tx = ct.context.complete_tx(tx);
    ct.context.should_be_passed(&tx, 100000000);

    input_token_cell.data.amount = 14;
    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input(tx, ct.alway_contract.clone(), Some(client_contract.clone()), &input_token_cell, 100);
    tx = ct.add_outpoint(tx, ct.alway_contract.clone(), Some(client_contract.clone()), &input_token_cell, 100);
    tx = ct.add_contract_cell_dep(tx, &server_contract);
    tx = ct.context.complete_tx(tx);
    ct.context.should_be_failed(&tx, 100000000);
}


#[test]
fn test_complex_chain_call() {
    // linked_server.rs
    let mut input_token_cell = XUDTDataCell::new([1; 32], XUDTData { amount: 1 });
    let mut ct = ContractUtil::new();
    let linked_server_contract = ct.deploy_contract("../../demo/build/release/linked_server");
    let boundary_server_contract = ct.deploy_contract("../../demo/build/release/boundary_server");
    let large_data_server_contract = ct.deploy_contract("../../demo/build/release/large_data_server");
    let server_contract = ct.deploy_contract("../../demo/build/release/ckb-script-ipc-test-serve");
    let complex_chain_call_contract = ct.deploy_contract("../../demo/build/release/complex_chain_call_client");

    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input(tx, ct.alway_contract.clone(), Some(complex_chain_call_contract.clone()), &input_token_cell, 100);
    tx = ct.add_outpoint(tx, ct.alway_contract.clone(), Some(complex_chain_call_contract.clone()), &input_token_cell, 100);
    tx = ct.add_contract_cell_dep(tx, &linked_server_contract);
    tx = ct.add_contract_cell_dep(tx, &boundary_server_contract);
    tx = ct.add_contract_cell_dep(tx, &large_data_server_contract);
    tx = ct.add_contract_cell_dep(tx, &server_contract);

    tx = ct.context.complete_tx(tx);
    ct.context.should_be_passed(&tx, 1000000000);
}


#[test]
fn test_chain_error_propagation() {
    let mut input_token_cell = XUDTDataCell::new([1; 32], XUDTData { amount: 13 });
    let mut ct = ContractUtil::new();
    let server_contract = ct.deploy_contract("../../demo/build/release/boundary_server");
    let client_contract = ct.deploy_contract("../../demo/build/release/chain_error_propagation_client");

    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input(tx, ct.alway_contract.clone(), Some(client_contract.clone()), &input_token_cell, 100);
    tx = ct.add_outpoint(tx, ct.alway_contract.clone(), Some(client_contract.clone()), &input_token_cell, 100);
    tx = ct.add_contract_cell_dep(tx, &server_contract);
    tx = ct.context.complete_tx(tx);
    ct.context.should_be_passed(&tx, 100000000);

  

}