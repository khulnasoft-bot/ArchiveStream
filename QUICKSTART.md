# Quick Start Guide

Get ArchiveStream running in **5 minutes**.

---

## 🚀 Option 1: Docker Compose (Recommended)

### Prerequisites
- Docker 20.10+
- Docker Compose 2.0+
- 8GB RAM minimum
- 50GB disk space

### Steps

```bash
# 1. Clone repository
git clone https://github.com/ArchiveStream/archivestream.git
cd archivestream

# 2. Copy environment file
cp .env.example .env

# 3. Start all services
docker-compose -f docker-compose.prod.yml up -d

# 4. Wait for services to be ready (30-60 seconds)
docker-compose -f docker-compose.prod.yml logs -f

# 5. Access the UI
open http://localhost:3000
```

That's it! ArchiveStream is now running.

---

## 🛠️ Option 2: Local Development

### Prerequisites
- Rust 1.75+
- Node.js 18+
- PostgreSQL 14+
- MinIO or S3
- OpenSearch 2.x

### Steps

```bash
# 1. Clone repository
git clone https://github.com/ArchiveStream/archivestream.git
cd archivestream

# 2. Start infrastructure
docker-compose -f infra/compose.yml up -d

# 3. Set environment variables
export DATABASE_URL="postgresql://admin:password@localhost:5432/archivestream"
export S3_ENDPOINT="http://localhost:9000"
export OPENSEARCH_URL="http://localhost:9200"

# 4. Run migrations
make migrate

# 5. Build and run services (in separate terminals)
cargo run --bin crawler &
cargo run --bin indexer &
cargo run --bin archive-api &

# 6. Start UI
cd apps/web-ui
npm install
npm run dev
```

Visit `http://localhost:3000`

---

## 📝 First Steps

### 1. Add a URL to crawl

```bash
curl -X POST http://localhost:3001/crawl \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com"}'
```

### 2. Search the archive

```bash
curl "http://localhost:3001/api/v1/search?q=example"
```

### 3. View the dashboard

Open `http://localhost:3000/dashboard` to see:
- Frontier heatmap
- Crawl success/failure metrics
- Real-time throughput

---

## 🔧 Common Commands

```bash
# View logs
docker-compose -f docker-compose.prod.yml logs -f

# Stop services
docker-compose -f docker-compose.prod.yml down

# Rebuild images
make docker-build

# Run tests
make test

# Clean up
make clean
```

---

## 🐛 Troubleshooting

### Services won't start
```bash
# Check if ports are available
lsof -i :3000  # UI
lsof -i :3001  # API
lsof -i :5432  # PostgreSQL
lsof -i :9000  # MinIO
lsof -i :9200  # OpenSearch
```

### Database connection errors
```bash
# Verify PostgreSQL is running
docker-compose -f docker-compose.prod.yml ps postgres

# Check logs
docker-compose -f docker-compose.prod.yml logs postgres
```

### Crawler not working
```bash
# Check crawler logs
docker-compose -f docker-compose.prod.yml logs crawler-us-east-1

# Verify frontier has URLs
psql $DATABASE_URL -c "SELECT COUNT(*) FROM url_frontier;"
```

---

## 📚 Next Steps

- Read [IMPLEMENTATION_SUMMARY.md](docs/IMPLEMENTATION_SUMMARY.md) for architecture
- Check [API_V1.md](docs/API_V1.md) for API documentation
- See [PRODUCTION_CHECKLIST.md](docs/PRODUCTION_CHECKLIST.md) for deployment
- Join our [Discord](https://discord.gg/archivestream) for support

---

**Happy Archiving!** 🌐✨
