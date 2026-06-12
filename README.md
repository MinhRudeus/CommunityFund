Community Fund Smart Contract
Project Title

Community Fund Smart Contract

Project Description

Community Fund is a decentralized, transparent smart contract platform designed to manage community donations and treasuries. Built using Rust and the Soroban SDK on the Stellar blockchain, it provides a highly secure vault where supporters can freely donate tokens (such as XLM or USDC). The contract strictly enforces access control, ensuring that only the verified administrator can withdraw funds to support operational costs, such as funding large-scale subtitle translation projects, maintaining community fanpages, or rewarding contributors.

Project Vision

The vision of the Community Fund is to bridge the trust gap between content creators and their communities. By leveraging blockchain technology, it offers a trustless and immutable treasury system. Supporters can transparently verify that their donations are safely stored, while community managers have a reliable, decentralized tool to securely aggregate and disburse funds for their creative endeavors and community-driven milestones.

Key Features

Immutable Initialization: Secure, one-time setup to assign the absolute administrator of the fund, preventing unauthorized takeovers.

Decentralized Donations: Anyone in the community can seamlessly contribute tokens to the smart contract using the donate function.

Strict Access Control: Absolute cryptographic enforcement ensuring that only the authorized admin wallet can trigger the withdraw function.

Transparent Treasury: All deposited funds, donor addresses, and withdrawal histories are publicly verifiable on the Stellar blockchain.

Asset Agnostic: Capable of handling various Soroban-compatible tokens, including native XLM and stablecoins.

Usage Instructions

Init: The creator deploys the contract and calls init with their wallet address to establish themselves as the Admin.

Donate: Community members invoke donate with their address, the specific token address, and the amount they wish to contribute to the fund.

Withdraw: The Admin invokes withdraw specifying the token, amount, and the destination address (e.g., paying a community translator or covering server costs).

Future Scope

Multi-Signature Administration: Require approvals from multiple core team members before a withdrawal can be executed.

Milestone-Based Funding: Lock funds until specific community goals (e.g., reaching a specific translation chapter or follower count) are met and voted upon.

Donor Leaderboard: On-chain recognition system to reward top contributors with special community roles or exclusive content access (NFTs).

Refund Mechanism: Implement a time-locked refund feature allowing donors to reclaim funds if a specific project goal is not achieved within a timeframe.

Event-Specific Vaults: Allow the admin to create sub-funds within the same contract for different concurrent projects.

Technology Stack

Rust & Soroban SDK: For writing highly efficient, safe, and secure smart contract logic.

Stellar Blockchain: For decentralized, low-cost, and lightning-fast state management.

Freighter / Stellar CLI: For wallet integration, cryptographic signing, and secure contract invocation.

Contribution

Community contributions are welcomed from blockchain developers, content creators, and community managers. Fork the repository and submit pull requests to assist in expanding the fund's features and front-end dashboard integrations.

License

This project is licensed under the MIT License.

Contract Detail

Network: Stellar Testnet

Contract ID: CB3QWXYLEKECMZF457EKDGUMDFDGUWV3R6AGWSA2GUBZSC27G5KT3TPV