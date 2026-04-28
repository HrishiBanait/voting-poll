## IPFS + Blockchain Image Storage Web App

Simple full-stack dApp that:

- **Uploads an image to a local IPFS daemon** (`http://127.0.0.1:5001/api/v0`)
- **Stores the resulting IPFS CID on a local Hardhat blockchain** via MetaMask
- **Displays transaction hash, block number, gas used, and sender address**

### 1. Prerequisites

- **Node.js** (LTS recommended)
- **MetaMask** browser extension
- **Local IPFS daemon** running on `127.0.0.1:5001`
  - You can start it with `ipfs daemon` (Go-IPFS / Kubo) or your preferred distribution.

### 2. Install dependencies

From the project root (`final`):

```bash
npm install
```

This installs:

- `hardhat` + `@nomicfoundation/hardhat-toolbox` for the blockchain side
- `http-server` to serve the frontend

### 3. Start local Hardhat network

In one terminal:

```bash
npm run hardhat:node
```

This starts a Hardhat JSON-RPC node at `http://127.0.0.1:8545` with several funded accounts.  
Add one of these accounts to MetaMask by importing its private key (shown in the Hardhat node output).

### 4. Deploy the Solidity contract

In another terminal (still in the project root):

```bash
npm run hardhat:deploy
```

This will:

- Compile the `ImageStorage` contract (`contracts/ImageStorage.sol`)
- Deploy it to the local Hardhat network
- **Generate `frontend/config.js`** with the deployed contract address:

```js
window.APP_CONFIG = {
  contractAddress: "0x...", // deployed ImageStorage address
  network: "localhost"
};
```

### 5. Start your local IPFS daemon

Ensure an IPFS node is running and exposing the HTTP API on `127.0.0.1:5001`:

```bash
ipfs daemon
```

You should be able to reach the API at `http://127.0.0.1:5001/api/v0`.

### 6. Serve the frontend

In a third terminal:

```bash
npm run frontend
```

This serves the `frontend` folder at `http://127.0.0.1:5173`.

Open that URL in your browser (the same one with MetaMask installed).

### 7. Using the dApp

1. **Connect MetaMask**
   - Click **“Connect MetaMask”**.
   - Approve the connection.
   - Make sure MetaMask is connected to **“Localhost 8545”** (the Hardhat node).
2. **Upload image & store hash**
   - Choose an image file.
   - Click **“Upload & Store Hash”**.
   - The app will:
     - Upload the image bytes to IPFS via `ipfs-http-client` (browser UMD build).
     - Receive the **CID** from IPFS.
     - Call the `storeHash(string _hash)` function on the `ImageStorage` contract using `ethers.js`.
3. **View transaction details**
   - After confirmation, the UI shows:
     - **Transaction hash**
     - **Block number**
     - **Gas used**
     - **Sender address**
     - **IPFS CID**

### 8. Code overview

- **Solidity contract**: `contracts/ImageStorage.sol`
  - `storeHash(string memory _hash)`:
    - Appends `_hash` to a per-user array in a `mapping(address => string[])`
    - Emits `HashStored(address user, string hash, uint256 timestamp)`
  - `getHashes(address user)` to read back all hashes for a user.

- **Hardhat config**: `hardhat.config.js`
  - Uses `@nomicfoundation/hardhat-toolbox`
  - Networks:
    - `hardhat` (in-memory)
    - `localhost` at `http://127.0.0.1:8545`

- **Deploy script**: `scripts/deploy.js`
  - Deploys `ImageStorage` to `localhost`
  - Writes `frontend/config.js` with `window.APP_CONFIG.contractAddress`

- **Frontend** (`frontend/`)
  - `index.html`
    - Simple UI: file input, connect button, and upload button
    - Loads:
      - `./config.js` (generated)
      - `ethers` v6 UMD build (via CDN, exposes `window.ethers`)
      - `ipfs-http-client` UMD build (via CDN, exposes `window.IpfsHttpClient`)
      - `./main.js`
  - `main.js`
    - Connects MetaMask using `ethers.BrowserProvider`
    - Creates `IpfsHttpClient.create({ url: "http://127.0.0.1:5001/api/v0" })`
    - On upload:
      - Reads the image as bytes and calls `ipfsClient.add(...)`
      - Extracts the CID (`result.path` / `result.cid`)
      - Calls `contract.storeHash(cid)` using `ethers.js`
      - Displays transaction hash, block number, gas used, sender, and CID

### 9. Notes & troubleshooting

- **MetaMask network**
  - Make sure MetaMask is pointing to `http://127.0.0.1:8545` (Localhost 8545).
  - Import a private key from the Hardhat node output so you have test ETH.
- **IPFS CORS**
  - If you see CORS issues, ensure your IPFS daemon allows requests from `http://127.0.0.1:5173`.
- **Re-deploying**
  - If you restart the Hardhat node, you must re-run:
    - `npm run hardhat:deploy` (to deploy again and regenerate `frontend/config.js`)

