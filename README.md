# TaskEscrow

Trustless escrow payments for freelancers using Stellar Soroban.

---

## 🚀 What This Project Is

TaskEscrow is a decentralized escrow system built on the Stellar network using Soroban smart contracts. It ensures freelancers get paid fairly by requiring clients to lock funds upfront before work begins.

This eliminates the need for trust between strangers and removes the risk of non-payment.

---

## ❗ Problem It Solves

A freelance graphic designer in the Philippines loses ₱3,000 after delivering completed work to a client who disappears without paying.

This is a common issue across freelance platforms:
- No guaranteed payment
- Weak enforcement
- High trust friction between client and freelancer

---

## 💡 How It Works

TaskEscrow introduces a simple, trustless workflow:

1. *Create Escrow*
   - Client defines the job and payment amount
   - Smart contract stores the agreement

2. *Deposit Funds*
   - Client deposits USDC into the contract
   - Funds are locked on-chain

3. *Freelancer Completes Work*
   - Freelancer submits work off-chain
   - Calls contract to mark completion

4. *Mutual Approval*
   - Freelancer confirms submission
   - Client approves work

5. *Payment Released*
   - Smart contract automatically releases USDC to freelancer

---

## 🔗 Deployed Smart Contract

- *Network: Stellar Testnet  
- *Contract ID:CA43CQXEJMR2ZGUMTQRPGQEOHSV4ZQNEP7EH7HERRJKX7MAPHPZ3RY4V

https://stellar.expert/explorer/testnet/tx/af1e80ef0144f7d72afee97a78c07882e3d605339f4e9f5a1abeda6ecfed83b5

https://lab.stellar.org/smart-contracts/contract-explorer?$=network$id=testnet&label=Testnet&horizonUrl=https:////horizon-testnet.stellar.org&rpcUrl=https:////soroban-testnet.stellar.org&passphrase=Test%20SDF%20Network%20/;%20September%202015;&smartContracts$explorer$contractId=CA43CQXEJMR2ZGUMTQRPGQEOHSV4ZQNEP7EH7HERRJKX7MAPHPZ3RY4V;;
