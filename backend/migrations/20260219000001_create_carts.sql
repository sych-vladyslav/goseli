CREATE TABLE carts (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    store_id    UUID         NOT NULL REFERENCES stores(id) ON DELETE CASCADE,
    user_id     UUID         REFERENCES users(id) ON DELETE CASCADE,
    session_id  VARCHAR(255),
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

-- Index for user cart lookup
CREATE INDEX idx_carts_store_user ON carts (store_id, user_id)
    WHERE user_id IS NOT NULL;

-- Index for guest cart lookup
CREATE INDEX idx_carts_store_session ON carts (store_id, session_id)
    WHERE session_id IS NOT NULL;

CREATE INDEX idx_carts_store_id ON carts (store_id);

CREATE TRIGGER set_carts_updated_at
    BEFORE UPDATE ON carts
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_updated_at();
