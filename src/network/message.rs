use serde::{Serialize, Deserialize};
use crate::crypto::hash::{Hashable, H256};
use ring::{digest};
use std::sync::{Arc, Mutex};
use crate::blockchain::{Blockchain};
use crate::block::{Block,Header};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Ping(String),
    Pong(String),
    NewBlockHashes(Vec<H256>),
    GetBlocks(Vec<H256>),
    Blocks(Vec<Block>),
}


