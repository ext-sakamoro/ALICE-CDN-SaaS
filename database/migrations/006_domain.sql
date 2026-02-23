-- ALICE CDN: Domain-specific tables
CREATE TABLE IF NOT EXISTS cdn_assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id),
    path TEXT NOT NULL,
    content_type TEXT,
    size_bytes BIGINT NOT NULL DEFAULT 0,
    ttl_secs INTEGER NOT NULL DEFAULT 86400,
    edge_url TEXT NOT NULL,
    regions TEXT[] NOT NULL DEFAULT '{"us-east","eu-west","ap-northeast"}',
    status TEXT NOT NULL DEFAULT 'deployed' CHECK (status IN ('deploying', 'deployed', 'purged')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS cdn_edge_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    region TEXT NOT NULL,
    period_start TIMESTAMPTZ NOT NULL,
    requests BIGINT NOT NULL DEFAULT 0,
    bytes_served BIGINT NOT NULL DEFAULT 0,
    cache_hit_rate DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    avg_latency_ms DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    p99_latency_ms DOUBLE PRECISION NOT NULL DEFAULT 0.0
);

CREATE INDEX idx_cdn_assets_user ON cdn_assets(user_id);
CREATE INDEX idx_cdn_edge_metrics_region ON cdn_edge_metrics(region, period_start);
