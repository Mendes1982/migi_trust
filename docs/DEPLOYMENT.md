# Deployment

## Program deployment (Devnet)
- Program ID: `3aNa219G1VwZq2gC8NRL9NRGXCbrhnhqivHDpmZS8TeA`
- Deployed via Anchor.

## Frontend deployment (Surge)
- Production: http://migi-trust-hub-2026.surge.sh

### Update frontend
From the frontend directory:
- `surge . --domain migi-trust-hub-2026.surge.sh`

## Operational notes
- Never commit private keys (`id.json`) or seed phrases.
- Keep build artifacts out of Git (`target/`).
