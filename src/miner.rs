use crate::network::server::Handle as ServerHandle;

use log::info;

use crossbeam::channel::{unbounded, Receiver, Sender, TryRecvError};
use std::time;

use std::thread;
use std::sync::{Arc, Mutex};
use crate::blockchain::{Blockchain};
use crate::block::{Block,Header};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use rand::prelude::*;
use chrono::prelude::*;
use crate::crypto::hash::{Hashable, H256};
use ring::{digest};


enum ControlSignal {
    Start(u64), // the number controls the lambda of interval between block generation
    Exit,
}

enum OperatingState {
    Paused,
    Run(u64),
    ShutDown,
}

pub struct Context {
    /// Channel for receiving control signal
    control_chan: Receiver<ControlSignal>,
    operating_state: OperatingState,
    server: ServerHandle,
    blockchain: Arc<Mutex<Blockchain>>,
}

#[derive(Clone)]
pub struct Handle {
    /// Channel for sending signal to the miner thread
    control_chan: Sender<ControlSignal>,
}

pub fn new(
    server: &ServerHandle, blockchain: &Arc<Mutex<Blockchain>> 
) -> (Context, Handle) {
    let (signal_chan_sender, signal_chan_receiver) = unbounded();

    let ctx = Context {
        control_chan: signal_chan_receiver,
        operating_state: OperatingState::Paused,
        server: server.clone(),
        blockchain: Arc::clone(blockchain),
    };

    let handle = Handle {
        control_chan: signal_chan_sender,
    };

    (ctx, handle)
}

impl Handle {
    pub fn exit(&self) {
        self.control_chan.send(ControlSignal::Exit).unwrap();
    }

    pub fn start(&self, lambda: u64) {
        self.control_chan
            .send(ControlSignal::Start(lambda))
            .unwrap();
    }

}

impl Context {
    pub fn start(mut self) {
        thread::Builder::new()
            .name("miner".to_string())
            .spawn(move || {
                self.miner_loop();
            })
            .unwrap();
        info!("Miner initialized into paused mode");
    }

    fn handle_control_signal(&mut self, signal: ControlSignal) {
        match signal {
            ControlSignal::Exit => {
                info!("Miner shutting down");
                self.operating_state = OperatingState::ShutDown;
            }
            ControlSignal::Start(i) => {
                info!("Miner starting in continuous mode with lambda {}", i);
                self.operating_state = OperatingState::Run(i);
            }
        }
    }

    fn miner_loop(&mut self) {
        let rand_u8 = digest::digest(&digest::SHA256,"442cabd17e40d95ac0932d977c0759397b9db4d93c4d62c368b95419db574db0".as_bytes());
        let diff_rand = <H256>::from(rand_u8);
        // main mining loop
        loop {
            // check and react to control signals
            match self.operating_state {
                OperatingState::Paused => {
                    let signal = self.control_chan.recv().unwrap();
                    self.handle_control_signal(signal);
                    continue;
                }
                OperatingState::ShutDown => {
                    return;
                }
                _ => match self.control_chan.try_recv() {
                    Ok(signal) => {
                        self.handle_control_signal(signal);
                    }
                    Err(TryRecvError::Empty) => {}
                    Err(TryRecvError::Disconnected) => panic!("Miner control channel detached"),
                },
            }
            if let OperatingState::ShutDown = self.operating_state {
                return;
            }

            // TODO: actual mining
            let mut rng = rand::thread_rng();
            let n2:u32 = rng.gen();
            let parent = self.blockchain.lock().unwrap().tip();
            let diff;
            if  self.blockchain.lock().unwrap().map.get(&parent).is_none(){
                diff = self.blockchain.lock().unwrap().map.get(&parent).unwrap().header.as_ref().unwrap().difficulty;
            }
            else{
                diff = diff_rand;
            }

            let in_ms:u64;
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => in_ms = n.as_secs() * 1000 + n.subsec_nanos() as u64 / 1000000,
                Err(_) => panic!("SystemTime before UNIX EPOCH!"),
            }


            let now = Utc::now();
            let head = Header{
            	parant: Some(Box::new(parent)),
            	nonce: n2,
            	difficulty: diff,
            	timestamp:  in_ms,
            	merkle_root: None,
            };

            let block = Block{
            	header: Some(Box::new(head)),
            	content: None,
            	height: 0,
            };

            if block.hash() <=diff{
            	self.blockchain.lock().unwrap().insert(&block);
            }

            // self.blockchain.lock().unwrap().insert(&block);


            if let OperatingState::Run(i) = self.operating_state {
                if i != 0 {
                    let interval = time::Duration::from_micros(i as u64);
                    thread::sleep(interval);
                }
            }
        }
    }
}
