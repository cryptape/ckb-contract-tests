use bytes::Bytes;
use ckb_testtool::ckb_jsonrpc_types::{Deserialize, Serialize};
use crate::cell_message::cell::MoleculeStructFlag;
use crate::impl_cell_methods;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Arr {
    pub arr: Vec<u8>,
}

pub struct BytesCell {
    pub lock_arg: Bytes,
    pub type_arg: Option<Bytes>,
    pub data: Bytes,
    pub witness: Option<Bytes>,
    pub struct_flag: MoleculeStructFlag,
}

impl BytesCell {
    pub fn default() -> Self {
        BytesCell {
            lock_arg: Bytes::default(),
            type_arg: None,
            data: Bytes::new(),
            witness: None,
            struct_flag: MoleculeStructFlag {
                lock_arg: false,
                type_arg: false,
                data: false,
                witness: false,
            },
        }
    }
}

impl_cell_methods!(BytesCell);
