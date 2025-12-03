# Geass

Geass is a lightweight Merkle Root Verifier implementation written in Rust. It takes a list of transaction strings, hashes them using SHA-256, and recursively pairs them to compute a single Merkle Root.

## How It Works


1.  **Input:** The program accepts a vector of transaction strings (e.g., "Tx1", "Tx2").
2.  **Leaf Hashing:** Each string is hashed individually using SHA-256 to form the initial layer of leaves.
3.  **Recursive Reduction:**
    * The list of hashes is processed in chunks of two.
    * **Odd Handling:** If a chunk has only one item (an odd remainder), that item is duplicated (concatenated with itself).
    * The pair is concatenated and hashed again to form a parent node.
4.  **Root:** This process repeats layer by layer until a single hash remains—the Merkle Root.
