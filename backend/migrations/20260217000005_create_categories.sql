CREATE TABLE categories (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    store_id    UUID         NOT NULL REFERENCES stores(id) ON DELETE CASCADE,
    parent_id   UUID         REFERENCES categories(id) ON DELETE SET NULL,
    name        VARCHAR(255) NOT NULL,
    slug        VARCHAR(255) NOT NULL,
    description TEXT,
    sort_order  INTEGER      NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    UNIQUE (store_id, slug)
);

CREATE INDEX idx_categories_store_id ON categories (store_id);
CREATE INDEX idx_categories_parent ON categories (parent_id);

CREATE TRIGGER set_categories_updated_at
    BEFORE UPDATE ON categories
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_updated_at();
