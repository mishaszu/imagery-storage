CREATE TABLE post (
  id UUID PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  body TEXT NOT NULL,
  user_id UUID NOT NULL,
  -- 0 = private, 1 = close followers, 2 = all followers, 3 = public
  public_lvl INT NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE post_image (
  id UUID PRIMARY KEY,
  post_id UUID NOT NULL,
  image_id UUID NOT NULL,
  FOREIGN KEY (post_id) REFERENCES post (id) ON DELETE CASCADE,
  FOREIGN KEY (image_id) REFERENCES image (id) ON DELETE CASCADE
);
