# nets

**nets** is a deterministic, competitive protocol where autonomous agents compete on verifiable tasks under selection pressure.

There is:

* no shared training
* no global model
* no gradient exchange
* no cooperative learning

Agents compete. Results are replayed. Capital flows to winners. Losers lose stake or disappear.

Blockchains are used **only** for commitment, settlement, and enforcement.
All execution happens off chain.

---

## Core Idea

Intelligence scales through **selection**, not synchronization.

Agents that perform well earn capital.
Capital controls access to harder leagues and more evaluation capacity.
Poor agents lose capital or are disqualified.

There is no coordination between agents. Evolution replaces training loops.

---

## What Exists (v0)

nets is not a concept. The following is implemented and working:

* Deterministic task environments
* Untrusted WASM agents
* Sandbox execution with fuel limits
* Replay verification
* Merkle-committed execution traces
* Multi-match leagues
* Stake slashing on replay mismatch
* Persistent balances and disqualification state
* On-chain commitment interface (adapter-based)

---

## Architecture Overview

### Systems

A **System** defines a deterministic task.

Requirements:

* deterministic execution
* cheap replay
* finite termination
* explicit scoring

Example systems:

* Snake (implemented)
* Board games
* Deterministic simulations
* Prediction tasks

---

### Agents

An **Agent** is a black-box decision function.

In nets-core:

* Agents are submitted as WASM binaries
* No filesystem, networking, or randomness
* Fixed fuel budget
* Single required export:

```c
extern "C" fn decide(input: u64) -> u64
```

Agents may be adversarial. The system does not trust them.

---

### Matches

A **Match** consists of:

* one system
* one agent
* deterministic execution from reset to termination

The output is a `(agent_id, score)` pair.

All matches are replayable.

---

### Verification & Slashing

Every match can be recomputed independently.

If recomputed score ≠ claimed score:

* stake is slashed
* agent is immediately disqualified
* remaining matches are skipped

There is no partial trust and no tolerance for mismatch.

---

### Traces & Commitments

During execution, a trace is recorded:

* hash(observation)
* hash(action)
* step index

Trace steps are aggregated into a Merkle tree.

Only the Merkle root is posted on chain.
Any step can be proven via inclusion proof.

---

### Leagues

Agents compete in **leagues**.

Each league defines:

* minimum stake requirement
* reward multiplier
* number of matches per round

Leagues run multiple matches per agent to reward consistency, not luck.

Capital determines which leagues an agent may enter.

---

### Settlement

After a league round:

* agents are ranked by total score
* rewards are distributed deterministically
* balances are updated
* balances persist across restarts

Capital controls future opportunity.

---

## Why This Exists

Blockchains are bad at:

* training models
* running simulations
* low-latency execution

They are good at:

* enforcing rules
* anchoring commitments
* settling outcomes
* coordinating adversaries

nets uses blockchains **only** where they add value.

---

## Non-Goals

nets explicitly does **not** support:

* on-chain training
* shared models
* federated learning
* gradient aggregation
* cooperative intelligence
* low-latency inference

If your system requires fast synchronization, nets is the wrong tool.

---

## Philosophy

nets treats intelligence as a **competitive market**, not a collective brain.

If an agent is good, it wins.
If it wins, it earns.
If it earns, it scales.

Everything else is implementation detail.