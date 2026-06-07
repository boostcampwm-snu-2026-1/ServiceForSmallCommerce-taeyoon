CREATE TABLE analyses (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id      UUID NOT NULL REFERENCES users(id),
    urls         TEXT[] NOT NULL,
    review_limit INT NOT NULL DEFAULT 100,
    status       TEXT NOT NULL DEFAULT 'pending',
    result       JSONB,
    error        TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);
CREATE INDEX idx_analyses_user_created ON analyses (user_id, created_at DESC);
