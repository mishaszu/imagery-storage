CREATE TABLE tag (
  id UUID PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE image_tag (
  id SERIAL PRIMARY KEY,
  image_id UUID NOT NULL,
  tag_id UUID NOT NULL,
  FOREIGN KEY (image_id) REFERENCES image (id) ON DELETE CASCADE,
  FOREIGN KEY (tag_id) REFERENCES tag (id) ON DELETE CASCADE
);
