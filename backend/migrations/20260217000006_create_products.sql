CREATE TABLE products (
    id                UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    store_id          UUID         NOT NULL REFERENCES stores(id) ON DELETE CASCADE,
    category_id       UUID         REFERENCES categories(id) ON DELETE SET NULL,
    name              VARCHAR(255) NOT NULL,
    slug              VARCHAR(255) NOT NULL,
    description       TEXT,
    short_description VARCHAR(500),
    price             INTEGER      NOT NULL CHECK (price >= 0),
    compare_at_price  INTEGER      CHECK (compare_at_price IS NULL OR compare_at_price >= 0),
    cost_price        INTEGER      CHECK (cost_price IS NULL OR cost_price >= 0),
    sku               VARCHAR(100),
    stock_quantity    INTEGER      NOT NULL DEFAULT 0 CHECK (stock_quantity >= 0),
    attributes        JSONB        NOT NULL DEFAULT '{}',
    status            VARCHAR(20)  NOT NULL DEFAULT 'draft'
                      CHECK (status IN ('draft', 'active', 'archived')),
    is_featured       BOOLEAN      NOT NULL DEFAULT false,
    created_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    UNIQUE (store_id, slug)
);

-- Partial unique index: sku must be unique per store when set
CREATE UNIQUE INDEX idx_products_sku ON products (store_id, sku)
    WHERE sku IS NOT NULL;

CREATE INDEX idx_products_store_id ON products (store_id);
CREATE INDEX idx_products_category ON products (store_id, category_id);
CREATE INDEX idx_products_status ON products (store_id, status);
CREATE INDEX idx_products_featured ON products (store_id, is_featured)
    WHERE is_featured = true;
CREATE INDEX idx_products_price ON products (store_id, price);
CREATE INDEX idx_products_created ON products (store_id, created_at DESC);
CREATE INDEX idx_products_attributes ON products USING GIN (attributes);

CREATE TRIGGER set_products_updated_at
    BEFORE UPDATE ON products
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_updated_at();
