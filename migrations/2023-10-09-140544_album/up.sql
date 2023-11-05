CREATE TABLE album (
    id UUID PRIMARY KEY,
    title VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
		picture_id UUID,
    is_favorite BOOLEAN NOT NULL DEFAULT FALSE,
    is_hidden BOOLEAN NOT NULL DEFAULT FALSE,
    is_print_album BOOLEAN NOT NULL DEFAULT FALSE,
    is_printed BOOLEAN NOT NULL DEFAULT FALSE,
    image_per_page INTEGER,
    album_date TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
		FOREIGN KEY (picture_id) REFERENCES image (id)
);

CREATE TABLE album_image (
    id UUID PRIMARY KEY,
    album_id UUID NOT NULL,
    image_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (album_id) REFERENCES album (id) ON DELETE CASCADE,
    FOREIGN KEY (image_id) REFERENCES image (id) ON DELETE CASCADE
);

CREATE TABLE album_rating_score (
    id UUID PRIMARY KEY,
    album_id UUID NOT NULL,
    rating_score_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (album_id) REFERENCES album (id) ON DELETE CASCADE,
    FOREIGN KEY (rating_score_id) REFERENCES rating_score (id) ON DELETE CASCADE
);

CREATE TABLE album_tag (
    id UUID PRIMARY KEY,
    album_id UUID NOT NULL,
    tag_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (album_id) REFERENCES album (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tag (id) ON DELETE CASCADE
);