CREATE TABLE users (
  id UUID PRIMARY KEY,
  email VARCHAR(255) NOT NULL UNIQUE,
  nick VARCHAR(255) NOT NULL UNIQUE,
  hash VARCHAR(255) NOT NULL,
  account_id UUID NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  FOREIGN KEY (account_id) REFERENCES account (id) ON DELETE CASCADE
);

INSERT INTO
    users (id, nick, email, hash, account_id)
VALUES
    (
        'a74f9b43-8a49-4d97-8270-9879d37c600d',
        'mishaszu',
        'mishaszu@gmail.com',
        '$argon2id$v=19$m=19456,t=2,p=1$AreaBODoNb1PVkrVYG47YQ$RqDZNg9uwWgRDFoeJkIED5RarIBPky6a0mvjr8sqVfs',
        'bf473007-7c05-4975-941a-dbd16426844d'
    );

