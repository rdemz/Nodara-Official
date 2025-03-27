# Nodara API Reference - Legendary Edition

This document provides a detailed reference for all RPC endpoints and SDK methods available within the Nodara BIOSPHÈRE QUANTIC network. It is designed to guide developers in integrating and interacting with the platform at the highest level of precision and reliability.

## RPC API Endpoints

### General Endpoints
- **nodara_getNetworkStatus**  
  **Description:** Retrieves the current network status, including node synchronization, block height, and performance metrics.  
  **Method:** `POST`  
  **Parameters:** None  
  **Response:** JSON object containing network details.

### Governance Endpoints
- **nodara_submitProposal**  
  **Description:** Submits a new governance proposal to update network parameters.  
  **Method:** `POST`  
  **Parameters:**  
  - `description`: Proposal description.  
  - `parameter`: Parameter to update.  
  - `value`: New value for the parameter.  
  **Response:** JSON object with proposal ID and status.

- **nodara_voteProposal**  
  **Description:** Casts a vote on an existing governance proposal.  
  **Method:** `POST`  
  **Parameters:**  
  - `proposal_id`: Unique proposal identifier.  
  - `vote`: Boolean indicating approval (`true`) or rejection (`false`).  
  **Response:** JSON object with vote status.

- **nodara_executeProposal**  
  **Description:** Executes an approved governance proposal.  
  **Method:** `POST`  
  **Parameters:**  
  - `proposal_id`: Unique proposal identifier.  
  **Response:** JSON object confirming execution.

## SDK Methods

### Base SDK (NodaraSDK)
- **sendRpcRequest(method: string, params: any[]): Promise<any>**  
  Sends an RPC request to the Nodara network.
- **getNetworkStatus(): Promise<any>**  
  Retrieves the current network status.

### Governance SDK (NodaraGovernanceSDK)
- **submitProposal(description: string, parameter: string, value: string): Promise<any>**  
  Submits a governance proposal.
- **voteProposal(proposalId: string, vote: boolean): Promise<any>**  
  Votes on a proposal.
- **executeProposal(proposalId: string): Promise<any>**  
  Executes a proposal.

*This API reference is intended for developers and system integrators who require detailed, low-level access to Nodara BIOSPHÈRE QUANTIC’s functionality.*
