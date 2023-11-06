CREATE TABLE album (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    picture UUID,
    public_lvl INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (user_id) REFERENCES account (id) ON DELETE CASCADE,
    FOREIGN KEY (picture) REFERENCES image (id) ON DELETE NO ACTION
);

CREATE TABLE album_post (
  id UUID PRIMARY KEY,
  album_id UUID NOT NULL,
  post_id UUID NOT NULL,
  FOREIGN KEY (album_id) REFERENCES album (id) ON DELETE CASCADE,
  FOREIGN KEY (post_id) REFERENCES post (id) ON DELETE CASCADE
);
