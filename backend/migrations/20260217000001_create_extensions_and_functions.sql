-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- UUID v7 generator function (RFC 9562)
-- Produces time-sortable UUIDs: first 48 bits = Unix timestamp in ms
CREATE OR REPLACE FUNCTION uuid_generate_v7() RETURNS uuid AS $$
DECLARE
    unix_ts_ms bytea;
    uuid_bytes bytea;
BEGIN
    unix_ts_ms = substring(int8send(floor(extract(epoch from clock_timestamp()) * 1000)::bigint) from 3);
    uuid_bytes = unix_ts_ms || gen_random_bytes(10);
    -- Set version bits (0111 = v7)
    uuid_bytes = set_byte(uuid_bytes, 6, (b'0111' || get_byte(uuid_bytes, 6)::bit(4))::bit(8)::int);
    -- Set variant bits (10 = RFC 4122)
    uuid_bytes = set_byte(uuid_bytes, 8, (b'10' || get_byte(uuid_bytes, 8)::bit(6))::bit(8)::int);
    RETURN encode(uuid_bytes, 'hex')::uuid;
END
$$ LANGUAGE plpgsql VOLATILE;

-- Auto-update updated_at trigger function
CREATE OR REPLACE FUNCTION trigger_set_updated_at() RETURNS trigger AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END
$$ LANGUAGE plpgsql;
