use crate::{
    ledger::Ledger,
    r#match::MatchResult,
    system::System,
};

pub struct SlashConfig {
    pub slash_amount: u64,
}

pub fn slash_on_mismatch<S: System>(
    ledger: &mut Ledger,
    agent_id: &str,
    claimed: &MatchResult<S>,
    recomputed: &MatchResult<S>,
    cfg: &SlashConfig,
) -> bool {
    if claimed.score != recomputed.score {
        ledger.slash(agent_id, cfg.slash_amount);
        return true;
    }
    false
}
