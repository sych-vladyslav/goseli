CREATE TABLE product_variants (
    id                UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    product_id        UUID         NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    name              VARCHAR(255) NOT NULL,
    sku               VARCHAR(100),
    price             INTEGER      NOT NULL CHECK (price >= 0),
    compare_at_price  INTEGER      CHECK (compare_at_price IS NULL OR compare_at_price >= 0),
    stock_quantity    INTEGER      NOT NULL DEFAULT 0 CHECK (stock_quantity >= 0),
    attributes        JSONB        NOT NULL DEFAULT '{}',
    sort_order        INTEGER      NOT NULL DEFAULT 0,
    is_active         BOOLEAN      NOT NULL DEFAULT true,
    created_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_product_variants_product ON product_variants (product_id);
CREATE UNIQUE INDEX idx_product_variants_sku ON product_variants (sku)
    WHERE sku IS NOT NULL;
CREATE INDEX idx_product_variants_attrs ON product_variants USING GIN (attributes);

CREATE TRIGGER set_product_variants_updated_at
    BEFORE UPDATE ON product_variants
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_updated_at();
