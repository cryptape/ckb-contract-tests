use ckb_std::ckb_types::bytes::Bytes;
use ckb_std::ckb_types::packed::CellInput;
use ckb_std::since::{EpochNumberWithFraction, Since};
use ckb_testtool::ckb_crypto::secp::{Generator, Privkey};
use ckb_testtool::ckb_hash;
use ckb_testtool::ckb_types::core::{TransactionBuilder, TransactionView};
use ckb_testtool::ckb_types::{H256, packed};
use ckb_testtool::ckb_types::packed::{CellInputBuilder, WitnessArgs};
use ckb_testtool::ckb_types::prelude::{Builder, Entity, Pack, Unpack};
use serde::Serialize;
use serde_molecule::to_vec;
use crate::cells::mutisig_all::{MutisigAllArgs, MutisigAllArgsData, MutisigAllCell, WitnessArg};
use serde_molecule::from_slice;
use crate::cell_message::cell::{Cell, MoleculeStructFlag};
use crate::ContractUtil;
use crate::prelude::ContextExt;
const SIGNATURE_SIZE: usize = 65;


#[test]
fn test_demo() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            Hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(from_slice(&hex::decode(
            "10000000100000001000000010000000").unwrap(), false).unwrap()),
        struct_flag: MoleculeStructFlag
        {
            lock_arg: true,
            type_arg: false,
            data: false,
            witness: false,
        },
    };
    let mut ct = ContractUtil::new();
    let secp256k1_blake160_multisig_all_contract = ct.deploy_contract("secp256k1_blake160_multisig_all");
    let secp256k1_data_contract = ct.deploy_contract("secp256k1_data");
    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, Since::from_block_number(9999999, true).unwrap().as_u64(), 1000);
    tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, Since::from_block_number(9999999, true).unwrap().as_u64(), 1000);
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
    ct.context.should_be_passed(&tx, 100000000);
}

// 7. 验证 check_since 检查

// 1. 无 since 字段
//     描述：since 字段为空，验证输入的 since 值是否 >= 约束。
//
#[test]
fn test_no_since() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };
    let mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            Hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: None,
            // since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(from_slice(&hex::decode(
            "10000000100000001000000010000000").unwrap(), false).unwrap()),
        struct_flag: MoleculeStructFlag
        {
            lock_arg: true,
            type_arg: false,
            data: false,
            witness: false,
        },
    };
    let mut ct = ContractUtil::new();
    let secp256k1_blake160_multisig_all_contract = ct.deploy_contract("secp256k1_blake160_multisig_all");
    let secp256k1_data_contract = ct.deploy_contract("secp256k1_data");
    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, Since::from_block_number(1, true).unwrap().as_u64(), 1000);
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
    ct.context.should_be_passed(&tx, 100000000);
}


//  2. 绝对块号
//     描述：since 字段指定绝对块号，验证输入的 since 值是否 >= 约束。
//     测试用例：
//         有效：约束 since = 100，输入 since = 150（标志匹配，值 >= 100），预期成功。
//         无效：约束 since = 100，输入 since = 50（标志匹配，值 < 100），预期返回 ERROR_INCORRECT_SINCE_VALUE。
//         标志不匹配：输入 since 标志为时间戳，预期返回 ERROR_INCORRECT_SINCE_FLAGS。
//     编码：since = (0 << 63) | (0 << 61) | value，小端序。例如，since = 100 为 0x0000000000000064。
#[test]
fn test_absolute_block() {
    let mutil_since = Since::from_block_number(100000, true).unwrap().as_u64();
    let valid_tx_input_since = Since::from_block_number(100000, true).unwrap().as_u64();
    //无效：约束
    let illegal_tx_input_since = Since::from_block_number(100000 - 1, true).unwrap().as_u64();
    // 标志不匹配
    let not_match_tx_input_since = Since::from_timestamp(100000, true).unwrap().as_u64();

    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };
    let mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            Hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(mutil_since),
        },
        type_arg: None,
        data: 0,
        witness: Some(from_slice(&hex::decode(
            "10000000100000001000000010000000").unwrap(), false).unwrap()),
        struct_flag: MoleculeStructFlag
        {
            lock_arg: true,
            type_arg: false,
            data: false,
            witness: false,
        },
    };
    let mut ct = ContractUtil::new();
    let secp256k1_blake160_multisig_all_contract = ct.deploy_contract("secp256k1_blake160_multisig_all");
    let secp256k1_data_contract = ct.deploy_contract("secp256k1_data");

    // valid
    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, valid_tx_input_since, 1000);
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
    ct.context.should_be_passed(&tx, 100000000);

    // illegal
    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, illegal_tx_input_since, 1000);
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
    let ret = ct.context.should_be_failed(&tx, 100000000).expect_err("expected error: ERROR_INCORRECT_SINCE_VALUE");
    assert!(ret.to_string().contains("code -24"));

    // 标志不匹配
    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, not_match_tx_input_since, 1000);
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
    let ret = ct.context.should_be_failed(&tx, 100000000).expect_err("expected error: ERROR_INCORRECT_SINCE_FLAGS");
    assert!(ret.to_string().contains("code -23"))
}


// 3. 相对块号
//     描述：since 字段指定相对块号，脚本直接比较数值。
//     测试用例：
//         有效：约束 since = 相对 10 块（0x800000000000000A），输入 since 匹配标志且值 >= 10，预期成功。
//         无效：输入 since 值 < 10，预期返回 ERROR_INCORRECT_SINCE_VALUE。
//     编码：since 最高位为 1，value 为增量。
#[test]
fn test_relative_block() {
    let mutil_since = Since::from_block_number(10, false).unwrap().as_u64();
    let valid_tx_input_since_vecs = vec![
        Since::from_block_number(10, false).unwrap().as_u64(),
        Since::from_block_number(11, false).unwrap().as_u64(),
        Since::from_block_number(1000, false).unwrap().as_u64(),
    ];
    let ill_tx_input_since_vecs = vec![
        Since::from_block_number(9, false).unwrap().as_u64()
    ];

    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };
    let mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            Hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(mutil_since),
        },
        type_arg: None,
        data: 0,
        witness: Some(from_slice(&hex::decode(
            "10000000100000001000000010000000").unwrap(), false).unwrap()),
        struct_flag: MoleculeStructFlag
        {
            lock_arg: true,
            type_arg: false,
            data: false,
            witness: false,
        },
    };
    let mut ct = ContractUtil::new();
    let secp256k1_blake160_multisig_all_contract = ct.deploy_contract("secp256k1_blake160_multisig_all");
    let secp256k1_data_contract = ct.deploy_contract("secp256k1_data");

    // valid
    for valid_tx_input_since in valid_tx_input_since_vecs {
        let mut tx = TransactionBuilder::default().build();
        tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, valid_tx_input_since, 1000);
        tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
        tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
        tx = ct.context.complete_tx(tx);
        tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
        ct.context.should_be_passed(&tx, 100000000);
    }
    // illegal
    for illegal_tx_input_since in ill_tx_input_since_vecs {
        let mut tx = TransactionBuilder::default().build();
        tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, illegal_tx_input_since, 1000);
        tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
        tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
        tx = ct.context.complete_tx(tx);
        tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
        let ret = ct.context.should_be_failed(&tx, 100000000).expect_err("expected error: ERROR_INCORRECT_SINCE_VALUE");
        assert!(ret.to_string().contains("code -24"));
    }
}

// 4. 绝对时间戳
//     描述：since 字段指定绝对时间戳，验证输入是否 >= 约束。
//     测试用例：
//         有效：约束 since = 1600000000（Unix 时间），输入 since 匹配标志且值 >= 1600000000，预期成功。
//         无效：输入 since 值 < 1600000000，预期失败。
//     编码：标志位为 0b00000010（绝对，时间戳），value 为时间戳。
#[test]
fn test_absolute_timestamp() {
    let mutil_since = Since::from_timestamp(1600000000, true).unwrap().as_u64();
    let valid_tx_input_since_vecs = vec![
        Since::from_timestamp(1600000000, true).unwrap().as_u64(),
        Since::from_timestamp(1600000001, true).unwrap().as_u64(),
        // Since::from_timestamp(, true).unwrap().as_u64(),
    ];
    let ill_tx_input_since_vecs = vec![
        Since::from_timestamp(1599999999, true).unwrap().as_u64()
    ];

    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };
    let mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            Hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(mutil_since),
        },
        type_arg: None,
        data: 0,
        witness: Some(from_slice(&hex::decode(
            "10000000100000001000000010000000").unwrap(), false).unwrap()),
        struct_flag: MoleculeStructFlag
        {
            lock_arg: true,
            type_arg: false,
            data: false,
            witness: false,
        },
    };
    let mut ct = ContractUtil::new();
    let secp256k1_blake160_multisig_all_contract = ct.deploy_contract("secp256k1_blake160_multisig_all");
    let secp256k1_data_contract = ct.deploy_contract("secp256k1_data");

    // valid
    for valid_tx_input_since in valid_tx_input_since_vecs {
        let mut tx = TransactionBuilder::default().build();
        tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, valid_tx_input_since, 1000);
        tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
        tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
        tx = ct.context.complete_tx(tx);
        tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
        ct.context.should_be_passed(&tx, 100000000);
    }

    for ill_tx_input_since in ill_tx_input_since_vecs {
        let mut tx = TransactionBuilder::default().build();
        tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, ill_tx_input_since, 1000);
        tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
        tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
        tx = ct.context.complete_tx(tx);
        tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
        let ret = ct.context.should_be_failed(&tx, 100000000).expect_err("expected error: ERROR_INCORRECT_SINCE_VALUE");
        println!("ret:{:?}", ret);
        assert!(ret.to_string().contains("code -24"));
    }
}

//5. 相对时间戳
//     描述：since 字段指定相对时间戳，脚本直接比较数值。
//     测试用例：
//         有效：约束 since = 相对 100 秒，输入 since 匹配标志且值 >= 100，预期成功。
//         无效：输入 since 值 < 100，预期失败。
//     编码：标志位为 0b10000010（相对，时间戳）。

#[test]
fn test_relative_timestamp() {
    let mutil_since = Since::from_timestamp(100, false).unwrap().as_u64();
    let valid_tx_input_since_vecs = vec![
        Since::from_timestamp(100, false).unwrap().as_u64(),
        Since::from_timestamp(101, false).unwrap().as_u64(),
    ];
    let ill_tx_input_since_vecs = vec![
        Since::from_timestamp(99, false).unwrap().as_u64(),
        Since::from_timestamp(0, false).unwrap().as_u64(),
    ];

    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };
    let mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            Hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(mutil_since),
        },
        type_arg: None,
        data: 0,
        witness: Some(from_slice(&hex::decode(
            "10000000100000001000000010000000").unwrap(), false).unwrap()),
        struct_flag: MoleculeStructFlag
        {
            lock_arg: true,
            type_arg: false,
            data: false,
            witness: false,
        },
    };
    let mut ct = ContractUtil::new();
    let secp256k1_blake160_multisig_all_contract = ct.deploy_contract("secp256k1_blake160_multisig_all");
    let secp256k1_data_contract = ct.deploy_contract("secp256k1_data");

    // valid
    for valid_tx_input_since in valid_tx_input_since_vecs {
        let mut tx = TransactionBuilder::default().build();
        tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, valid_tx_input_since, 1000);
        tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
        tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
        tx = ct.context.complete_tx(tx);
        tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
        ct.context.should_be_passed(&tx, 100000000);
    }

    for ill_tx_input_since in ill_tx_input_since_vecs {
        let mut tx = TransactionBuilder::default().build();
        tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, ill_tx_input_since, 1000);
        tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
        tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
        tx = ct.context.complete_tx(tx);
        tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
        let ret = ct.context.should_be_failed(&tx, 100000000).expect_err("expected error: ERROR_INCORRECT_SINCE_VALUE");
        println!("ret:{:?}", ret);
        assert!(ret.to_string().contains("code -24"));
    }
}

// 6. 绝对纪元
//     描述：since 字段指定绝对纪元，验证输入是否 >= 约束，使用 epoch_number_with_fraction_cmp。
//     测试用例：
//         有效：约束 since = 纪元 10.0（E=10, I=0, L=1），输入 since = 纪元 10.5（E=10, I=50, L=100），预期成功（因后者较晚）。
//         无效：输入 since = 纪元 9.9（E=9, I=99, L=100），预期返回 ERROR_INCORRECT_SINCE_VALUE。
//     编码：标志位为 0b00100000（绝对，纪元），value 编码 E, I, L。
#[test]
fn test_absolute_epoch() {
    let mutil_since = Since::from_epoch(EpochNumberWithFraction::new(10, 10, 100), true).as_u64();
    let valid_tx_input_since_vecs = vec![
        Since::from_epoch(EpochNumberWithFraction::new(11, 0, 1), true).as_u64(),
        Since::from_epoch(EpochNumberWithFraction::new(10, 10, 100), true).as_u64(),
        Since::from_epoch(EpochNumberWithFraction::new(10, 11, 100), true).as_u64(),
    ];
    let ill_tx_input_since_vecs = vec![
        // Since::from_epoch(EpochNumberWithFraction::new(9, 15, 255), true).as_u64(),
        Since::from_epoch(EpochNumberWithFraction::new(10, 9, 100), true).as_u64(),
        Since::from_epoch(EpochNumberWithFraction::new(10, 9, 99), true).as_u64(),
        Since::from_epoch(EpochNumberWithFraction::new(9, 99, 101), true).as_u64(),
    ];

    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };
    let mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            Hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(mutil_since),
        },
        type_arg: None,
        data: 0,
        witness: Some(from_slice(&hex::decode(
            "10000000100000001000000010000000").unwrap(), false).unwrap()),
        struct_flag: MoleculeStructFlag
        {
            lock_arg: true,
            type_arg: false,
            data: false,
            witness: false,
        },
    };
    let mut ct = ContractUtil::new();
    let secp256k1_blake160_multisig_all_contract = ct.deploy_contract("secp256k1_blake160_multisig_all");
    let secp256k1_data_contract = ct.deploy_contract("secp256k1_data");

    // valid
    for valid_tx_input_since in valid_tx_input_since_vecs {
        let mut tx = TransactionBuilder::default().build();
        tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, valid_tx_input_since, 1000);
        tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
        tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
        tx = ct.context.complete_tx(tx);
        tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
        ct.context.should_be_passed(&tx, 100000000);
    }
    // illegal
    for ill_tx_input_since in ill_tx_input_since_vecs {
        let mut tx = TransactionBuilder::default().build();
        tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, ill_tx_input_since, 1000);
        tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
        tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
        tx = ct.context.complete_tx(tx);
        tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
        let ret = ct.context.should_be_failed(&tx, 100000000).expect_err("expected error: ERROR_INCORRECT_SINCE_VALUE");
        println!("ret:{:?}", ret);
        assert!(ret.to_string().contains("code -24"));
    }
}



//7. 相对纪元
//     描述：since 字段指定相对纪元，脚本直接比较数值。
//     测试用例：
//         有效：约束 since = 相对 2 纪元，输入 since 匹配标志且值 >= 约束，预期成功。
//         无效：输入 since 值 < 约束，预期失败。
//     编码：标志位为 0b10100000（相对，纪元）。
// todo : 目前不支持 
#[test]
#[ignore]
fn test_relative_epoch() {
    let mutil_since = Since::from_epoch(EpochNumberWithFraction::new(10, 10, 100), false).as_u64();
    let valid_tx_input_since_vecs = vec![
        Since::from_epoch(EpochNumberWithFraction::new(10, 10, 100), false).as_u64(),
        // Since::from_epoch(EpochNumberWithFraction::new(10, 10, 100), false).as_u64(),
        // Since::from_epoch(EpochNumberWithFraction::new(10, 11, 100), false).as_u64(),
    ];
    let ill_tx_input_since_vecs = vec![
        // Since::from_epoch(EpochNumberWithFraction::new(10, 9, 100), false).as_u64(),
        // Since::from_epoch(EpochNumberWithFraction::new(10, 9, 99), false).as_u64(),
        // Since::from_epoch(EpochNumberWithFraction::new(9, 99, 101), false).as_u64(),
    ];

    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };
    let mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            Hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(mutil_since),
        },
        type_arg: None,
        data: 0,
        witness: Some(from_slice(&hex::decode(
            "10000000100000001000000010000000").unwrap(), false).unwrap()),
        struct_flag: MoleculeStructFlag
        {
            lock_arg: true,
            type_arg: false,
            data: false,
            witness: false,
        },
    };
    let mut ct = ContractUtil::new();
    let secp256k1_blake160_multisig_all_contract = ct.deploy_contract("secp256k1_blake160_multisig_all.debug.4");
    let secp256k1_data_contract = ct.deploy_contract("secp256k1_data");

    // valid
    for valid_tx_input_since in valid_tx_input_since_vecs {
        let mut tx = TransactionBuilder::default().build();
        tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, valid_tx_input_since, 1000);
        tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
        tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
        tx = ct.context.complete_tx(tx);
        tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
        ct.context.should_be_failed(&tx, 100000000);
    }
    // illegal
    for ill_tx_input_since in ill_tx_input_since_vecs {
        let mut tx = TransactionBuilder::default().build();
        tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, ill_tx_input_since, 1000);
        tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
        tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
        tx = ct.context.complete_tx(tx);
        tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
        let ret = ct.context.should_be_failed(&tx, 100000000).expect_err("expected error: ERROR_INCORRECT_SINCE_VALUE");
        assert!(ret.to_string().contains("code -24"));
    }
}


// 8. 多个输入
// 描述：测试多个输入是否均满足条件。
// 测试用例：
//     所有输入满足：两个输入均匹配标志且值 >= 约束，预期成功。
//     一个输入不满足：一个输入值 < 约束，预期返回 ERROR_INCORRECT_SINCE_VALUE。
#[test]
fn test_mutil_input_since() {
    let mutil_since = Since::from_block_number(100000, true).unwrap().as_u64();
    // 所有输入满足
    let valid_tx_input_since_vecs = vec![
        vec![
            Since::from_block_number(100000, true).unwrap().as_u64(),
            Since::from_block_number(100001, true).unwrap().as_u64(),
        ],
    ];
    // 所有输入不满足
    let ill_tx_input_since_vecs = vec![
        vec![
            Since::from_block_number(100000, true).unwrap().as_u64(),
            Since::from_block_number(100000, false).unwrap().as_u64(),
        ],
        vec![
            Since::from_block_number(100000, true).unwrap().as_u64(),
            Since::from_block_number(99999, true).unwrap().as_u64(),
        ],
    ];

    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };

    let mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            Hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(mutil_since),
        },
        type_arg: None,
        data: 0,
        witness: Some(from_slice(&hex::decode(
            "10000000100000001000000010000000").unwrap(), false).unwrap()),
        struct_flag: MoleculeStructFlag
        {
            lock_arg: true,
            type_arg: false,
            data: false,
            witness: false,
        },
    };
    let mut ct = ContractUtil::new();
    let secp256k1_blake160_multisig_all_contract = ct.deploy_contract("secp256k1_blake160_multisig_all");
    let secp256k1_data_contract = ct.deploy_contract("secp256k1_data");

    // valid
    for valid_tx_input_sinces in valid_tx_input_since_vecs {
        let mut tx = TransactionBuilder::default().build();
        for valid_tx_input_since in valid_tx_input_sinces {
            tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, valid_tx_input_since, 1000);
            tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
        }
        tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
        tx = ct.context.complete_tx(tx);
        tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
        ct.context.should_be_passed(&tx, 100000000);
    }
    // illegal
    for ill_tx_input_sinces in ill_tx_input_since_vecs {
        let mut tx = TransactionBuilder::default().build();
        for ill_tx_input_since in ill_tx_input_sinces {
            tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, ill_tx_input_since, 1000);
        }
        tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
        tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
        tx = ct.context.complete_tx(tx);
        tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
        let ret = ct.context.should_be_failed(&tx, 100000000).expect_err("expected error: ERROR_INCORRECT_SINCE_VALUE");
        println!("ret:{:?}", ret);
        // assert!(ret.to_string().contains("code -24"));
    }
}

#[test]
fn test_mutil_input_since_2() {
    let mutil_since = Since::from_block_number(100000, true).unwrap().as_u64();
    // 所有输入满足
    let valid_tx_input_since_vecs = vec![
        vec![
            Since::from_block_number(100000, true).unwrap().as_u64(),
            Since::from_block_number(100001, true).unwrap().as_u64(),
        ],
    ];
   

    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };

    let mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            Hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(mutil_since),
        },
        type_arg: None,
        data: 0,
        witness: Some(from_slice(&hex::decode(
            "10000000100000001000000010000000").unwrap(), false).unwrap()),
        struct_flag: MoleculeStructFlag
        {
            lock_arg: true,
            type_arg: false,
            data: false,
            witness: false,
        },
    };
    let mut ct = ContractUtil::new();
    let secp256k1_blake160_multisig_all_contract = ct.deploy_contract("secp256k1_blake160_multisig_all");
    let secp256k1_data_contract = ct.deploy_contract("secp256k1_data");

    // valid
    for valid_tx_input_sinces in valid_tx_input_since_vecs {
        let mut tx = TransactionBuilder::default().build();
        for valid_tx_input_since in valid_tx_input_sinces {
            tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, valid_tx_input_since, 1000);
            tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);

        }
        // tx = ct.add_input(tx, ct.alway_contract.clone(), None, &mutisigAllCell, 1000);
        tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
        tx = ct.context.complete_tx(tx);
        tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
        ct.context.should_be_passed(&tx, 100000000);
    }

}



#[test]
fn test_mutil_input_since_empty() {
    let mutil_since = Since::from_block_number(0, true).unwrap().as_u64();
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };

    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            Hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(mutil_since),
        },
        type_arg: None,
        data: 0,
        witness: Some(from_slice(&hex::decode(
            "10000000100000001000000010000000").unwrap(), false).unwrap()),
        struct_flag: MoleculeStructFlag
        {
            lock_arg: true,
            type_arg: false,
            data: false,
            witness: false,
        },
    };
    let mut ct = ContractUtil::new();
    let secp256k1_blake160_multisig_all_contract = ct.deploy_contract("secp256k1_blake160_multisig_all");
    let secp256k1_data_contract = ct.deploy_contract("secp256k1_data");

    // valid
    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    mutisigAllCell.witness = Some(WitnessArg{
        lock: Bytes::default(),
        input_type:None,
        output_type:None
    });
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
    ct.context.should_be_passed(&tx, 100000000);
}

// 9. 纪元特殊比较
//     描述：测试 epoch_number_with_fraction_cmp 的行为。
//     测试用例：
//         约束 E=10, I=0, L=1，输入 E=10, I=0, L=1 → 相等，预期成功。
//         约束 E=10, I=50, L=100，输入 E=10, I=40, L=100 → 输入较早，预期失败。
//         约束 E=10, I=0, L=1，输入 E=11, I=0, L=1 → 输入较晚，预期成功。
fn multi_sign_tx(
    tx: TransactionView,
    multi_sign_script: &Bytes,
    keys: &[&Privkey],
) -> TransactionView {
    let tx_hash = tx.hash();
    let signed_witnesses: Vec<packed::Bytes> = tx
        .inputs()
        .into_iter()
        .enumerate()
        .map(|(i, _)| {
            if i == 0 {
                let mut blake2b = ckb_hash::new_blake2b();
                let mut message = [0u8; 32];
                blake2b.update(&tx_hash.raw_data());
                let witness = WitnessArgs::new_unchecked(Unpack::<Bytes>::unpack(
                    &tx.witnesses().get(0).unwrap(),
                ));
                let mut lock = multi_sign_script.to_vec();
                let lock_without_sig = {
                    let sig_len = keys.len() * SIGNATURE_SIZE;
                    let mut buf = lock.clone();
                    buf.resize(buf.len() + sig_len, 0);
                    buf
                };
                let witness_without_sig = witness
                    .clone()
                    .as_builder()
                    .lock(Some(Bytes::from(lock_without_sig)).pack())
                    .build();
                let len = witness_without_sig.as_bytes().len() as u64;
                blake2b.update(&len.to_le_bytes());
                blake2b.update(&witness_without_sig.as_bytes());
                (1..tx.witnesses().len()).for_each(|n| {
                    let witness: Bytes = tx.witnesses().get(n).unwrap().unpack();
                    let len = witness.len() as u64;
                    blake2b.update(&len.to_le_bytes());
                    blake2b.update(&witness);
                });
                blake2b.finalize(&mut message);
                let message = H256::from(message);
                keys.iter().for_each(|key| {
                    let sig = key.sign_recoverable(&message).expect("sign");
                    lock.extend_from_slice(&sig.serialize());
                });
                witness
                    .as_builder()
                    .lock(Some(Bytes::from(lock)).pack())
                    .build()
                    .as_bytes()
                    .pack()
            } else {
                tx.witnesses().get(i).unwrap_or_default()
            }
        })
        .collect();
    // calculate message
    tx.as_advanced_builder()
        .set_witnesses(signed_witnesses)
        .build()
}


fn generate_keys(n: usize) -> Vec<Privkey> {
    let mut keys = Vec::with_capacity(n);
    for _ in 0..n {
        keys.push(Generator::random_privkey());
    }
    keys
}


pub fn blake160(message: &[u8]) -> Bytes {
    Bytes::from(ckb_hash::blake2b_256(message)[..20].to_vec())
}