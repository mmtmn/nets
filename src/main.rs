use std::fs;

use nets::{
    system::System,
    agent::Agent,
    snake::{SnakeSystem, Dir},
    wasm_agent::WasmAgent,
    r#match::run_match,
    match_trace::run_match_with_trace,
    ledger::Ledger,
    evolution::evolve,
    league::League,
    league_state::LeagueState,
    slashing::{slash_on_mismatch, SlashConfig},
    persist,
    mock_chain::MockChain,
    onchain_commit::post_merkle,
};

/* ------------------------------
   Action decoding
-------------------------------*/

fn u64_to_dir(v: u64) -> Dir {
    match v % 4 {
        0 => Dir::Up,
        1 => Dir::Down,
        2 => Dir::Left,
        _ => Dir::Right,
    }
}

/* ------------------------------
   WASM Snake Adapter
-------------------------------*/

struct SnakeWasmAgent {
    inner: WasmAgent,
}

impl Agent<
    <SnakeSystem as System>::Observation,
    <SnakeSystem as System>::Action,
> for SnakeWasmAgent {
    fn id(&self) -> String {
        self.inner.id.clone()
    }

    fn decide(
        &mut self,
        obs: <SnakeSystem as System>::Observation,
    ) -> <SnakeSystem as System>::Action {
        let ((hx, hy), (ax, ay), _) = obs;

        let packed =
            ((hx as u64 & 0xFF) << 24)
            | ((hy as u64 & 0xFF) << 16)
            | ((ax as u64 & 0xFF) << 8)
            | (ay as u64 & 0xFF);

        u64_to_dir(self.inner.decide(packed))
    }
}

/* ------------------------------
   Main
-------------------------------*/

fn main() {
    println!("nets :: full league with slashing + persistence + chain");

    let mut ledger = Ledger::new();
    let mut league_state = LeagueState::default();
    persist::load("state.json", &mut ledger, &mut league_state);

    let mut chain = MockChain::new();

    let wasm_a = fs::read("agents/agent_a.wasm").unwrap();
    let wasm_b = fs::read("agents/agent_b.wasm").unwrap();

    let mut agents = vec![
        SnakeWasmAgent {
            inner: WasmAgent::load("agent_A".into(), &wasm_a).unwrap(),
        },
        SnakeWasmAgent {
            inner: WasmAgent::load("agent_B".into(), &wasm_b).unwrap(),
        },
    ];

    let system = SnakeSystem::new(10, 10, 300);
    let league = League::bronze();
    let slash_cfg = SlashConfig { slash_amount: 50 };

    for agent in agents.iter_mut() {
        if league_state.is_disqualified(&agent.id()) {
            continue;
        }

        // Claimed result
        let claimed = run_match(system.clone(), agent);

        // Deterministic replay
        let recomputed = run_match(system.clone(), agent);

        if slash_on_mismatch(
            &mut ledger,
            &agent.id(),
            &claimed,
            &recomputed,
            &slash_cfg,
        ) {
            league_state.disqualify(&agent.id());
            continue;
        }

        // Trace + Merkle commitment
        let trace = run_match_with_trace(system.clone(), agent);
        let root = trace.merkle.root();

        post_merkle(
            &mut chain,
            &league,
            &system.id(),
            &agent.id(),
            root,
            claimed.score as i64,
        );

        ledger.credit(&agent.id(), 100);
    }

    persist::save("state.json", &ledger, &league_state);

    println!("\nfinal balances:");
    for agent in ["agent_A", "agent_B"] {
        println!(
            "{} balance={} capacity={}",
            agent,
            ledger.balance(agent),
            evolve(&ledger, agent)
        );
    }

    println!("\nstatus: production-grade league complete");
}
