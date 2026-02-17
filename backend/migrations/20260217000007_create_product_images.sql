CREATE TABLE product_images (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    product_id  UUID          NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    url         VARCHAR(2048) NOT NULL,
    alt_text    VARCHAR(255),
    sort_order  INTEGER       NOT NULL DEFAULT 0,
    is_primary  BOOLEAN       NOT NULL DEFAULT false,
    created_at  TIMESTAMPTZ   NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_product_images_product ON product_images (product_id, sort_order);
