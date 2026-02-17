# FairScale / FairScore Integration (Core Logic)

## Why FairScore matters
MIGI Trust Hub is a trust layer for the IT freelance market. FairScore turns “trust” into a programmable primitive:
- reduces hiring fraud and CV inflation
- improves allocation quality (who gets access to good opportunities)
- enables risk-managed flows (limits / escrow / step-up verification)

## Where FairScore is used (core, not decorative)
1) Reputation gating (tiers)
- Tier 0–1: browse only
- Tier 2–3: can create a service profile + apply to jobs
- Tier 4+: premium access (featured listing, higher limits, faster payouts)

2) Risk management
- Low FairScore => additional friction (manual review / smaller limits)
- High FairScore => fast-track

3) Dynamic rewards
- High FairScore wallets earn boosts (higher visibility + faster reputation growth)

## Implementation strategy (secure)
Do NOT call FairScale API from the browser.
Use a backend proxy:
- GET /fairscore?wallet=<pubkey>
- Proxy calls FairScale API using secret in .env
- Returns score + tier only (sanitized)
