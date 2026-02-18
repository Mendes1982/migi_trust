# System Architecture - MIGI Trust

### 1. Wallet Verification
We use **FairScale's FairScore** to gate access. 
- **Silver Tier:** Standard Access.
- **Gold Tier:** Premium verified status.

### 2. Reputation On-chain
Every interaction triggers the `completar_trabalho` instruction on our Anchor Program, minting reputation points to the user's profile.

### 3. Backend Proxy
A Node.js Proxy ensures the Frontend never exposes sensitive FairScale API keys, providing a secure bridge between the client and the infrastructure.
