CREATE TABLE account (
  id UUID PRIMARY KEY,
  fullname VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL UNIQUE,
  kind VARCHAR(255) NOT NULL DEFAULT 'guest',
  is_admin BOOLEAN NOT NULL DEFAULT FALSE,
  public_lvl INT NOT NULL DEFAULT 1,
  is_banned BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO
    account (id, email, is_admin, fullname)
VALUES
    (
        'bf473007-7c05-4975-941a-dbd16426844d',
        'root@test.com',
        true,
        'admin name'
    );


