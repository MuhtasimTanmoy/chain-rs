use crypto::digest::Digest;
use crypto::sha2::Sha256;
use failure::format_err;
use log::error;
use serde::{Serialize,Deserialize};
use crate::blockchain::Blockchain;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub input: Vec<TXInput>,
    pub output: Vec<TXOutput>,
}

impl Transaction {

    pub fn new(from: &str, to: &str, amount: i32, bc: &Blockchain) -> Result<Transaction, failure::Error> {
        let mut input = Vec::new();
        let acc_v = bc.find_spendable_outputs(from, amount);
        if acc_v.0 < amount {
            error!("Not Enough balance");
            return Err(format_err!("Not Enough balance: current balance {}",acc_v.0));
        }

        for tx in acc_v.1 {
            for out in tx.1 {
                let txin = TXInput {
                    txid: tx.0.clone(),
                    vout: out,
                    script_sig: String::from(from),
                };
                input.push(txin);
            }
        }

        let mut output = vec![TXOutput {
            value: amount,
            script_pub_key: String::from(to),
        }];

        if acc_v.0 > amount {
            output.push(TXOutput {
                value: acc_v.0 - amount,
                script_pub_key: String::from(from),
            })
        }

        let mut tx = Transaction {
            id: String::new(),
            input,
            output,
        };
        tx.set_id()?;
        Ok(tx)
    }

    pub fn new_coinbase(to: String, mut data: String) -> Result<Transaction, failure::Error> {
        if data == String::from("") {
            data += &format!("Reward to '{}'", to);
        }
        let mut tx = Transaction {
            id: String::new(),
            input: vec![TXInput {
                txid: String::new(),
                vout: -1,
                script_sig: data,
            }],
            output: vec![TXOutput {
                value: 100,
                script_pub_key: to,
            }],
        };
        tx.set_id()?;
        Ok(tx)
    }

    fn set_id(&mut self) -> Result<(), failure::Error> {
        let mut hasher = Sha256::new();
        let data = bincode::serialize(self)?;
        hasher.input(&data);
        self.id = hasher.result_str();
        Ok(())
    }

    pub fn is_coinbase(&self) -> bool {
        self.input.len() == 1 && self.input[0].txid.is_empty() && self.input[0].vout == -1
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXInput {
    pub txid: String,
    pub vout: i32,
    pub script_sig: String,
}

impl TXInput {
    pub fn can_unlock_output_with(&self, unlocking_data: &str) -> bool {
        self.script_sig == unlocking_data
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutput {
    pub value: i32,
    pub script_pub_key: String,
}

impl TXOutput {
    pub fn can_be_unlock_with(&self, unlocking_data: &str) -> bool {
        self.script_pub_key == unlocking_data
    }
}