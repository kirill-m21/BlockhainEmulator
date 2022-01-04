use chrono::Utc;
use num_bigint::{BigInt, RandomBits};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::collections::{LinkedList, VecDeque};

#[derive(Clone, Debug, PartialEq)]
struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

#[derive(Clone, Debug, PartialEq)]
struct Block {
    header: Header,
    tr_data: Transaction,
    hash: String,
    prev_hash: String,
}

#[derive(Clone, Debug, PartialEq)]
struct Blockchain {
    chain: LinkedList<Block>,
    tr_queue: VecDeque<Transaction>,
}

impl Blockchain {
    //ctor
    fn new() -> Blockchain {
        let mut block_chain_tmp: LinkedList<Block> = LinkedList::new();
        let tr_queue_tmp: VecDeque<Transaction> = VecDeque::new();

        let genesis = Block {
            header: Header {
                timestamp: (String::from("0")),
                nonce: (BigInt::from(0)),
            },
            tr_data: Transaction {
                from: String::from("Satoshi"),
                to: String::from("GENESIS"),
                amount: 100_000_000,
            },
            hash: Utc::now().timestamp().to_string(),
            prev_hash: String::from("0"),
        };

        block_chain_tmp.push_back(genesis);

        Blockchain {
            chain: block_chain_tmp,
            tr_queue: tr_queue_tmp,
        }
    }

    fn new_transaction(&mut self, tr_amount: usize) {
        for _iter in 0..tr_amount {
            self.tr_queue.push_back(Transaction {
                from: String::from("sender"),
                to: String::from("receiver"),
                amount: rand::thread_rng().gen_range(0..100_000_000 as u64),
            });
        }
    }

    fn mint(&mut self) {
        if self.tr_queue.is_empty() {
            println!("No transactions queued!");
            return;
        }

        let mut new_block = Block {
            header: Header {
                timestamp: String::from("0"),
                nonce: BigInt::from(0),
            },
            tr_data: self.tr_queue.front().unwrap().clone(),
            hash: String::from(""),
            prev_hash: self.chain.back().unwrap().clone().hash,
        };

        loop {
            let mut sum_string: String = "".to_owned();

            sum_string.push_str(&new_block.prev_hash);
            sum_string.push_str(&new_block.tr_data.from);
            sum_string.push_str(&new_block.tr_data.to);
            sum_string.push_str(&new_block.tr_data.amount.to_string());

            new_block.header.nonce = rand::thread_rng().sample(RandomBits::new(256));
            sum_string.push_str(&new_block.header.nonce.to_string());

            let mut hasher = Sha256::new();
            hasher.update(sum_string);

            let check_hash = format!("{:x}", hasher.finalize());

            if check_hash.chars().filter(|&c| c == '1').count() >= 6 {
                new_block.hash = check_hash;
                new_block.header.timestamp = Utc::now().timestamp().to_string();
                break;
            }
        }
        self.chain.push_back(new_block);
        self.tr_queue.pop_front();
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Header {
    timestamp: String,
    nonce: BigInt,
}
fn main() {
    let mut blnch: Blockchain = Blockchain::new(); //override ctor?

    blnch.new_transaction(3); //queues transactions with a random amount

    blnch.mint();
    blnch.mint();

    println!("{:#?}", blnch.chain);
    println!("--------------------");
    println!("{:#?}", blnch.tr_queue);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_blockchain_test() {
        let blnch: Blockchain = Blockchain::new();
        let genesis = Block {
            header: Header {
                timestamp: (String::from("0")),
                nonce: BigInt::from(0),
            },
            tr_data: Transaction {
                from: String::from("Satoshi"),
                to: String::from("GENESIS"),
                amount: 100_000_000,
            },
            hash: Utc::now().timestamp().to_string(),
            prev_hash: String::from("0"),
        };
        assert_eq!(blnch.chain.front().unwrap(), &genesis);
    }

    #[test]
    fn new_transaction_test() {
        let mut blnch: Blockchain = Blockchain::new();
        blnch.new_transaction(3);
        assert_eq!(blnch.tr_queue.len(), 3);
    }

    #[test]
    fn mint_test() {
        let mut blnch: Blockchain = Blockchain::new();
        blnch.tr_queue.push_back(Transaction {
            from: String::from("Sender"),
            to: String::from("Receiver"),
            amount: 21u64,
        });
        blnch.mint();
        assert_eq!(blnch.chain.len(), 2);
        assert!(blnch.chain.back().unwrap().clone().hash.chars().filter(|&c| c == '1').count() >= 6);
    }
}
