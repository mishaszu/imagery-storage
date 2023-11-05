CREATE TABLE sys_config (  
  id UUID PRIMARY KEY,
  allow_registration BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO
    sys_config (id, allow_registration)
VALUES
    (
        '6d5c6e99-3d45-4873-8ca0-af2fefedd873',
        false
    );
