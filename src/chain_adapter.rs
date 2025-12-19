use crate::chain::{MatchCommitment, SettlementTx};

pub trait ChainAdapter {
    fn post_commitment(&mut self, commitment: &MatchCommitment) -> [u8; 32];
    fn settle(&mut self, tx: SettlementTx);
}
