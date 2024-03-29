use bitcoin::{
    secp256k1::{All, Secp256k1},
    taproot::TaprootSpendInfo,
    ScriptBuf, TxOut,
};
use serde::{Deserialize, Serialize};

use crate::model::Utxo;

#[derive(Debug, Deserialize, Serialize)]
pub struct PayloadWrapper {
    pub args: Payload,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    pub bitworkc: String,
    pub mint_ticker: String,
    pub nonce: u64,
    pub time: u64,
}

#[derive(Clone, Debug)]
pub struct TransactionData {
    pub secp: Secp256k1<All>,
    pub satsbyte: u64,
    // bitwork_info_commit: BitworkInfo,
    pub bitwork_info_commit: String,
    pub additional_outputs: Vec<TxOut>,
    pub reveal_script: ScriptBuf,
    pub reveal_spend_info: TaprootSpendInfo,
    pub fees: Fees,
    pub funding_utxo: Utxo,
}

#[derive(Clone, Debug)]
pub struct Fees {
    pub commit: u64,
    pub commit_and_reveal_and_outputs: u64,
    pub reveal_and_outputs: u64,
}
