use bytes::Bytes;
use ckb_testtool::ckb_crypto::secp::Privkey;
use ckb_testtool::ckb_hash;
use ckb_testtool::ckb_jsonrpc_types::{Deserialize, Serialize};
use crate::cell_message::cell::{MoleculeStructFlag};
use crate::{impl_cell_methods, impl_cell_methods_without_import};

pub struct MutisigAllArgs {
    pub S: u8,
    pub R: u8, // u128 in little endian
    pub M: u8,
    pub keys: Vec<Privkey>,
    // pub since: Option<u64>,
}

impl MutisigAllArgs {
    pub fn get_args(&self) -> Vec<u8> {
        // return to_vec(self, true).unwrap().to_vec();
        let pubkeys = self.keys
            .iter()
            .map(|key| key.pubkey().unwrap())
            .collect::<Vec<_>>();
        let mut script = vec![self.S, self.R, self.M, pubkeys.len() as u8];
        pubkeys.iter().for_each(|pubkey| {
            script.extend_from_slice(&blake160(&pubkey.serialize()));
        });
        script.into()
    }

    pub fn blake160_args(&self) -> Bytes {
        blake160(self.get_args().as_slice())
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct MutisigAllArgsData {
    pub hash: [u8; 20],
    pub since: Option<u64>,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct WitnessArg {
    pub lock: Option<Vec<u8>>,
    pub input_type: Option<Vec<u8>>,
    pub output_type: Option<Vec<u8>>,
}

impl WitnessArg {
    pub fn default() -> Self {
        WitnessArg {
            lock: None,
            input_type: None,
            output_type: None,
        }
    }
    
}

pub struct MutisigAllCell {
    pub lock_arg: MutisigAllArgsData,
    pub type_arg: Option<u8>,
    pub data: u8,
    pub witness: Option<WitnessArg>,
    pub struct_flag: MoleculeStructFlag,
}



impl_cell_methods!(MutisigAllCell);

impl MutisigAllCell {
    pub fn new(lock_arg: MutisigAllArgsData, type_arg: Option<u8>, data: u8, witness: Option<WitnessArg>, struct_flag: MoleculeStructFlag) -> Self {
        MutisigAllCell {
            lock_arg,
            type_arg,
            data,
            witness,
            struct_flag,
        }
    }
    fn default() -> Self {
        MutisigAllCell {
            lock_arg: MutisigAllArgsData {
                hash: [0u8; 20],
                since: None,
            },
            type_arg: None,
            data: 0,
            witness: None,
            struct_flag: MoleculeStructFlag {
                lock_arg: true,
                type_arg: true,
                data: true,
                witness: false,
            },
        }
    }
}


pub fn blake160(message: &[u8]) -> Bytes {
    Bytes::from(ckb_hash::blake2b_256(message)[..20].to_vec())
}