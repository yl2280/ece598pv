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
     pub map: HashMap<H256, Block>,
     // height_map: HashMap<H256, u32>
     height: u32,

}

impl Blockchain {
    /// Create a new blockchain, only containing the genesis block
    pub fn new() -> Self {
        // unimplemented!()
        // let mut x = None;
        let intit = digest::digest(&digest::SHA256, "None".as_bytes());
        let t = <H256>::from(intit);
        let rand_u8 = digest::digest(&digest::SHA256,"442cabd17e40d95ac0932d977c0759397b9db4d93c4d62c368b95419db574db0".as_bytes());
        let diff_rand = <H256>::from(rand_u8);
        // let head = Header{
        //     	parant: None,
        //     	nonce: 0,
        //     	difficulty: diff_rand,
        //     	timestamp:  0,
        //     	merkle_root: None,
        //     };

        // let block = Block{
        //     header: Some(Box::new(head)),
        //     content: None,
        //     height: 0,
        // };
        // let mut t = 
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
            let mut b = block.clone();
            b.height = 0;
            self.tip = b.hash();
            self.map.insert(b.hash(), b.clone());
            // self.genesisBlock = Some(b);

        }
        else{
            let parant = block.header.clone().unwrap().parant.unwrap();
            if self.map.get(&parant).unwrap().height + 1 > self.height{
                self.height += 1;
                let mut b = block.clone();
                b.height = self.height;
                self.tip = b.hash();
                self.map.insert(b.hash(), b.clone());
            }
            else{
                let mut b = block.clone();
                b.height = self.map.get(&parant).unwrap().height + 1;
                // self.tip = b.hash();
                self.map.insert(b.hash(), b.clone());
            }
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
        // unimplemented!()
        // let intit = digest::digest(&digest::SHA256, "None".as_bytes());
        // let t = <H256>::from(intit);
        let mut ret = Vec::new();
        // ret.push(t);
        let parant = self.map.get(&self.tip).unwrap().header.clone().unwrap().parant.unwrap();
        let mut parant_b = self.map.get(&parant);
        ret.push(self.tip);
        ret.insert(0,*parant);
        while *parant_b.unwrap() != *self.genesisBlock.as_ref().unwrap(){
            let key = parant_b.unwrap().header.as_ref().unwrap().parant.as_ref().unwrap(); 
            parant_b = self.map.get(&key);
            ret.insert(0,**key);
        }
        return ret;

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
        let block1 = generate_random_block(&genesis_hash);
        let block2 = generate_random_block(&(block1.hash()));
        let block3 = generate_random_block(&(block2.hash()));
        let block4 = generate_random_block(&genesis_hash);
        let block5 = generate_random_block(&(block4.hash()));
        blockchain.insert(&block1);
        blockchain.insert(&block2);
        blockchain.insert(&block3);
        blockchain.insert(&block4);
        blockchain.insert(&block5);
        assert_eq!(blockchain.tip(), block5.hash());

    }
}
