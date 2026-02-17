CREATE TABLE users (
    id            UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    store_id      UUID         NOT NULL REFERENCES stores(id) ON DELETE CASCADE,
    email         VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    first_name    VARCHAR(100),
    last_name     VARCHAR(100),
    role          VARCHAR(20)  NOT NULL DEFAULT 'customer'
                  CHECK (role IN ('super_admin', 'store_admin', 'staff', 'customer')),
    is_active     BOOLEAN      NOT NULL DEFAULT true,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    UNIQUE (store_id, email)
);

CREATE INDEX idx_users_store_id ON users (store_id);
CREATE INDEX idx_users_role ON users (store_id, role);

CREATE TRIGGER set_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_updated_at();
