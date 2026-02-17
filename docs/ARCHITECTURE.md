# Architecture

## Components
1) **Solana Program (Anchor/Rust)**
- Cluster: Devnet
- Program ID: `3aNa219G1VwZq2gC8NRL9NRGXCbrhnhqivHDpmZS8TeA`
- Purpose: store verifiable primitives for on-chain reputation.

2) **Frontend (Static Web)**
- Purpose: wallet connect + UX shell for reputation experience.
- Hosting: Surge.

3) **FairScale Integration Layer (Planned / In progress)**
To avoid exposing API keys in the browser, FairScore should be fetched via a small backend/proxy:
- `GET /fairscore?wallet=<pubkey>`
- Returns: score + tier + metadata

## Data flow (target)
Wallet -> Frontend -> (FairScore Proxy) -> FairScale API -> Frontend gating / UI
Wallet -> Solana Program -> On-chain data -> FairScore computation signals (FairScale)

## Why Solana
- Low fees
- High throughput
- Great UX for consumer-facing reputation products
