#[allow(clippy::unreadable_literal)]
pub static BUNDLED_CELL: ::includedir::Files = ::includedir::Files {
files:  
::phf::Map {
    key: 3347381344252206323,
    disps: ::phf::Slice::Static(&[
        (2, 0),
    ]),
    entries: ::phf::Slice::Static(&[
        ("specs/cells/secp256k1_data", (::includedir::Compression::Gzip, include_bytes!("/Users/xueyanli/vscode/ckb-contract-tests/target/debug/build/ckb-system-scripts-e2857cf3236418f2/out/specs/cells/secp256k1_data") as &'static [u8])),
        ("specs/cells/secp256k1_blake160_sighash_all", (::includedir::Compression::Gzip, include_bytes!("/Users/xueyanli/vscode/ckb-contract-tests/target/debug/build/ckb-system-scripts-e2857cf3236418f2/out/specs/cells/secp256k1_blake160_sighash_all") as &'static [u8])),
        ("specs/cells/secp256k1_blake160_multisig_all", (::includedir::Compression::Gzip, include_bytes!("/Users/xueyanli/vscode/ckb-contract-tests/target/debug/build/ckb-system-scripts-e2857cf3236418f2/out/specs/cells/secp256k1_blake160_multisig_all") as &'static [u8])),
        ("specs/cells/dao", (::includedir::Compression::Gzip, include_bytes!("/Users/xueyanli/vscode/ckb-contract-tests/target/debug/build/ckb-system-scripts-e2857cf3236418f2/out/specs/cells/dao") as &'static [u8])),
    ]),
}
, passthrough: ::std::sync::atomic::AtomicBool::new(false)
};
