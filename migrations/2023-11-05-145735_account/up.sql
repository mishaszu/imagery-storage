CREATE TABLE account (
  id UUID PRIMARY KEY,
  referral_id UUID UNIQUE,
  email VARCHAR(255) NOT NULL UNIQUE,
  kind VARCHAR(255) NOT NULL DEFAULT 'commenter',
  is_admin BOOLEAN NOT NULL DEFAULT FALSE,
  is_public BOOLEAN NOT NULL DEFAULT FALSE,
  is_active BOOLEAN NOT NULL DEFAULT TRUE,
  is_banned BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
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


