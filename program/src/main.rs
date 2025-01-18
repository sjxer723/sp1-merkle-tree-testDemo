//! A simple program to be proven inside the zkVM.

#![no_main]

// use alloy_merkle_tree::tree::MerkleTree;
use alloy_merkle_tree::incremental_tree::IncrementalMerkleTree;
// use alloy_primitives::{Uint, B256};
sp1_zkvm::entrypoint!(main);


pub fn main() {
    let count = sp1_zkvm::io::read::<u8>();

    let mut tree = IncrementalMerkleTree::<8>::new();
    for i in 0..count {
        tree.append([i as u8; 32].into()).unwrap();
    }
    for i in 0..count {
        let leaf = [i as u8; 32].into();
        let proof = tree.proof_at_index(i as usize).unwrap();
        assert!(tree.verify_proof(leaf, i as usize, &proof));
    }

    sp1_zkvm::io::write(&count);

    // let count = sp1_zkvm::io::read::<u64>();

    // let mut tree = MerkleTree::new();
    // for i in 0..count {
    //     tree.insert(B256::from(Uint::from(i)));
    // }
    // tree.finish();

    // for i in 0..count {
    //     let proof = tree.create_proof(&B256::from(Uint::from(i))).unwrap();
    //     assert!(MerkleTree::verify_proof(&proof));
    // }

    // sp1_zkvm::io::write(&count);
}
