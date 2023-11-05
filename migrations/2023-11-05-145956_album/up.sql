CREATE TABLE album (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    picture UUID,
    is_public BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES account (id) ON DELETE CASCADE,
    FOREIGN KEY (picture) REFERENCES image (id) ON DELETE NO ACTION
);

CREATE TABLE album_image (
  id UUID PRIMARY KEY,
  album_id UUID NOT NULL,
  image_id UUID NOT NULL,
  FOREIGN KEY (album_id) REFERENCES album (id) ON DELETE CASCADE,
  FOREIGN KEY (image_id) REFERENCES image (id) ON DELETE CASCADE
);
