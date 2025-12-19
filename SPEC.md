# nets Protocol Specification (v0)

## 1. Overview

**nets** is a decentralized competitive system where autonomous agents (neural networks or deterministic programs) compete on deterministic tasks.

There is:

* no shared training
* no global model
* no gradient aggregation
* no coordination between agents

Selection happens purely through **measurable performance** under **replayable rules**.

Blockchains are used only for:

* commitment
* dispute resolution
* settlement

All execution happens off chain.

---

## 2. Core Entities

### 2.1 System

A **System** defines a task environment.

A valid system MUST satisfy:

* deterministic execution
* deterministic initial state
* cheap replay
* finite termination

Formally, a System exposes:

* `Observation`
* `Action`
* `Score`

And the following functions:

* `reset()`
* `observe() -> Observation`
* `step(Action)`
* `is_done() -> bool`
* `score() -> Score`
* `id() -> String`

Given the same sequence of actions, a System MUST always produce the same score.

---

### 2.2 Agent

An **Agent** is a black box decision function.

Formally, an agent implements:

```
decide(observation) -> action
```

Properties:

* Agents are untrusted
* Agents do not share state
* Agents may be adversarial
* Agents may not access IO or randomness unless encoded deterministically

In nets-core, agents are submitted as **WASM binaries**.

---

### 2.3 WASM Execution Model

Agents are executed inside a WASM sandbox with:

* no filesystem
* no networking
* no syscalls
* no randomness
* fixed fuel budget

The only required export is:

```
extern "C" fn decide(input: u64) -> u64
```

Failure modes:

* trap
* fuel exhaustion
* invalid export

Any failure results in the agent losing the match.

---

## 3. Match Execution

A **Match** is defined as:

* one System instance
* one Agent
* deterministic execution from reset until termination

Execution steps:

1. System reset
2. Loop until `is_done()`:

   * observe
   * agent decides
   * system steps
3. Final score recorded

The output is a `MatchResult`:

```
(agent_id, score)
```

---

## 4. Replay Verification

All matches MUST be replayable.

Given:

* System definition
* Agent binary
* Initial seed (if applicable)

Any node can recompute the match and MUST obtain the same score.

If recomputation yields a different score, the original result is invalid.

---

## 5. Trace and Merkle Commitment

During execution, a **trace** is recorded:

Each step records:

* step index
* hash(observation)
* hash(action)

Each step is hashed, and all step hashes are aggregated into a **Merkle tree**.

The Merkle root is the canonical commitment to the execution.

Properties:

* Any step can be proven via inclusion proof
* Full execution does not need to be posted on chain
* Fraud proofs are logarithmic size

---

## 6. Leagues

A **League** defines a competitive tier.

Each league has:

* `id`
* minimum stake requirement
* reward multiplier
* system assignment

Agents may only enter leagues for which they satisfy the stake requirement.

---

## 7. Multi-Match League Execution

In a league round:

* Each eligible agent plays **N matches**
* Total score is the sum of match scores
* Rankings are based on total score

This prevents single-match luck and enforces consistency.

---

## 8. Slashing Rules

Slashing is **mechanical and deterministic**.

An agent is slashed if:

* replayed score ≠ claimed score

On slashing:

* a fixed stake amount is removed
* the agent is immediately **disqualified** from the league round
* remaining matches are skipped
* score is frozen

There is no partial trust or soft failure.

---

## 9. Disqualification

Disqualified agents:

* cannot continue in the current league
* remain disqualified across restarts
* must re-enter via future rounds (implementation dependent)

Disqualification state is persisted.

---

## 10. Settlement

After league completion:

* agents are ranked by total score
* rewards are distributed according to rank and league multiplier
* rewards increase agent capital
* capital controls future league eligibility

Settlement logic is deterministic.

---

## 11. Persistence

nets-core persists:

* agent balances
* disqualification state

Persistence is local, deterministic, and restart-safe.

No global database is required.

---

## 12. On-Chain Interface

nets does **not** execute on chain.

The chain is used only for:

* posting Merkle commitments
* anchoring match results
* enforcing settlement

A chain integration MUST support:

* posting a commitment object
* settling payouts

The chain never:

* runs matches
* sees agent code
* evaluates models

---

## 13. Security Model

nets assumes:

* adversarial agents
* adversarial hosts
* honest verifiers

Security comes from:

* determinism
* replayability
* cryptographic commitments
* economic penalties

There is no reliance on honesty or cooperation.

---

## 14. Non-Goals

nets explicitly does NOT attempt to provide:

* on-chain training
* shared models
* federated learning
* gradient exchange
* low-latency inference
* cooperative intelligence

If coordination is required, nets is the wrong system.

---

## 15. Philosophy

nets treats intelligence as a **competitive process**, not a collective artifact.

If an agent is good, it wins.
If it wins, it earns.
If it earns, it scales.

Everything else is implementation detail.