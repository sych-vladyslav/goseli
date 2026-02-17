CREATE TABLE stores (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    slug        VARCHAR(63)  NOT NULL UNIQUE,
    name        VARCHAR(255) NOT NULL,
    description TEXT,
    config      JSONB        NOT NULL DEFAULT '{}',
    theme       VARCHAR(63)  NOT NULL DEFAULT 'default',
    currency    VARCHAR(3)   NOT NULL DEFAULT 'USD',
    domain      VARCHAR(255),
    is_active   BOOLEAN      NOT NULL DEFAULT true,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_stores_domain ON stores (domain) WHERE domain IS NOT NULL;
CREATE INDEX idx_stores_config ON stores USING GIN (config);

CREATE TRIGGER set_stores_updated_at
    BEFORE UPDATE ON stores
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_updated_at();
