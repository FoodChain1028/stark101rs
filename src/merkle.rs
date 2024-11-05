use crate::field::FieldElement;
use sha256::digest;
use std::collections::HashMap;

pub struct MerkleTree {
    data: Vec<FieldElement>,
    height: u32,
    root: String,
    facts: HashMap<String, (String, String)>,
}

impl MerkleTree {
    pub fn new(data: Vec<FieldElement>) -> Self {
        assert!(!data.is_empty(), "Cannot construct an empty Merkle Tree.");
        let num_leaves = 2_u32.pow((data.len() as f32).log2().ceil() as u32);
        let height = (data.len() as f64).log2().ceil() as u32;
        let mut data_padded = data.clone();
        data_padded.resize(num_leaves as usize, FieldElement::zero());
        let mut tree = MerkleTree {
            data: data_padded,
            height,
            root: String::new(),
            facts: HashMap::new(),
        };
        tree.root = tree.build_tree();
        tree
    }

    fn build_tree(&mut self) -> String {
        self.recursive_build_tree(1)
    }

    fn recursive_build_tree(&mut self, node_id: u32) -> String {
        let data_len: u32 = self
            .data
            .len()
            .try_into()
            .expect("Error converting usize to u32");
        if node_id >= data_len {
            let id_in_data: u32 = node_id - data_len;
            let leaf_data = self.data[id_in_data as usize].to_string();
            let hash = digest(&leaf_data);
            self.facts.insert(hash.clone(), (leaf_data, String::new()));
            hash
        } else {
            let left = self.recursive_build_tree(2 * node_id);
            let right = self.recursive_build_tree(2 * node_id + 1);
            let hash = digest(left.clone() + &right);
            self.facts.insert(hash.clone(), (left, right));
            hash
        }
    }

    fn get_authentication_path(&self, leaf_id: u32) -> Vec<String> {
        assert!(leaf_id < self.data.len() as u32, "Invalid leaf_id");
        let mut decommitment = Vec::new();
        let mut cur = &self.root;
        let node_id = leaf_id + self.data.len() as u32;

        for bit in format!("{:b}", node_id).chars().skip(1) {
            let (left, right) = self.facts.get(cur).unwrap();

            if bit == '0' {
                decommitment.push(right.clone());
                cur = left;
            } else {
                decommitment.push(left.clone());
                cur = right;
            }
        }
        decommitment
    }
}

pub fn verify_decommitment(
    leaf_id: u32,
    leaf_data: FieldElement,
    decommitment: Vec<String>,
    root: String,
) -> bool {
    let leaf_num = 2_u32.pow(decommitment.len() as u32);
    let node_id = leaf_id + leaf_num;
    let mut cur = digest(leaf_data.to_string());
    let bin_node_id: Vec<char> = format!("{:b}", node_id).chars().collect();
    for (bit, auth) in bin_node_id
        .iter()
        .skip(1)
        .rev()
        .zip(decommitment.iter().rev())
    {
        cur = if *bit == '0' {
            digest(cur.clone() + &auth)
        } else {
            digest(auth.clone() + &cur)
        };
    }
    cur == root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_merkle_tree() -> (MerkleTree, Vec<String>) {
        let data = vec![
            FieldElement::new(1),
            FieldElement::new(2),
            FieldElement::new(3),
            FieldElement::new(4),
        ];
        let tree = MerkleTree::new(data);
        let path = tree.get_authentication_path(2);
        (tree, path)
    }

    #[test]
    fn test_merkle_tree_valid() {
        let (tree, path) = setup_merkle_tree();
        assert!(verify_decommitment(
            2,
            FieldElement::new(3),
            path,
            tree.root
        ));
    }

    #[test]
    fn test_merkle_tree_invalid_content() {
        let (tree, path) = setup_merkle_tree();
        assert!(!verify_decommitment(
            2,
            FieldElement::new(4),
            path,
            tree.root
        ));
    }

    #[test]
    fn test_merkle_tree_invalid_decommitment() {
        let (tree, path) = setup_merkle_tree();
        let mut invalid_path = path.clone();
        invalid_path[0] = "invalid".to_string();
        assert!(!verify_decommitment(
            2,
            FieldElement::new(3),
            invalid_path,
            tree.root
        ));
    }
}
