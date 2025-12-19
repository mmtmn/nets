use crate::ledger::Ledger;

pub fn evolve(
    ledger: &Ledger,
    agent_id: &str,
) -> u64 {
    let capital = ledger.balance(agent_id);

    // Capital directly maps to future capacity
    capital / 10
}
