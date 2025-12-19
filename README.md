# nets

**nets** is a decentralized market where neural networks compete instead of cooperating.

There is no shared training, no global gradients, and no pretending blockchains can do backprop. Each participant runs their own model locally and submits it to open competitions. Rewards flow purely from measurable performance.

The blockchain is used only for coordination, incentives, and settlement.

## Core Idea

Intelligence emerges from selection pressure, not synchronization.

Models compete on externally verifiable tasks. Winners earn capital. Capital buys more compute, more entries, or higher tier competitions. Losers fade out. Evolution replaces training loops.

## How It Works

1. **Tasks**

   * Deterministic, cheap to verify environments
   * Examples: Snake, Chess, board games, simulations, prediction tasks

2. **Participants**

   * Anyone can run a node
   * Nodes self host models or deterministic agents
   * No code or weights are shared by default

3. **Competition**

   * Models are matched in tournaments or leagues
   * Matches are run off chain
   * Scores or match proofs are posted on chain

4. **Settlement**

   * The chain verifies results
   * Rewards are distributed based on performance
   * Stake slashing applies to fraud or invalid submissions

5. **Evolution**

   * Earnings can be used to:

     * Buy more evaluation slots
     * Enter harder leagues
     * Fund retraining or new variants
   * No coordination required between models

## Why This Exists

Blockchains are bad at training neural networks. They are slow, adversarial, and expensive.

They are good at:

* Enforcing rules
* Settling markets
* Rewarding outcomes
* Coordinating strangers who do not trust each other

nets only uses blockchains for what they are good at.

## Initial Focus

* Simple environments first (Snake)
* Deterministic execution
* Transparent scoring
* Clear incentives

Complex domains like Chess come later, once the market mechanics are proven.

## Non Goals

* On chain training
* Shared gradients
* Global models
* Low latency inference
* Cooperative learning protocols

If your idea requires fast synchronization, nets is the wrong system.

## Philosophy

nets treats intelligence as a competitive market, not a collective brain.

If a model is good, it wins.
If it wins, it earns.
If it earns, it scales.

That’s it.
