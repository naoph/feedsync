CREATE TABLE channels (
    -- Internal data about the channel
    id BIGSERIAL PRIMARY KEY,
    remote text UNIQUE NOT NULL,
    slug text UNIQUE NOT NULL,

    -- Data specified by the channel
    d_title text NOT NULL,
    d_link text NOT NULL,
    d_description text NOT NULL,
    d_categories text[] NOT NULL,
    d_image text,
    d_author text,
    d_subtitle text,

    -- Cached media
    c_image text
);

CREATE TABLE items (
    -- Internal data about the item
    id BIGSERIAL PRIMARY KEY,
    channel bigint NOT NULL,
    has_enclosure boolean NOT NULL,
    discovered timestamp with time zone NOT NULL,

    -- Data specified by the item
    d_title text,
    d_link text,
    d_description text,
    d_author text,
    d_categories text[] NOT NULL,
    d_encl_url text,
    d_encl_length text,
    d_encl_mime_type text,
    d_guid text,
    d_pub_date timestamp with time zone,
    d_content text,
    d_image text,
    d_duration integer,

    -- Cached media
    c_encl_url text,
    c_image text,

    UNIQUE (channel, d_guid),
    CHECK (has_enclosure = (d_encl_url IS NOT NULL)),
    CHECK ((d_encl_url IS NULL) = (d_encl_length IS NULL)),
    CHECK ((d_encl_length IS NULL) = (d_encl_mime_type IS NULL))
);
