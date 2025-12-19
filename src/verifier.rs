use sha2::{Sha256, Digest};
use serde::Serialize;

use crate::system::System;
use crate::r#match::MatchResult;

pub fn hash_result<S>(result: &MatchResult<S>) -> String
where
    S: System,
    S::Score: Serialize,
{
    let mut hasher = Sha256::new();
    hasher.update(result.agent_id.as_bytes());
    hasher.update(bincode::serialize(&result.score).unwrap());
    format!("{:x}", hasher.finalize())
}

pub fn verify<S: System>(
    claimed: &MatchResult<S>,
    recomputed: &MatchResult<S>,
) -> bool {
    claimed.agent_id == recomputed.agent_id
        && claimed.score == recomputed.score
}
