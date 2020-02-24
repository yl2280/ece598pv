use super::message::Message;
use super::peer;
use crate::network::server::Handle as ServerHandle;
use crossbeam::channel;
use log::{debug, warn};
use crate::crypto::hash::{Hashable, H256};
use ring::{digest};
use std::sync::{Arc, Mutex};
use crate::blockchain::{Blockchain};
use crate::block::{Block,Header};
use std::thread;

#[derive(Clone)]
pub struct Context {
    msg_chan: channel::Receiver<(Vec<u8>, peer::Handle)>,
    num_worker: usize,
    server: ServerHandle,
    blockchain: Arc<Mutex<Blockchain>>,
}

pub fn new(
    num_worker: usize,
    msg_src: channel::Receiver<(Vec<u8>, peer::Handle)>,
    server: &ServerHandle
) -> Context {
    let block = Blockchain::new();
    let block_m = Arc::new(Mutex::new(block));
    Context {
        msg_chan: msg_src,
        num_worker,
        server: server.clone(),
        blockchain: block_m,
    }
}

impl Context {
    pub fn start(self) {
        let num_worker = self.num_worker;
        for i in 0..num_worker {
            let cloned = self.clone();
            thread::spawn(move || {
                cloned.worker_loop();
                warn!("Worker thread {} exited", i);
            });
        }
    }

    fn worker_loop(&self) {
        loop {
            let msg = self.msg_chan.recv().unwrap();
            let (msg, peer) = msg;
            let msg: Message = bincode::deserialize(&msg).unwrap();
            match msg {
                Message::Ping(nonce) => {
                    debug!("Ping: {}", nonce);
                    peer.write(Message::Pong(nonce.to_string()));
                }

                Message::Pong(nonce) => {
                    debug!("Pong: {}", nonce);
                }

                Message::NewBlockHashes(nonce) =>{
                    debug!("NewBlockHashes: {:?}", nonce);
                    if nonce.len() != 0{
                        let mut vec = Vec::new();
                        for i in nonce.iter(){
                            if self.blockchain.lock().unwrap().map.get(&i).is_none(){
                                vec.push(*i);
                                // peer.write(Message::GetBlocks(nonce));
                            }
                        }
                        if vec.len() != 0{
                            peer.write(Message::GetBlocks(vec));
                        }
                    }
                }

                Message::GetBlocks(nonce) => {
                    debug!("GetBlocks: {:?}", nonce);
                    if nonce.len() != 0{
                        let mut vec = Vec::new();
                        for i in nonce.iter(){
                            if self.blockchain.lock().unwrap().map.get(&i).is_some(){
                                // let temp = self.blockchain.lock().unwrap().map.get(&i).unwrap();
                                // peer.write(Message::GetBlocks(nonce));
                                vec.push(self.blockchain.lock().unwrap().map.get(&i).unwrap().clone());
                            }
                        }
                        if vec.len() != 0{
                            peer.write(Message::Blocks(vec));
                        }
                    }
                }

                Message::Blocks(nonce) => {
                    debug!("Blocks: {:?}", nonce);
                    for i in nonce.iter(){
                        // let h = 
                        if self.blockchain.lock().unwrap().map.get(&i.hash()).is_none(){
                            self.blockchain.lock().unwrap().insert(&i);
                        }
                    }
                }
            }
        }
    }
}
