use ckb_testtool::ckb_types::core::TransactionBuilder;
use ckb_testtool::ckb_types::prelude::Builder;
use crate::cells::xudt_data::{XUDTData, XUDTDataCell};
use crate::ContractUtil;
use crate::prelude::ContextExt;


#[test]
fn test_boundary_values(){
    let mut input_token_cell = XUDTDataCell::new([1; 32], XUDTData { amount: 65439 });
    let mut ct = ContractUtil::new();
    let server_contract = ct.deploy_contract("../../demo/build/release/boundary_server");
    let client_contract = ct.deploy_contract("../../demo/build/release/boundary_client");

    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input(tx, ct.alway_contract.clone(), Some(client_contract.clone()), &input_token_cell, 100);
    tx = ct.add_outpoint(tx, ct.alway_contract.clone(), Some(client_contract.clone()), &input_token_cell, 100);
    tx = ct.add_contract_cell_dep(tx, &server_contract);
    tx = ct.context.complete_tx(tx);
    ct.context.should_be_passed(&tx, 100000000);

}

