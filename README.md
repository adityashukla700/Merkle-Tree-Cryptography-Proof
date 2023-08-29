# Merkle-Tree-Cryptography-Proof

### Approach for Solving Problem
The main idea for the cryptographic proof is to verify that a node has been removed successfully without revealing the data itself. To solve this, whenever we have to remove a node from merkle tree we will only remove the leaf node and maintain rest of the hashes as it is, which will used for generating the proof.

Furthermore, once the node is removed, we will generate and store the path of siblings of that node from leaf node to the root node and use that to verify that the node has been deleted.

To verify that the node has instead been removed, we will store the removed leaf node (which is hash of the element) and then from the help of the siblings which was stored in path we will verify by again calculate the hashes. If the resultant hash is equal to the root hash, we can be sure that the node has been removed successfully and path arrray will be the proof of the solution.

### Code
In the program, I have create a  MerkleTree struct which stores merkle tree level by level starting from the leaf nodes to the root node.For this I have used Vec<Vec<String>>.

Inside impl MerkleTree all the necessary methods have been created.

**Important Points:**
1. Inside the construct_tree function we are recursively creating the merkle tree. 

2. The case where the number of elements/nodes are not in power of 2 has also been handled appropriately by making left_hash = right_hash. Hence, the code works for all valid inputs.
3.  The sibling is right sibling or left sibling has also been stored in proof_path. We are calculating the sibling index by xoring deleted_element_index with 1 in generate_proof function.

4.  This information is useful when we are recalculating the hashes for verification purposes, in verify proof function. 
 
### Running Code

To run the code, clone the repository and move to the repository directory.
Run command: cargo run
This will run the main.rs file.
