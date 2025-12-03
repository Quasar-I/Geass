use sha2::{Digest, Sha256};

fn main() {
    let transactions = vec![
        "Tx1: Induj pays 5 BTC",
        "Tx2: IIT Jammu pays Tuition",
        "Tx3: Buy Groceries",
        "Tx4: Travel fund",
        "Tx5: Gym membership",
    ];

    println!("--- Geass Merkle Verifier ---");
    println!("Total Transactions: {}", transactions.len());

    let mut current_layer: Vec<Vec<u8>> = transactions
        .iter()
        .map(|tx| hash_data(tx.as_bytes()))
        .collect();

    while current_layer.len() > 1 {
        let mut next_layer = Vec::new();

        for pair in current_layer.chunks(2) {
            let left = &pair[0];

            let right = if pair.len() == 2 { &pair[1] } else { &pair[0] };

            let mut combined = Vec::new();
            combined.extend_from_slice(left);
            combined.extend_from_slice(right);

            let new_hash = hash_data(&combined);
            next_layer.push(new_hash);
        }

        current_layer = next_layer;
    }

    let merkle_root = hex::encode(&current_layer[0]);
    println!("Final Merkle Root: {}", merkle_root);
}

fn hash_data(input: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}
