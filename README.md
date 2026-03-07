# ALICE-CDN-SaaS

Multi-region edge content delivery network with Vivaldi coordinate-based latency routing, atomic asset deployment, and glob-pattern cache purge.

## Architecture

```
Client
  │
  ▼
┌─────────────────────────────────────────┐
│           ALICE-CDN-SaaS API            │
│         (Rust / Axum, port 8081)        │
└──────────┬──────────────┬──────────────┘
           │              │
  ┌────────▼──────┐  ┌────▼────────────┐
  │  Asset Push   │  │  Vivaldi Router  │
  │  Pipeline     │  │  (Latency Map)   │
  └────────┬──────┘  └────┬────────────┘
           │               │
  ┌────────▼───────────────▼────────────┐
  │           Edge Node Manager         │
  │  us-east │ eu-west │ ap-south │ ... │
  └────────────────────────────────────┘
           │
  ┌────────▼────────────────────────────┐
  │   Cache Purge Engine                │
  │   (Glob pattern, multi-region sync) │
  └─────────────────────────────────────┘
```

## Features

| Feature | Details |
|---------|---------|
| Edge Asset Deployment | Push assets to any subset of edge regions atomically |
| Vivaldi Latency Mapping | Decentralized coordinate-based routing to lowest-latency edge |
| Multi-Region CDN | Active edge nodes across US, EU, and APAC |
| Glob-Pattern Cache Purge | Purge single files or entire path prefixes globally |
| Bandwidth Analytics | Per-region bandwidth, cache hit rate, and error rate |

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/cdn/push` | Deploy an asset from origin to specified edge regions |
| DELETE | `/api/v1/cdn/purge` | Purge cached assets matching a URL glob pattern |
| POST | `/api/v1/cdn/latency` | Estimate latency between two edge regions (Vivaldi) |
| GET | `/api/v1/cdn/edges` | List all active edge nodes with status and coordinates |
| GET | `/api/v1/cdn/stats` | Bandwidth, cache hit rates, and per-region metrics |

## Quick Start

```bash
# Clone and start the backend
git clone https://github.com/your-org/ALICE-CDN-SaaS.git
cd ALICE-CDN-SaaS
cargo run --release

# In a second terminal, start the frontend
cd frontend
npm install
npm run dev
# Open http://localhost:3000
```

### Example: Push Asset to Edge

```bash
curl -X POST http://localhost:8081/api/v1/cdn/push \
  -H "Content-Type: application/json" \
  -d '{"asset_url":"https://origin.example.com/logo.png","edges":["us-east","eu-west"]}'
```

### Example: Purge Cache

```bash
curl -X DELETE http://localhost:8081/api/v1/cdn/purge \
  -H "Content-Type: application/json" \
  -d '{"pattern":"/assets/*"}'
```

### Example: Estimate Latency

```bash
curl -X POST http://localhost:8081/api/v1/cdn/latency \
  -H "Content-Type: application/json" \
  -d '{"from":"us-east","to":"ap-south"}'
```

## License

AGPL-3.0-or-later — see [LICENSE](LICENSE) for details.
