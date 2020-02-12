use serde::{Serialize, Deserialize};
use crate::crypto::hash::{H256, Hashable};
use crate::crypto::merkle::MerkleTree;
use bincode::{serialize, deserialize};
use crate::transaction::Transaction;
use ring::{digest};
use rand::prelude::*;
use std::time::{Duration, SystemTime};

use chrono::{NaiveDate, NaiveDateTime};

#[derive(Serialize, Deserialize,Debug, Default, Clone,PartialEq)]
pub struct Header {
    pub parant: Option<Box<H256>>,
    pub nonce: u32,
    pub difficulty: H256,
    pub timestamp: i64,
    pub merkle_root: Option<Box<MerkleTree>>,
}

impl Hashable for Header {
    fn hash(&self) -> H256 {
        let temp = digest::digest(&digest::SHA256, &serialize(self).unwrap());
        <H256>::from(temp)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Block {
   pub header: Option<Box<Header>>,
   pub content: Option<Vec<Transaction>>,
   pub height: u32,
}
 
impl Hashable for Block {
    fn hash(&self) -> H256 {
        (self.header).as_ref().unwrap().hash()
    }
}

#[cfg(any(test, test_utilities))]
pub mod test {
    use super::*;
    use crate::crypto::hash::H256;

    pub fn generate_random_block(parent: &H256) -> Block {
        // unimplemented!()
        let mut rng = rand::thread_rng();
        let n2:u32 = rng.gen();
         // let now = SystemTime::now()
        let date_time: NaiveDateTime = NaiveDate::from_ymd(2017, 11, 12).and_hms(17, 33, 44);

        let head = Header{
        	parant: Some(Box::new(*parent)),
        	nonce: n2,
        	difficulty: *parent,
        	timestamp: date_time.timestamp(),
        	merkle_root: None,
        };

        return Block{
        	header: Some(Box::new(head)),
        	content: None,
            height: 0,
        };
    }
}
