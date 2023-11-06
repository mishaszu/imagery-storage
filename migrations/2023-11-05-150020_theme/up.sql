CREATE TABLE theme (
  id Uuid PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  color VARCHAR(255) NOT NULL,
  picture UUID,
  user_id UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  FOREIGN KEY (picture) REFERENCES image (id) ON DELETE NO ACTION,
  FOREIGN KEY (user_id) REFERENCES account (id) ON DELETE CASCADE
);
