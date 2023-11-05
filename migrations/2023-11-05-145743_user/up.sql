CREATE TABLE users (
  id UUID PRIMARY KEY,
  email VARCHAR(255) NOT NULL UNIQUE,
  nick VARCHAR(255) NOT NULL UNIQUE,
  hash VARCHAR(255) NOT NULL,
  access_key VARCHAR(255) UNIQUE,
  picture VARCHAR(255),
  is_public BOOLEAN NOT NULL DEFAULT FALSE,
  account_id UUID NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  FOREIGN KEY (account_id) REFERENCES account (id) ON DELETE CASCADE
);

INSERT INTO
    users (id, nick, email, hash, is_public, account_id)
VALUES
    (
        'a74f9b43-8a49-4d97-8270-9879d37c600d',
        'mishaszu',
        'mishaszu@gmail.com',
        '$argon2id$v=19$m=19456,t=2,p=1$AreaBODoNb1PVkrVYG47YQ$RqDZNg9uwWgRDFoeJkIED5RarIBPky6a0mvjr8sqVfs',
        true,
        'bf473007-7c05-4975-941a-dbd16426844d'
    );

