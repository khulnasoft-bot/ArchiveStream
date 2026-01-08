# 🌐 ArchiveStream

> **An open-source, self-hostable web archive system.**
> Built with Rust + Next.js. WARC-compliant. Modern. Fast.

[![Crawler CI](https://github.com/KhulnaSoft/ArchiveStream/actions/workflows/crawler.yml/badge.svg)](https://github.com/KhulnaSoft/ArchiveStream/actions/workflows/crawler.yml)
[![Backend CI](https://github.com/KhulnaSoft/ArchiveStream/actions/workflows/backend.yml/badge.svg)](https://github.com/KhulnaSoft/ArchiveStream/actions/workflows/backend.yml)
[![UI CI](https://github.com/KhulnaSoft/ArchiveStream/actions/workflows/ui.yml/badge.svg)](https://github.com/KhulnaSoft/ArchiveStream/actions/workflows/ui.yml)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

ArchiveStream is a senior-engineer-grade, decentralized alternative to the Wayback Machine. It provides a full suite of tools to crawl the web, store immutable snapshots in industry-standard WARC files, and replay them faithfully.

---

## 🚀 Key Features

*   **⚡ High-Performance Crawler**: Built in Rust with `tokio` and `reqwest`.
*   **📦 WARC Standard**: Uses the ISO standard Web ARChive format.
*   **🛡️ Content Deduplication**: Integrated SHA-256 based deduplication.
*   **🌐 Modern UI**: Next.js 14 App Router interface with a premium aesthetic.
*   **🐳 Docker-First**: One-command deployment with Docker Compose.
*   **🔐 AGPL Licensed**: Free, open, and stays that way.

---

## 🏗️ Architecture

ArchiveStream is designed as a modular monorepo:

*   `crates/crawler`: The high-speed Rust engine responsible for fetching and link extraction.
*   `crates/archive-api`: The Axum-based REST API serving metadata and replay data.
*   `apps/web-ui`: The Next.js frontend for searching and exploring history.
*   `infra/`: Docker configurations and deployment scripts.

---

## 🛠️ Getting Started

### Prerequisites

*   Docker & Docker Compose
*   (Optional) Rust 1.75+
*   (Optional) Node.js 20+

### Local Development

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/KhulnaSoft/ArchiveStream.git
    cd ArchiveStream
    ```

2.  **Start the services**:
    ```bash
    docker compose -f infra/compose.yml up --build
    ```

3.  **Access the applications**:
    *   **Frontend**: [http://localhost:3000](http://localhost:3000)
    *   **Backend API**: [http://localhost:3001](http://localhost:3001)
    *   **MinIO Console**: [http://localhost:9001](http://localhost:9001)

---

## 🕷️ Project Vision

We believe the memory of the web should be decentralized and public. ArchiveStream aims to provide the tools for individuals and organizations to preserve the digital commons without relying on a single central authority.

---

## 🤝 Contributing

We welcome contributions from the community! Please read our [CONTRIBUTING.md](docs/CONTRIBUTING.md) to get started.

## 📄 License

ArchiveStream is licensed under the [AGPL-3.0 License](LICENSE).
