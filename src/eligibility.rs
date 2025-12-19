use crate::ledger::Ledger;
use crate::league::League;

pub fn can_enter(
    ledger: &Ledger,
    agent_id: &str,
    league: &League,
) -> bool {
    ledger.balance(agent_id) >= league.min_stake
}

pub fn highest_eligible_league<'a>(
    ledger: &Ledger,
    agent_id: &str,
    leagues: &'a [League],
) -> Option<&'a League> {
    leagues
        .iter()
        .filter(|l| can_enter(ledger, agent_id, l))
        .max_by_key(|l| l.min_stake)
}
