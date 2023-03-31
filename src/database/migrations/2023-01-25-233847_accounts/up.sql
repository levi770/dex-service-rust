DROP TABLE IF EXISTS accounts;
CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR NOT NULL,
    address VARCHAR NOT NULL,
    keystore VARCHAR NOT NULL
);