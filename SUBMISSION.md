# Part 1 Theoretical Questions

## Part 1.1

Synchronization with a blockchain refers to the process of getting the most updated information on that Blockchain's state. Ethereum provides a great example of 1) the types of client nodes and 2) synchronization strategies that users can use in order to keep up with the network.

Types of Ethereum nodes:

- Full node:
  - Stores most blockchain data (very old data is pruned periodically, delegated to archive node's responsibility)
  - Participates in block validation by verifying all blocks and states
  - All states can be derived from a full node (although very old states are reconstructed from making requests to archive nodes)
  - Serve the network and provide data on request.
- Light node:
  - Downloads every block's headers (which contain summary of block) instead of entire block state.
  - Makes requests to full nodes for any other information not contained in the block header.
  - Independently verifies data they receive against the state root contained in the block header.
- Archive node:
  - Stores everything kept in the full node and builds an archive of historical states.

Synchronization Strategies:

- Full sync:
  - Downloads all blocks (headers, transactions, receipts) and generates the state of blockchain incrementally by executing every block from genesis. Can take days to weeks to process all transactions.
- Fast sync:
  - Downloads all blocks (headers, transactions, receipts), verifies all headers, downloads the state and verifies it against the headers. Takes only a few hours.
- Light sync:
  - Downloads all block headers, data, and verifies some randomly. Takes only a few minutes.
- Snap sync:
  - Uses dynamic snapshots served by peers to retrieve all account and storage data without downloading intermediate trie nodes and reconstructs the Merkle Trie locally.
  - Fastest sync strategy currently.
- Warp sync:
  - Uses regularly generated state snapshots which any peer can fetch these snapshots over the network, enabling a fast sync from this point.
  - Only available on OpenEthereum client nodes.
- Beam sync:
  - Like fast sync, but also downloads data needed to execute latest blocks.
  - Only available on Nethermind and Trinity client nodes.

Since light nodes do not download data and makes requests to full nodes only when the requested data is not contained within the already-downloaded block headers, running light nodes is the fastest way users can sync with the network which also consumes the least amount of storage (disk) space. The caveat with Ethereum specifically is that there used to only be the full node type, and light node types are a new addition to the set of node types and a rapidly undergoing development.

Users can additionally choose to use the light or snap sync strategies to quickly sync with the network within minutes, without sacrificing security. Although, the sync strategy that is available for use depends on which client node software is being used - for example, warp sync is only available to client nodes developed by OpenEthereum. However, users may not always optimize for the fastest way to sync - they might optimize for data availability and will need another type of node. For this reason, users should always check the open-source code for the exact software implementation that they are satisfied with.

References:

- https://ethereum.org/en/developers/docs/nodes-and-clients/

## Part 1.2

An epoch is a period that defines the block interval when validator elections are only allowed to occur. For Celo, an epoch is a day. Within each epoch, there will likely be more than 1 block.

Plumo builds 3 main improvements over basic PoS light clients:

- Epoch-based syncing
  - Light clients can reach the head of the chain by synchronizing only the epoch-transition blocks until they reach the latest blocks. Due to the SNARK constructions that prove validity of transitions over many epochs, light clients "hop" from SNARK to SNARK until they reach the latest proof, before donwloading the remaining epoch block headers to sync to the head of the chain. Epoch-transition blocks are the last block in each epoch, where the validator set **diff** for the next epoch is contained in the `extradata` field.
- Signature aggregation
  - Every block requires a quorum involving at least 2/3 signatures from validators before it is deemed valid to be committed to the blockchain. BLS signatures are used to aggregate multiple validator signatures over the same message into a single signature per each block's header.
- zk-SNARK proof per many epochs (specifically, 1 proof per 120 epochs)
  - Each SNARK proof attests that the validator set changes over some large number of epochs starting from a given initial validator set A to a given final validator set B. Effectively, Plumo's SNARK proof aggregates many epochs transitions into a single transition from A to B and is highly succinct.

There are 2 main approaches Plumo can construct its SNARK proofs:

1. Recursive

- Each proof proves the last proof's epoch transitions and the next 120 epochs transitions following the last proof.

2. "Simple Induction"

- Each proof proves 120 epochs transitions.

Currently, Plumo uses approach 2, but there is flexibility for it to implement the recursive approach in the future, depending on the need for it.

References:

- https://docs.zkproof.org/pages/standards/accepted-workshop3/proposal-plumo_celolightclient.pdf

## Part 1.3

Harmony blockchain uses Fast Byzantine Fault Tolerance (FBFT) consensus algorithm which is based on the Practical BFT algorithm. Celo clients use the [Istanbul BFT consensus algorithm](https://arxiv.org/pdf/2002.03613.pdf). Both are leader-based algorithms.

One difference between FBFT and IBFT is how they perform view / round changes to handle untimely communication or faulty leaders:

- FBFT in Harmony uses a fully synchronous algorithm to rotate leaders - specifically, the view ID is calculated based on the elapsed time of the last successfully committed block’s timestamp. Even though each validator is using their own local clock, the robustness of this approach relies on how likely it is that more than 2/3 validators maintain a relatively accurate local clock that's not drifting by a few seconds.
- IBFT in Celo is triggered by a timer that is available in each consensus instance. Specifically, if the algorithm has not made sufficient progress for a process to decide during some round, then the timer will eventually expire and that process advances to the next round and broadcasts a `ROUND-CHANGE` message.

Another difference between FBFT and IBFT is in their communication complexities:

- FBFT in Harmony runs a BLS multi-signature signing process to collect validators' votes in a fixed-size signature before broadcasting it, as opposed to asking all validators to broadcast their votes. Additionally, the use of BLS signature scheme allows for one round trip instead of two.
- IBFT in Celo involves each process to broadcast their votes during each of the `PRE-PREPARE`, `PREPARE` and `COMMIT` phases.

References:

- Harmony Whitepaper: https://harmony.one/whitepaper.pdf
- Istanbul BFT: https://arxiv.org/abs/2002.03613

# Part 2 Final Project

## Part 2.1

### [NEW] Anonymous DAO Compensation Tooling (eg. Anonymous Coordinape / Utopia)

Compensation tooling leveraging ZKP to prove that the contributor was compensated without revealing 1) the amount compensated or 2) the wallet address that was compensated.

Design overview:

- Need to first study the existing UX of Coordinape
- Smart contract for verification purposes.
- Arithmetic circuit for proof generation.
- Frontend application to generate proofs and perform proof verification against the verifier smart contract.

## Part 2.2

Evaluate and rank your three project ideas (and your new idea(s), if any) from last week in terms of implementation difficulty, potential user base, and the importance of ZK in the idea.

**Implementation Difficulty (1 hardest, 5 easiest)**

| Idea                      | Score |
| ------------------------- | ----- |
| Anonymous Coordinape      | 3     |
| ZK Token Gating           | 3     |
| ZK Cards Against Humanity | 5     |

**Potential User Base (1 least, 5 most)**

| Idea                      | Score |
| ------------------------- | ----- |
| Anonymous Coordinape      | 5     |
| ZK Token Gating           | 4     |
| ZK Cards Against Humanity | 1     |

**Importance of ZK in the idea (1 least, 5 most)**

| Idea                      | Score |
| ------------------------- | ----- |
| Anonymous Coordinape      | 4     |
| ZK Token Gating           | 4     |
| ZK Cards Against Humanity | 5     |

Total scores + rank:

1. Anonymous Coordinape (12)
2. ZK Token Gating (11)
3. ZK Cards Against Humanity (11)

## Part 2.3

I am more inclined to work on 1) Anonymous Coordinape since it ranked highest in the section above. It has a clear scope, there is an existing product in public to reference in terms of main UX, has few uncertainties remaining to be figured out, and can potentially be very impactful as there are many DAOs being created today and compensation can be sensitive data that users prefer to hide.

# Part 3 Frontend Assignment

## Part 3.1

The `ProvidePlugin` makes a package available as a variable in every module compiled through webpack. If webpack sees that variable used, it will include the given package in the final bundle. Hence, lines 8-10 in `next.config.js` says that, anywhere the `global` variable is used and is a free variable (not locally defined in any module), then include the `global` package and provide it to the modules that need it.

Then, lines 13-21 declare how to redirect module requests when normal resolving fails via the `resolve.fallback` configuration. Specifically, we are declaring not to include polyfills for any of the declared modules other than `assert`.

A polyfill is a piece of code (usually JavaScript on the Web) used to provide modern functionality on older browsers that do not natively support it.

References:

- https://developer.mozilla.org/en-US/docs/Glossary/Polyfill

## Part 3.3

YouTube Video Link: https://youtu.be/9OOIU4sGGOk
