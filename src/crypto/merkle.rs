use super::hash::{Hashable, H256};
use ring::{digest};
use ring::digest::Algorithm;
use serde::{Serialize,Deserialize};


/// A Merkle tree.
#[derive(Serialize, Deserialize,Debug, Default, Clone, PartialEq)]
struct Node {
    val: H256,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
}


#[derive(Serialize, Deserialize, Debug, Default, Clone,PartialEq)]
pub struct MerkleTree {
    root: Option<Box<H256>>,
    // blocks: Vec<H256>,
    tree: Option<Box<Node>>,
 

}

impl MerkleTree {
    pub fn new<T>(data: &[T]) -> Self where T: Hashable, {
        // unimplemented!()
        if data.len() == 0{
            return MerkleTree{root: None, tree: None};
        }

        if data.len() == 1{
            let root = data[0].hash();
            let tree:Node = Node{val: root, l:None, r : None};
            return MerkleTree{root: Some(Box::new(root)), tree: Some(Box::new(tree))};
        }

        let mut vec = Vec::with_capacity(data.len());

        for i in data{
            let leaf = Node{val: i.hash(), l: None, r: None};
            vec.push(leaf);
        }

        

        if vec.len()%2 == 1{
            let le = vec.len()-1;
            let odd = &vec[le];
            let n = odd.val;
            let last = Node{val: odd.val, l: None, r:None};
            vec.push(last);
        }

        while vec.len() > 1{
            let mut vec_ = Vec::new();
            if vec.len()%2 == 1{
                let le = vec.len()-1;
                let odd = &vec[le];
                let n = odd.val;
                let last = Node{val: odd.val, l: odd.l.clone() , r: odd.r.clone()};
                vec.push(last);
            }
            while vec.len()!= 0{
                if vec.len() == 1{
                    vec_.push(vec.remove(0));
                }
                else{
                    let l1 = vec.remove(0);
                    let r1 = vec.remove(0);
                    let l2 = (l1.val).as_ref();
                    let r2 = (r1.val).as_ref();
                    let parent = [&l2[..], &r2[..]].concat();
                    let parentH = digest::digest(&digest::SHA256, &parent);
                    let val = <H256>::from(parentH);
                    let newNode:Node = Node{val,  l: Some(Box::new(l1)), r: Some(Box::new(r1))};
                    // let newNode = Some(Box::new(node))
                    vec_.push(newNode);
                }
            }

            vec = vec_;
        }

        let t = vec.remove(0);
        let r = t.val;

        return MerkleTree{root: Some(Box::new(r)), tree: Some(Box::new(t))};


    }
    pub fn root(&self) -> H256 {
        // unimplemented!()
        let ret = self.root.as_ref();
        let r = ret.unwrap();
        return **r;
    }

    /// Returns the Merkle Proof of data at index i
    pub fn proof(&self, index: usize) -> Vec<H256> {
        // unimplemented!()
        // let cur = self.tree;
        let mut vec = Vec::new();
        // let merkle = self.clone();
        let mut cur = (self.tree).as_ref();
        let mut curr = (cur).unwrap();
        let mut count = 2;
        let i = index + 1;
        let mut j = 0;

        while cur.is_some(){
            // let curr = &*(cur.unwrap());
            j += 1;
            let left = &curr.l;
            let right = &curr.r;
            if left.is_none() || right.is_none(){
                // vec.push(curr.val);
                break;
            }

            if i < count{
                let tran = &((curr.l).as_ref().unwrap());
                cur = Some(tran);

                // cur = &(curr.l.as_ref());
                let sibiling = (curr.r.as_ref()).unwrap();
                let sibV = sibiling.val;
                vec.push(sibV);
                curr = ((curr.l).as_ref()).unwrap();
            }

            else{
                let tran = &((curr.r).as_ref().unwrap());
                cur = Some(tran);
                // cur = curr.r;
            
                // cur = &(curr.r.as_ref());
                let sibiling = (curr.l.as_ref()).unwrap();
                let sibV = sibiling.val;
                vec.push(sibV);
                curr = (curr.r.as_ref()).unwrap();
            }

            count *= 2;
        }
        return vec;

    }
}

/// Verify that the datum hash with a vector of proofs will produce the Merkle root. Also need the
/// index of datum and `leaf_size`, the total number of leaves.
pub fn verify(root: &H256, datum: &H256, proof: &[H256], index: usize, leaf_size: usize) -> bool {
    // unimplemented!()
    let mut i = index + 1;
    let mut ret = datum.clone();
    let mut vec = Vec::from(proof.clone());
    while vec.len() > 0{
        if i%2 == 1{
            let l1 = ret;
            let r1 = vec.remove(vec.len()-1);
            let l2 = l1.as_ref();
            let r2 = r1.as_ref();
            let parent = [&l2[..], &r2[..]].concat();
            let parentH = digest::digest(&digest::SHA256, &parent);
            ret = <H256>::from(parentH);
            i = (i+1)/2;
        } 

        else{
            let r1 = ret;
            let l1 = vec.remove(vec.len()-1);
            let l2 = l1.as_ref();
            let r2 = r1.as_ref();
            let parent = [&l2[..], &r2[..]].concat();
            let parentH = digest::digest(&digest::SHA256, &parent);
            ret = <H256>::from(parentH);
            i /= 2;
        }
    }

    return *root == ret;
}

#[cfg(test)]
mod tests {
    use crate::crypto::hash::H256;
    use super::*;

    macro_rules! gen_merkle_tree_data {
        () => {{
            vec![
                (hex!("0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d")).into(),
                (hex!("0101010101010101010101010101010101010101010101010101010101010202")).into(),
            ]
        }};
    }

    #[test]
    fn root() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let root = merkle_tree.root();
        let a = input_data[0].hash();
        let b = input_data[1].hash();

        assert_eq!(
            root,
            (hex!("6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920")).into()
        );
        // "b69566be6e1720872f73651d1851a0eae0060a132cf0f64a0ffaea248de6cba0" is the hash of
        // "0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d"
        // "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f" is the hash of
        // "0101010101010101010101010101010101010101010101010101010101010202"
        // "6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920" is the hash of
        // the concatenation of these two hashes "b69..." and "965..."
        // notice that the order of these two matters
    }

    #[test]
    fn proof() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        // println!("{:?}", input_data);
        let merkle_tree = MerkleTree::new(&input_data);
        let proof = merkle_tree.proof(0);
        // println!("{:?}", proof);
        assert_eq!(proof,
                   vec![hex!("965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f").into()]
        );
        // "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f" is the hash of
        // "0101010101010101010101010101010101010101010101010101010101010202"
    }

    #[test]
    fn verifying() {
        let input_data: Vec<H256> = gen_merkle_tree_data!();
        let merkle_tree = MerkleTree::new(&input_data);
        let proof = merkle_tree.proof(0);
        assert!(verify(&merkle_tree.root(), &input_data[0].hash(), &proof, 0, input_data.len()));
    }
}
