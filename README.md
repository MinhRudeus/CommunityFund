# Community Fund

---

## Project Title

**Community Fund**

---

## Project Description

Community Fund is a decentralized crowdfunding and donation smart contract built on the Stellar blockchain using Soroban. It enables transparent community donations, proposal creation, and decentralized voting for fund allocation. Every transaction and proposal is recorded on-chain, ensuring accountability and trust without relying on a centralized organization.

---

## Project Vision

The vision of Community Fund is to create a transparent and secure funding platform where communities can collectively manage donations and make funding decisions through decentralized governance. By leveraging Soroban smart contracts, the project eliminates the need for intermediaries while ensuring every donation and vote is publicly verifiable.

---

## Key Features

- **Admin Initialization:** Initialize the contract with a designated administrator.
- **Community Donations:** Anyone can contribute funds to the community treasury.
- **Transparent Fund Tracking:** Track the total amount of donated funds on-chain.
- **Proposal Creation:** The administrator can create funding proposals for community projects.
- **Community Voting:** Members can vote in favor of or against each proposal.
- **One Vote Per User:** Prevent duplicate voting using on-chain storage.
- **Proposal Query:** Retrieve proposal details including title, receiver, requested amount, and voting results.
- **Public Transparency:** All proposals and donations are stored permanently on-chain.

---

## Usage Instructions

### 1. Initialize Contract

Deploy the contract and initialize it with an administrator.

```
init(admin)
```

---

### 2. Donate

Donate tokens to the community fund.

```
donate(from, token_address, amount)
```

---

### 3. Create Proposal

Admin creates a proposal requesting community funding.

```
create_proposal(
    title,
    receiver,
    amount
)
```

---

### 4. Vote

Community members vote for or against a proposal.

```
vote(
    proposal_id,
    voter,
    approve
)
```

---

### 5. Query Proposal

Retrieve proposal information.

```
get_proposal(proposal_id)
```

---

### 6. Query Total Fund

View the total amount of donations.

```
get_total()
```

---

### 7. Query Administrator

View the administrator address.

```
get_admin()
```

---

## Future Scope

- Proposal execution with automatic token transfer.
- Multi-admin governance.
- Voting deadline support.
- Minimum quorum requirement.
- Treasury management dashboard.
- Donation history for each contributor.
- Token-based voting power.
- Multi-token donation support (USDC, XLM Assets, etc.).
- Event logging and analytics dashboard.
- Web frontend integration.

---

## Technology Stack

- **Rust**
- **Soroban SDK**
- **Stellar Blockchain**
- **Soroban Smart Contracts**
- **On-chain Persistent Storage**

---

## Contribution

Contributions are welcome! Feel free to fork this project, improve the smart contract, add new governance features, or build a frontend interface for Community Fund.

---

## License

This project is licensed under the MIT License.

---

## Contract Detail

**Contract Name:** Community Fund

**Platform:** Stellar Soroban

**Language:** Rust

**Network:** Testnet

**Contract ID:**

```
<YOUR_CONTRACT_ID>
```

Replace `<YOUR_CONTRACT_ID>` with your deployed contract ID after deployment.

---