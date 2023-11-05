CREATE TABLE users (
    id UUID PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    hash VARCHAR(255) NOT NULL,
    fp INTEGER NOT NULL DEFAULT 0,
    wsic INTEGER NOT NULL DEFAULT 0,
    is_admin BOOLEAN NOT NULL DEFAULT false,
    subscription VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- initial password to change: test_pass
INSERT INTO
    users (id, name, email, is_admin, hash, subscription)
VALUES
    (
        'a74f9b43-8a49-4d97-8270-9879d37c600d',
        'mishaszu',
        'mishaszu@gmail.com',
        true,
        '$argon2id$v=19$m=19456,t=2,p=1$AreaBODoNb1PVkrVYG47YQ$RqDZNg9uwWgRDFoeJkIED5RarIBPky6a0mvjr8sqVfs',
        'diamond'
    );
