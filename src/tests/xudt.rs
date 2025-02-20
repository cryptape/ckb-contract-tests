use crate::cells::xudt_data::{XUDTData, XUDTDataCell};
use crate::ContractUtil;
use ckb_testtool::ckb_types::core::{TransactionBuilder, TransactionView};
use ckb_testtool::ckb_types::packed::Transaction;
use ckb_testtool::ckb_types::prelude::{AsTransactionBuilder, Builder, Entity};

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

    tx = ct.add_input(
        tx,
        ct.alway_contract.clone(),
        Some(type_contract.clone()),
        &input_token_cell,
        100,
    );
    tx = ct.add_input(
        tx,
        ct.alway_contract.clone(),
        Some(type_contract.clone()),
        &input_token2_cell,
        100,
    );

    tx = ct.add_outpoint(
        tx,
        ct.alway_contract.clone(),
        Some(type_contract.clone()),
        &output_token1_cell,
        100,
    );
    tx = ct.add_outpoint(
        tx,
        ct.alway_contract.clone(),
        Some(type_contract.clone()),
        &output_token2_cell,
        100,
    );

    tx = ct.context.complete_tx(tx);
    let ret1 = ct.context.should_be_passed(&tx, 1000000);
    println!("ret:{:?}", ret1);
}

#[test]
fn test_12() {
    let arg_bin = hex::decode("6c0300000c000000a2020000960200001c00000020000000490000004d0000007d0000005a0200000000000001000000bf9cddee9210615b33d4ab6d24a0828390f5ae6bc46a3619817fdc8d1f4c9b8a09000000000000000001000000000000000000000034b580d57e2570a13a1661e0d3e9f575593bccec8f3218a15c272bcc62a22fbb00000000dd01000010000000c60000007c010000b600000010000000180000006100000000af5854030000004900000010000000300000003100000003ae445d1bec9930ba5d9c77a0c2110c0f7e5fa504e7343dc13d9aaa8649cfcb021400000044b901a5d52eb7f9fbd82f43d8a4cb797cf7829855000000100000003000000031000000bb4469004225b39e983929db71fe2253cba1d49a76223e9e1d212cdca1f79f28012000000032e555f3ff8e135cece1351a6a2971518392c1e30375c1e006ad0ce8eac07947b600000010000000180000006100000000ce624e03000000490000001000000030000000310000009bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce80114000000c8328aabcd9b9e8e64fbc566c4385c3bdeb219d755000000100000003000000031000000bb4469004225b39e983929db71fe2253cba1d49a76223e9e1d212cdca1f79f28012000000032e555f3ff8e135cece1351a6a2971518392c1e30375c1e006ad0ce8eac0794761000000100000001800000061000000a8ef7f0105000000490000001000000030000000310000009bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce80114000000c8328aabcd9b9e8e64fbc566c4385c3bdeb219d73c000000100000002400000038000000100000000065cd1d000000000000000000000000100000000083a92a17000000000000000000000000000000ca00000008000000be000000be00000010000000be000000be000000aa0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").expect("Failed to decode arg hex string");
    // let data  = Hex::from("0xcb0400000c0000000d040000010400001c00000020000000490000004d000000a9000000a503000000000000010000008924295bde0c8224f3e7324729e70121d91bac34ca763b8fd283905feb75b9fa0a0000000000000000020000000000000000000000cf62b95cc52957d9bbf5d5ee17d07136f2f6b6640bd6db4c1ea37873638c410800000000000000000000000069d4d177686e89e546f6aef9992426f6ab63e28009407c54d921e1406e25d5ed00000000fc02000018000000ce000000840100003a0200009b020000b6000000100000001800000061000000005eb1a8060000004900000010000000300000003100000003ae445d1bec9930ba5d9c77a0c2110c0f7e5fa504e7343dc13d9aaa8649cfcb02140000007d9b5db02c7322b8749ffd83647877f29e65b5815500000010000000300000003100000073e5467341b55ffd7bdeb5b6f32aa0e9433baf6808f8c5f2472dbc36b1ab04f7012000000032e555f3ff8e135cece1351a6a2971518392c1e30375c1e006ad0ce8eac07947b600000010000000180000006100000000ce624e03000000490000001000000030000000310000009bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce80114000000470dcdc5e44064909650113a274b3b36aecb6dc75500000010000000300000003100000073e5467341b55ffd7bdeb5b6f32aa0e9433baf6808f8c5f2472dbc36b1ab04f7012000000032e555f3ff8e135cece1351a6a2971518392c1e30375c1e006ad0ce8eac07947b600000010000000180000006100000000ce624e03000000490000001000000030000000310000009bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce80114000000c8328aabcd9b9e8e64fbc566c4385c3bdeb219d75500000010000000300000003100000073e5467341b55ffd7bdeb5b6f32aa0e9433baf6808f8c5f2472dbc36b1ab04f7012000000032e555f3ff8e135cece1351a6a2971518392c1e30375c1e006ad0ce8eac0794761000000100000001800000061000000a863bba510000000490000001000000030000000310000009bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce80114000000c8328aabcd9b9e8e64fbc566c4385c3bdeb219d7610000001000000018000000610000008df57f0105000000490000001000000030000000310000009bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce80114000000470dcdc5e44064909650113a274b3b36aecb6dc75c000000180000002c0000004000000054000000580000001000000000c2eb0b0000000000000000000000001000000000bf7c481809000000000000000000001000000000bf7c481809000000000000000000000000000000000000be0000000c0000006500000055000000550000001000000055000000550000004100000028c30b9b35e7d50ec7e446ed57acf0ace2d05d6203f006386f0f9cde632b29cc3a0ecd9222cc0c8f2b62a4542922715921d6707f2d13765b3daff7e4a4273ae60155000000550000001000000055000000550000004100000060314e40b6a360f1c76b5fe3c10bf1bd253fc3ef2ae993f367b8f27ff38d92ae765dbfee0dbdc342eae849d82066fc9e773817788811509f8fbf11e4328f4f8201").unwrap();
    let dd = Transaction::from_compatible_slice(&arg_bin).unwrap();
    println!("dd:{:?}", dd.as_advanced_builder());
}
