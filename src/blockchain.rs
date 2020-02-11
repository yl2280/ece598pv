use crate::block::{Block,Header};
use crate::crypto::hash::{Hashable,H256};
use serde::{Serialize, Deserialize};
use crate::crypto::merkle::MerkleTree;
use bincode::{serialize, deserialize};
use crate::transaction::Transaction;
use ring::{digest};
use std::collections::HashMap;


#[derive(Serialize, Deserialize,Debug, Default)]
pub struct Blockchain {
     genesisBlock: Option<Block>,
     tip: H256,
     map: HashMap<H256, Block>,
     height: u32,
}

impl Blockchain {
    /// Create a new blockchain, only containing the genesis block
    pub fn new() -> Self {
        // unimplemented!()
        // let mut x = None;
        let intit = digest::digest(&digest::SHA256, "None".as_bytes());
        let t = <H256>::from(intit);
        Blockchain{
            genesisBlock: None,
            tip: t,
            map: HashMap::new(),
            height: 0,
        }
    }

    /// Insert a block into blockchain
    pub fn insert(&mut self, block: &Block) {
        // unimplemented!()
        if self.genesisBlock.is_none(){
            // let newB = block.as_ref();
            self.genesisBlock = Some(block.clone());
            self.tip = block.hash();
            self.map.insert(block.hash(), block.clone());
        }
        else{
            self.tip = block.hash();
            self.map.insert(block.hash(), block.clone());
            self.height += 1;
        }

    }

    /// Get the last block's hash of the longest chain
    pub fn tip(&self) -> H256 {
        // unimplemented!()
        self.tip
    }

    /// Get the last block's hash of the longest chain
    #[cfg(any(test, test_utilities))]
    pub fn all_blocks_in_longest_chain(&self) -> Vec<H256> {
        unimplemented!()
    }

}

#[cfg(any(test, test_utilities))]
mod tests {
    use super::*;
    use crate::block::test::generate_random_block;
    use crate::crypto::hash::Hashable;

    #[test]
    fn insert_one() {
        let mut blockchain = Blockchain::new();
        let genesis_hash = blockchain.tip();
        let block = generate_random_block(&genesis_hash);
        blockchain.insert(&block);
        assert_eq!(blockchain.tip(), block.hash());

    }
}
