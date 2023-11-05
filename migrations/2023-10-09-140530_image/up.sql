CREATE TABLE image (
    id UUID PRIMARY KEY,
    title TEXT,
    description TEXT,
    -- after init path should lead to internal file system, after upload it should lead to lust storage
    path TEXT UNIQUE NOT NULL,
    is_uploaded BOOLEAN NOT NULL DEFAULT FALSE,
    is_printable BOOLEAN NOT NULL DEFAULT FALSE,
    is_printed BOOLEAN NOT NULL DEFAULT FALSE,
    is_favorite BOOLEAN NOT NULL DEFAULT FALSE,
    is_hidden BOOLEAN NOT NULL DEFAULT FALSE,
    image_date TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE image_rating_score (
    id UUID PRIMARY KEY,
    image_id UUID NOT NULL,
    rating_score_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (image_id) REFERENCES image (id) ON DELETE CASCADE,
    FOREIGN KEY (rating_score_id) REFERENCES rating_score (id) ON DELETE CASCADE
);

CREATE TABLE image_tag (
    id UUID PRIMARY KEY,
    image_id UUID NOT NULL,
    tag_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (image_id) REFERENCES image (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tag (id) ON DELETE CASCADE
);