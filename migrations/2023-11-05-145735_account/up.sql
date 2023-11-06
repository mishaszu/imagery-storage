CREATE TABLE account (
  id UUID PRIMARY KEY,
  referral_id UUID UNIQUE,
  email VARCHAR(255) NOT NULL UNIQUE,
  kind VARCHAR(255) NOT NULL DEFAULT 'commenter',
  followee_id UUID,
  is_admin BOOLEAN NOT NULL DEFAULT FALSE,
  public_lvl INT NOT NULL DEFAULT 0,
  is_banned BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- initial password to change: test_pass
INSERT INTO
    account (id, email, is_admin)
VALUES
    (
        'bf473007-7c05-4975-941a-dbd16426844d',
        'mishaszu@gmail.com',
        true
    );


