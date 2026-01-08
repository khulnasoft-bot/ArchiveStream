# ArchiveStream Federation Protocol (ASFP) v1

## 1. Overview
The ArchiveStream Federation Protocol (ASFP) enables autonomous ArchiveStream instances to form a distributed archival network. Peers can share crawl burdens, synchronize WARC datasets, and federate search queries.

## 2. Architecture

```
[Node A] <===> [Node B] <===> [Node C]
   |              |              |
(Local DB)     (Local DB)     (Local DB)
```

### 2.1 Node Identity
*   **Node ID**: SHA-256 hash of public key
*   **Endpoint**: `https://<host>:<port>/api/v1/federation`
*   **Capabilities**: `[crawl, store, search, index]`

## 3. Discovery Mechanisms

### 3.1 Bootstrap
New nodes join the network by connecting to a known "seed" peer defined in configuration.

### 3.2 Gossip
Once connected, nodes periodically exchange their known peer lists (gossip).
*   `GET /federation/peers` -> Returns list of active peers with metadata.

## 4. API Specification

All federation endpoints are namespaced under `/api/v1/federation`.

### 4.1 Handshake
**POST** `/federation/handshake`
*   Input: `{ "node_id": "...", "version": "1.0", "endpoint": "..." }`
*   Output: `{ "status": "accepted", "challenge": "..." }`

### 4.2 Sync Protocol (WARC Exchange)
Inspired by BitTorrent and git.
1.  **Have List**: Node A sends list of WARC file hashes it holds.
2.  **Want List**: Node B responds with hashes it is missing.
3.  **Transfer**: Node B requests specific WARCs via `GET /federation/blob/{hash}`.

### 4.3 Federated Search
**POST** `/federation/search`
*   Input: `{ "query": "example", "timeout_ms": 500 }`
*   Node A broadcasts query to top N healthy peers.
*   Peers return local search results (ranked).
*   Node A aggregates, deduplicates, and returns to user.

## 5. Security Model
*   **Transport**: TLS 1.3 required for all federation traffic.
*   **Trust**:
    *   *Public Mode*: Connect to any node (potential spam risk).
    *   *Private Mode*: Whitelist of allowed CA or specific public keys (friend-to-friend).

## 6. Implementation Stages
1.  **Skeleton**: Peer list management and heartbeat.
2.  **Search**: Broadcast search queries.
3.  **Sync**: WARC file replication.
