use crate::cell_message::cell::{Cell, MoleculeStructFlag};
use ckb_testtool::ckb_types::prelude::{Entity, Reader};
use serde::{Deserialize, Serialize};
use serde_molecule::big_array_serde;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct FundingWitness {
    pub empty_witness_args: [u8; 16],
    pub version: u64,
    #[serde(with = "big_array_serde")]
    pub funding_out_point: [u8; 36],
    pub pubkey: [u8; 32],
    #[serde(with = "big_array_serde")]
    pub signature: [u8; 64],
}

impl Entity for FundingWitness {
    fn as_bytes(&self) -> Vec<u8> {
        // 序列化结构体为字节数组
        bincode::serialize(self).unwrap()
    }

    fn from_slice(slice: &[u8]) -> Result<Self, ckb_testtool::error::Error> {
        // 从字节数组反序列化为结构体
        bincode::deserialize(slice)
            .map_err(|_| ckb_testtool::error::Error::Encoding)
    }
}

pub struct FundingCell {
    pub lock_arg: [u8; 20],
    pub type_arg: Option<u8>,
    pub data: u8,
    pub witness: Option<FundingWitness>,
    pub struct_flag: MoleculeStructFlag
}

impl Cell for FundingCell {
    fn get_lock_arg(&self) -> Vec<u8> {
        self.lock_arg.to_vec()
    }

    fn get_type_arg(&self) -> Option<Vec<u8>> {
        self.type_arg.map(|t| vec![t])
    }

    fn get_data(&self) -> Vec<u8> {
        vec![self.data]
    }

    fn get_witness(&self) -> Option<Vec<u8>> {
        self.witness.as_ref().map(|w| w.as_bytes())
    }

    fn from_arg(
        lock_arg: Vec<u8>,
        type_arg: Option<Vec<u8>>,
        data: Vec<u8>,
        witness_args: Option<Vec<u8>>,
    ) -> Self {
        let mut fc = FundingCell {
            lock_arg: <[u8; 20]>::try_from(lock_arg).unwrap(),
            type_arg: type_arg.map(|t| t[0]),
            data: data[0],
            witness: None,
            struct_flag: MoleculeStructFlag::default(),
        };
        if let Some(w) = witness_args {
            fc.witness = Some(FundingWitness::from_slice(&w).unwrap());
        }
        fc
    }
}
