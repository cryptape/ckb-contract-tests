use ckb_std::ckb_types::bytes::Bytes;
use ckb_std::since::{EpochNumberWithFraction, Since};
use ckb_testtool::ckb_crypto::secp::{Generator, Privkey};
use ckb_testtool::ckb_hash;
use ckb_testtool::ckb_types::core::{TransactionBuilder, TransactionView};
use ckb_testtool::ckb_types::{H256, packed};
use ckb_testtool::ckb_types::packed::{WitnessArgs};
use ckb_testtool::ckb_types::prelude::{Builder, Entity, Pack, Unpack};
use serde::Serialize;
use crate::cells::mutisig_all::{MutisigAllArgs, MutisigAllArgsData, MutisigAllCell, WitnessArg};
use serde_molecule::from_slice;
use crate::cell_message::cell::{MoleculeStructFlag};
use crate::cells::bytes_cell::BytesCell;
use crate::cells::xudt_data::{XUDTData, XUDTDataCell};
use crate::ContractUtil;
use crate::prelude::ContextExt;
const SIGNATURE_SIZE: usize = 65;


// 1. 脚本参数验证
//     测试目标：确保脚本参数长度正确，并验证锁定期值。
//     测试用例：
//         用例 1.1：参数长度为 20 字节（仅包含 blake160 哈希）。
//             预期：成功。
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
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            // since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
            since: None,
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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


//         用例 1.2：参数长度为 28 字节（包含 blake160 哈希和有效 since 值）。
//             预期：成功。
#[test]
fn test_args_with_since() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
            // since: None,
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
//         用例 1.3：参数长度为 19 字节。
//             预期：返回 ERROR_ARGUMENTS_LEN。
// #define ERROR_ARGUMENTS_LEN -1
#[test]
fn test_args_len_19() {
    let mutisigAllCell = BytesCell {
        lock_arg: Bytes::from(vec![0; 19]),
        type_arg: None,
        data: Bytes::new(),
        witness: None,
        struct_flag: MoleculeStructFlag::default(),
    };
    let mut ct = ContractUtil::new();
    let secp256k1_blake160_multisig_all_contract = ct.deploy_contract("secp256k1_blake160_multisig_all");
    let secp256k1_data_contract = ct.deploy_contract("secp256k1_data");
    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, Since::from_block_number(9999999, true).unwrap().as_u64(), 1000);
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    let err = ct.context.should_be_failed(&tx, 100000000).expect_err("#define ERROR_ARGUMENTS_LEN -1");
    assert!(err.to_string().contains("code -1"))
}
//         用例 1.4：参数长度为 29 字节。
//             预期：返回 ERROR_ARGUMENTS_LEN。
#[test]
fn test_args_len_29() {
    let mutisigAllCell = BytesCell {
        lock_arg: Bytes::from(vec![0; 29]),
        type_arg: None,
        data: Bytes::new(),
        witness: None,
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
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    let err = ct.context.should_be_failed(&tx, 100000000).expect_err("#define ERROR_ARGUMENTS_LEN -1");
    assert!(err.to_string().contains("code -1"))
}
//         用例 1.5：since 值有效，交易的 since >= 要求。
//             预期：成功。
#[test]
fn test_since_valid() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
    ct.context.should_be_passed(&tx, 100000000);
}
//         用例 1.6：since 值无效，交易的 since < 要求。
//             预期：check_since 返回错误。
#[test]
fn test_since_ill() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, Since::from_block_number(99999, true).unwrap().as_u64(), 1000);
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    let err = ct.context.should_be_failed(&tx, 100000000).expect_err("expected error: ERROR_INCORRECT_SINCE_VALUE");
    assert!(err.to_string().contains("code -24"))
}
//         用例 1.7：since 值不存在
//             预期: 不会验证 check_since
// #[test]
// fn test_no_since(){}


// 2. 多重签名脚本验证
//     测试目标：验证 multisig_script 的格式和规则。
//     测试用例：
//         用例 2.1：保留字段 S = 0。
//             预期：成功。
//         用例 2.2：保留字段 S = 1。
//             预期：返回 ERROR_INVALID_RESERVE_FIELD:-41
#[test]
fn test_s_is_1() {
    let mutisig_all_args = MutisigAllArgs {
        S: 1,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
    let ret = ct.context.should_be_failed(&tx, 100000000).expect_err("expected error: ERROR_INVALID_RESERVE_FIELD");
    assert!(ret.to_string().contains("code -41"))
}

//         用例 2.3：公钥数量 N = 0。
//             预期：返回 ERROR_INVALID_PUBKEYS_CNT:-42。
#[test]
fn test_n_is_0() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: vec![],
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[]);
    let ret = ct.context.should_be_failed(&tx, 100000000).expect_err("expected error: ERROR_INVALID_RESERVE_FIELD");
    println!("ret:{}", ret.to_string());
    assert!(ret.to_string().contains("code -42"))
}

//         用例 2.4：阈值 M = 0，N = 3。
//             预期：返回 ERROR_INVALID_THRESHOLD: -43
#[test]
fn test_m_is_0() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 0,
        keys: generate_keys(1),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
    let ret = ct.context.should_be_failed(&tx, 100000000).expect_err("expected error: ERROR_INVALID_RESERVE_FIELD");
    assert!(ret.to_string().contains("code -43"))
}

//         用例 2.5：阈值 M = 4，N = 3（M > N）。
//             预期：返回 ERROR_INVALID_THRESHOLD。
#[test]
fn test_m_gt_n() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 0,
        M: 4,
        keys: generate_keys(3),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0], &mutisig_all_args.keys[1], &mutisig_all_args.keys[2]]);
    let ret = ct.context.should_be_failed(&tx, 100000000).expect_err("expected error: ERROR_INVALID_THRESHOLD");
    assert!(ret.to_string().contains("code -43"))
}

//         用例 2.6：必须签名的前N个公钥 R = 3，阈值 M = 2，N = 3（R > M）。
//             预期：返回 ERROR_INVALID_REQUIRE_FIRST_N:44。
#[test]
fn test_r_gt_m() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 3,
        M: 2,
        keys: generate_keys(3),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0], &mutisig_all_args.keys[1], &mutisig_all_args.keys[2]]);
    let ret = ct.context.should_be_failed(&tx, 100000000).expect_err("expected error: ERROR_INVALID_REQUIRE_FIRST_N");
    assert!(ret.to_string().contains("code -44"))
}

//         用例 2.7：R = 0，M = 2，N = 3。
//             预期：成功。
#[test]
fn test_n_gt_m_gt_r() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 0,
        M: 2,
        keys: generate_keys(3),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    let tx1 = multi_sign_tx(tx.clone(), &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0], &mutisig_all_args.keys[1]]);
    ct.context.should_be_passed(&tx1, 100000000).expect("pass");
    let tx1 = multi_sign_tx(tx.clone(), &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[1], &mutisig_all_args.keys[2]]);
    ct.context.should_be_passed(&tx1, 100000000).expect("pass");
}

//         用例 2.8：R = M，M = 2，N = 3。
//             预期：成功。
#[test]
fn test_r_eq_m() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 2,
        M: 2,
        keys: generate_keys(3),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    let tx1 = multi_sign_tx(tx.clone(), &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0], &mutisig_all_args.keys[1]]);
    ct.context.should_be_passed(&tx1, 100000000).expect("pass");
    let tx1 = multi_sign_tx(tx.clone(), &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[1], &mutisig_all_args.keys[2]]);
    let err = ct.context.should_be_failed(&tx1, 100000000).expect_err("err");
    assert!(err.to_string().contains("code -52"))
}

// 3. 哈希检查
//     测试目标：确保 multisig_script 的哈希与脚本参数中的 blake160 哈希匹配。
//     测试用例：
//         用例 3.1：multisig_script 哈希与参数匹配。
//             预期：成功。
#[test]
fn test_hash_match() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
    ct.context.should_be_passed(&tx, 100000000).expect("pass");
}
//         用例 3.2：修改 multisig_script 后哈希不匹配。
//             预期：返回 ERROR_MULTSIG_SCRIPT_HASH。
#[test]
fn test_hash_not_match() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: [0; 20],
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
    let ret = ct.context.should_be_failed(&tx, 100000000).expect_err("expected error: ERROR_MULTSIG_SCRIPT_HASH");
    println!("ret:{}", ret.to_string());
    assert!(ret.to_string().contains("code -51"))
}

// 4. 签名验证
//     测试目标：验证签名数量和有效性，包括阈值和必须签名的前N个公钥规则。
//     测试用例：
//         用例 4.1：2-of-3 配置，提供 2 个有效签名。
//             预期：成功。
#[test]
fn test_2_of_3() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 2,
        keys: generate_keys(3),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    let tx1 = multi_sign_tx(tx.clone(), &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[1], &mutisig_all_args.keys[0]]);
    ct.context.should_be_passed(&tx1, 100000000).expect("pass");
}

//         用例 4.2：2-of-3 配置，仅提供 1 个签名。
//             预期：返回 ERROR_WITNESS_SIZE:-22。
#[test]
fn test_2_of_3_err() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 2,
        keys: generate_keys(3),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    let tx1 = multi_sign_tx(tx.clone(), &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
    let ret = ct.context.should_be_failed(&tx1, 100000000).expect_err("err:ERROR_WITNESS_SIZE");
    assert!(ret.to_string().contains("code -22"));
}

//         用例 4.3：提供来自未在 multisig_script 中的公钥的签名。
//             预期：返回 ERROR_VERIFICATION。
#[test]
fn test_2_of_3_err2() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 2,
        keys: generate_keys(3),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    let tx1 = multi_sign_tx(tx.clone(), &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0], &generate_keys(3)[0]]);
    let ret = ct.context.should_be_failed(&tx1, 100000000).expect_err("pass");
    println!("ret:{}", ret.to_string());
    assert!(ret.to_string().contains("code -52"))
}

//         用例 4.4：提供重复签名（同一私钥签名两次）。
//             预期：失败，因为只算一个有效签名。
#[test]
fn test_2_of_3_err3() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 2,
        keys: generate_keys(3),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    let tx1 = multi_sign_tx(tx.clone(), &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0], &mutisig_all_args.keys[0]]);
    let ret = ct.context.should_be_failed(&tx1, 100000000).expect_err("err:ERROR_VERIFICATION");
    assert!(ret.to_string().contains("code -52"))
}

//         用例 4.5：R=1，M=2，N=3，提供第一公钥和第三公钥的签名。
//             预期：成功。
#[test]
fn test_r1_m2_n3() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 2,
        keys: generate_keys(3),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    let tx1 = multi_sign_tx(tx.clone(), &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0], &mutisig_all_args.keys[2]]);
    ct.context.should_be_passed(&tx1, 100000000).expect("pass");
}

//         用例 4.6：R=2，M=2，N=3，仅提供第一公钥和第三公钥的签名。
//             预期：失败，因为第二公钥未签名。
#[test]
fn test_r2_m2_n3_err() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 2,
        M: 2,
        keys: generate_keys(3),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    let tx1 = multi_sign_tx(tx.clone(), &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0], &mutisig_all_args.keys[2]]);
    let ret = ct.context.should_be_failed(&tx1, 100000000).expect_err("err");
    assert!(ret.to_string().contains("code -52"))
}

// 5. 见证处理
//     测试目标：确保见证格式正确，处理多个见证并检查大小限制。
//     测试用例：
//         用例 5.1：第一见证格式正确，包含有效的 multisig_script 和签名。
//             预期：成功。

//         用例 5.2：见证大小超过 MAX_WITNESS_SIZE（32768字节）。
//             预期：返回 ERROR_WITNESS_SIZE。
#[test]
fn test_witness_size() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg{
            lock: None,
            input_type: Some(vec![0;32769]),
            output_type: None,
        }),
        // witness:Some(WitnessArg{
        //     lock: Default::default(),
        //     input_type: Some(Bytes::from(vec![0; 100])),
        //     output_type: None,
        // }),
        struct_flag: MoleculeStructFlag
        {
            lock_arg: true,
            type_arg: false,
            data: false,
            witness: false,
        },
    };
    // mutisigAllCell.witness.unwrap().input_type = Some(vec![0; 32769]);
    let mut ct = ContractUtil::new();
    let secp256k1_blake160_multisig_all_contract = ct.deploy_contract("secp256k1_blake160_multisig_all");
    let secp256k1_data_contract = ct.deploy_contract("secp256k1_data");
    let mut tx = TransactionBuilder::default().build();
    tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, Since::from_block_number(9999999, true).unwrap().as_u64(), 1000);
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    let tx1 = multi_sign_tx(tx.clone(), &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
    let ret = ct.context.should_be_failed(&tx1, 100000000).expect_err("err");
    println!("ret:{}", ret.to_string());
    assert!(ret.to_string().contains("code -22"))
}
//         用例 5.3：多个输入，使用相同锁脚本，提供对应见证。
//             预期：成功，所有见证正确哈希。
#[test]
fn test_multi_input() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 10,
        M: 100,
        keys: generate_keys(115),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    for i in 0..5 {
        tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, Since::from_block_number(9999999, true).unwrap().as_u64(), 1000);
        tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    }
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    let mut keys = vec![];
    for i in 0..100 {
        keys.push(&mutisig_all_args.keys[i]);
    }
    tx = multi_sign_tx(tx.clone(), &mutisig_all_args.get_args().into(), &keys);
    ct.context.should_be_passed(&tx, 1000000000);

}


// 6. 交易结构
//     测试目标：测试多输入交易和不同锁脚本的兼容性。
//     测试用例：
//         用例 6.1：交易包含两个使用相同锁脚本的输入，提供对应见证。
//             预期：成功。
#[test]
fn test_multi_input_same_lock() {
    let mutisig_all_args = MutisigAllArgs {
        S: 0,
        R: 1,
        M: 1,
        keys: generate_keys(1),
    };
    let mut mutisigAllCell = MutisigAllCell {
        lock_arg: MutisigAllArgsData {
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    for i in 0..2 {
        tx = ct.add_input_with_since(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, Since::from_block_number(9999999, true).unwrap().as_u64(), 1000);
        tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    }
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    let tx1 = multi_sign_tx(tx.clone(), &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
    ct.context.should_be_passed(&tx1, 100000000);
}
//         用例 6.2：交易包含使用不同锁脚本的输入。
//             预期：成功，但只验证当前锁脚本相关的见证。


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
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: None,
            // since: Some(Since::from_block_number(100000, true).unwrap().as_u64()),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(mutil_since),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(mutil_since),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(mutil_since),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(mutil_since),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(mutil_since),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(mutil_since),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(mutil_since),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(mutil_since),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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


// 9. 纪元特殊比较
//     描述：测试 epoch_number_with_fraction_cmp 的行为。
//     测试用例：
//         约束 E=10, I=0, L=1，输入 E=10, I=0, L=1 → 相等，预期成功。
//         约束 E=10, I=50, L=100，输入 E=10, I=40, L=100 → 输入较早，预期失败。
//         约束 E=10, I=0, L=1，输入 E=11, I=0, L=1 → 输入较晚，预期成功。
#[test]
fn test_absolute_epoch_epoch_number_with_fraction_cmp() {}

// 10. 多签 since 不为空,tx input since 为空
//       描述：测试多签 since 不为空,tx input since 为空,input since 不可以为空，默认为0，相当于 Since::from_block_number(0, true)
//       测试用例：
//          有效：约束 since = 相对 0 块（0x8000000000000000），since 为默认值。

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
            hash: mutisig_all_args.blake160_args().as_ref().try_into().unwrap(),
            since: Some(mutil_since),
        },
        type_arg: None,
        data: 0,
        witness: Some(WitnessArg::default()),
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
    mutisigAllCell.witness = Some(WitnessArg::default());
    tx = ct.add_outpoint(tx, secp256k1_blake160_multisig_all_contract.clone(), None, &mutisigAllCell, 1000);
    tx = ct.add_contract_cell_dep(tx, &secp256k1_data_contract);
    tx = ct.context.complete_tx(tx);
    tx = multi_sign_tx(tx, &mutisig_all_args.get_args().into(), &[&mutisig_all_args.keys[0]]);
    ct.context.should_be_passed(&tx, 100000000);
}


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