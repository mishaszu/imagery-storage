CREATE TABLE rating_score (
    id UUID PRIMARY KEY,
    rating_id UUID NOT NULL,
    score INTEGER NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (rating_id) REFERENCES rating (id)
);