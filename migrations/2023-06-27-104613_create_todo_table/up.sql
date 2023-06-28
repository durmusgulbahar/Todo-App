-- Your SQL goes here
CREATE TABLE todos
(
    id          VARCHAR(255) PRIMARY KEY,
    title       VARCHAR(255) NOT NULL,
    description TEXT,
    done BOOLEAN,
    created_at  TIMESTAMP,
    user_id VARCHAR(255) NOT NULL

);
