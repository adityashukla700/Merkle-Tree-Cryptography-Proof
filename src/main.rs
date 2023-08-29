use sha2::{Sha256,Digest};


struct MerkleTree {
    levels: Vec<Vec<String>>,
}

impl MerkleTree {
    fn new() -> Self {
       MerkleTree { levels: Vec::new() }
    }

    fn construct_tree(&mut self, data: Vec<String>) {
        if data.is_empty() {
            return;
        }
        self.levels.push(data.clone());

        if data.len() == 1 {
            return;
        }

        let mut new_level = Vec::new();
        for i in (0..data.len()).step_by(2) {
            let left_hash = data[i].clone();
            let right_hash = if i + 1 < data.len() {
                data[i + 1].clone()
            } else {
                left_hash.clone()
            };
            let combined_hash = format!("{:x}", Sha256::digest(&(left_hash + &right_hash).as_bytes()));
            new_level.push(combined_hash);
        }

        self.construct_tree(new_level);
    }

    fn generate_proof(&mut self, mut index_to_delete: usize) -> (String, Vec<(String, usize)>) {
        let mut proof_path: Vec<(String, usize)> = Vec::new();
        let deleted_element_hash = self.levels[0][index_to_delete].clone();
        self.levels[0][index_to_delete]=String::from("deleted");

        for level in self.levels.iter().take(self.levels.len() - 1) {
            let sibling_index = index_to_delete ^ 1;
            proof_path.push((level[sibling_index].clone(), sibling_index));
            index_to_delete /= 2;
        }

        (deleted_element_hash, proof_path)
    }

    fn verify_proof(&self, deleted_element_hash: &str, proof_path: &[(String, usize)], merkle_tree_root: &str) -> bool {
        let mut del_hex = deleted_element_hash.to_string();
        for sibling in proof_path {
            if sibling.1 == 0 {
                del_hex = format!("{:x}", Sha256::digest(&(sibling.0.clone() + &del_hex).as_bytes()));
            } else {
                del_hex = format!("{:x}", Sha256::digest(&(del_hex.clone() + &sibling.0).as_bytes()));
            }
        }
        del_hex == *merkle_tree_root
    }
    fn print_tree(&self) {
        println!("Merkle Tree: {:?}", self.levels);
    }
}

fn main() {
    // Initial data
    let initial_data = vec!["data1", "data2", "data3", "data4", "data5", "data6", "data7", "data8"];
    // let initial_data = vec!["data1", "data2", "data3", "data4", "data5", "data6"];
    // the code works fine in both cases whether the data/files numbers are in power of 2 or not.

    // Hashing the initial data
    let mut hashed_data = Vec::new();
    for s in &initial_data {
        let mut hasher = Sha256::new();
        hasher.update(s.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        hashed_data.push(hash);
    }

    // Constructing the Merkle Tree
    let mut merkle_tree = MerkleTree::new();
    merkle_tree.construct_tree(hashed_data.clone());

    // Printing the Merkle Tree
    merkle_tree.print_tree();

    // Index of element to be deleted
    let index_to_delete = 1;
    let (deleted_element_hash, proof_path) = merkle_tree.generate_proof(index_to_delete);
    
    

    // Retrieving the Merkle Tree root hash
    let merkle_tree_root = merkle_tree.levels.last().unwrap()[0].clone();
    println!("\nMerkle Tree Root: {}", merkle_tree_root);
    println!("\n Proof Path: {:?}", proof_path);

    // Verifying the proof
    let is_proof_valid = merkle_tree.verify_proof(&deleted_element_hash, &proof_path, &merkle_tree_root);
    if is_proof_valid {
        println!("\n Proof is valid and the element has been removed");
    } else {
        println!("\n Proof is invalid");
    }
    println!("\n Merkle Tree after deletion\n");
    merkle_tree.print_tree();

    
}