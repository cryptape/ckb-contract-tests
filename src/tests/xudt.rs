use ckb_testtool::ckb_types::core::{TransactionBuilder, TransactionView};
use ckb_testtool::ckb_types::packed::Transaction;
use ckb_testtool::ckb_types::prelude::{AsTransactionBuilder, Builder, Entity};
use crate::cells::xudt_data::{XUDTData, XUDTDataCell};
use crate::{ContractUtil};
use ckb_testtool::ckb_jsonrpc_types as json_types;
use crate::prelude::ContextExt;


#[test]
fn test_transfer_successful() {
    let input_token_cell = XUDTDataCell::new([1; 32], XUDTData { amount: 2005 });
    let input_token2_cell = XUDTDataCell::new([1; 32], XUDTData { amount: 2001 });

    let output_token1_cell = XUDTDataCell::new([1; 32], XUDTData { amount: 2000 });
    let output_token2_cell = XUDTDataCell::new([1; 32], XUDTData { amount: 2000 });

    let mut ct = ContractUtil::new();
    let type_contract = ct.deploy_contract("XUDT");
    let mut tx = TransactionBuilder::default().build();

    tx = ct.add_input(tx, ct.alway_contract.clone(), Some(type_contract.clone()), &input_token_cell, 100);
    tx = ct.add_input(tx, ct.alway_contract.clone(), Some(type_contract.clone()), &input_token2_cell, 100);

    tx = ct.add_outpoint(tx, ct.alway_contract.clone(), Some(type_contract.clone()), &output_token1_cell, 100);
    tx = ct.add_outpoint(tx, ct.alway_contract.clone(), Some(type_contract.clone()), &output_token2_cell, 100);


    tx = ct.context.complete_tx(tx);
    let ret1 = ct.context.should_be_passed(&tx, 1000000);
    println!("ret:{:?}", ret1);
}

//ckb-cli --url https://testnet.ckbapp.dev/rpc mock-tx dump --tx-file tx.json --output-file mock_tx.json
// status: success
// guopenglin@MacBook-Pro-4 tests % ckb-debugger --tx-file mock_tx.json --cell-index 0 --cell-type input --script-group-type lock
// Run result: 8
// All cycles: 90466(88.3K)
#[test]
fn test_12() {
    let arg_bin = hex::decode("150400000c00000005020000f90100001c000000200000009300000097000000f3000000e50100000000000003000000fc543a94d69a144e42f7958762b8ea81a8d5f71d7951c47758ddee2366fd268601000000005a5288769cecde6451cb5d301416c297a6da43dc3ac2f3253542b4082478b19b0000000000f8de3bb47d055cdf460d93a2a6e1b05f7432f9777c8c474abf4eec1d4aee5d37000000000100000000020000000421986800000040af5032d11f3d487abf93cfbf02eecda08248468111e710c46d10822b537b94fa0000000004000000000300a0af5032d11f3d487abf93cfbf02eecda08248468111e710c46d10822b537b94fa01000000f20000000c0000009100000085000000100000001800000085000000993fefd2a90100006d000000100000003000000031000000740dee83f87c6f309824d8fd3fbdd3c8380ee6fc9acc90b1a748438afcdf81d801380000000b4bf321379ecccbff98c4b018b68f3f5b14741806000000000100a000000000000034e4609a52f335f15429d9f469777c6e154794573117610000001000000018000000610000001b5e3d4e15080000490000001000000030000000310000009bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8011400000031c33732dabea1e0459ffc9e3c6306229b35eaad140000000c000000100000000000000000000000100200000c000000b7010000a701000010000000100000001000000010000000030400e10700000000000000000000000000001120d85360c1dcd87aeeeb4e6ecd059596947334da1a7b9bf5d03a39a0344c2833bc6682f8b07be5f0dfcf0573db6ebeff5397d6336e9698e9c890dfdf09a0680000004000db070000000000000000000000000000ca89d97ef16cdb4dc7a2574d8ef7093109b9898e6da4d916c81f8eafbf30a0682d2a4768ec3524891422c88e0d49ab50908e52a9c94b9eafdf3d11e53d679d680000004000e1070000000000000000000000000000b3a50ed915cbbea3edc8e16af1434db6f22fa650ef069a7a2df9126a677822f463fe5546841923debc01f308c0e4379663d617f2dd228ab6c931e8af830aa0680000004000e407000000000000000000000000000009ee8caf8793311b74b274b22c8405cd272e54402ccaf668213ca807933b57c217787920038b13072c7a1bb88840c3576184a8438cb7e488518d6d4d6b5ca16800000040a041dc3efe5b351d654a42a7102285cf838912b766050c2fd627599d9a5ed083147dea480468e6a36c6c1b7b13102bc4b38d84f565e8addf8e37f264f92abfc800550000005500000010000000550000005500000041000000ea06be80eb5cef7e93fb00b7edb0ae6322e5e2b6fb654758dbac9e195d83fb3d49af268b431c606f699c4f9433ab42164ebad69c83170081c3e5b36d36fd5b0300").expect("Failed to decode arg hex string");
    let dd = Transaction::from_compatible_slice(&arg_bin).unwrap();
    let tx:json_types::Transaction = dd.as_advanced_builder().build().data().into();
    println!("tx:{:?}", serde_json::to_string_pretty(&tx).unwrap());
    std::fs::write("tx1.json", serde_json::to_string_pretty(&tx).unwrap()).unwrap();
}
