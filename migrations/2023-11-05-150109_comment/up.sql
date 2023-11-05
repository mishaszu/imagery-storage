CREATE TABLE comment (
  id UUID PRIMARY KEY,
  user_id UUID NOT NULL,
  image_id UUID NOT NULL,
  body TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE NO ACTION,
  FOREIGN KEY (image_id) REFERENCES image(id) ON DELETE CASCADE
);
