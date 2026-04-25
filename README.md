# TaskEscrow

Trustless escrow payments for freelancers using Stellar.

---

## Problem

A Filipino freelancer loses payment after delivering work to a client who disappears.

## Solution

TaskEscrow locks USDC in a Soroban smart contract and only releases payment when both freelancer and client approve completion.

---

## Timeline

Day 1: Escrow contract (create + deposit)  
Day 2: Approval + release logic  
Day 3: Frontend UI  
Day 4: Wallet integration  
Day 5: Demo polish  

---

## Stellar Features Used

- Soroban smart contracts  
- USDC transfers  
- Clawback / Compliance  

---

## Vision

Make freelancing in SEA trustless, reducing scams and unlocking more digital work opportunities.

---

## Prerequisites

- Rust (latest stable)
- Soroban CLI (v22+)

---

## Build

```bash
soroban contract build
Test
Copy code
Bash
cargo test
Deploy
Copy code
Bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/task_escrow.wasm \
  --source YOUR_ACCOUNT
Sample CLI
Create Escrow
Copy code
Bash
soroban contract invoke \
  --id CONTRACT_ID \
  --fn create_escrow \
  --arg escrow_id=1 \
  --arg client=G... \
  --arg freelancer=G... \
  --arg amount=1000
Deposit
Copy code
Bash
soroban contract invoke \
  --id CONTRACT_ID \
  --fn deposit \
  --arg escrow_id=1
Release Payment
Copy code
Bash
soroban contract invoke \
  --id CONTRACT_ID \
  --fn release_payment \
  --arg escrow_id=1
License
MIT
