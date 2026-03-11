-- Add migration script here

DROP TABLE IF EXISTS user_group;
DROP TABLE IF EXISTS group_answer;
DROP TABLE IF EXISTS user_answer;
DROP TABLE IF EXISTS groups;
DROP TABLE IF EXISTS survey_answer;
DROP TABLE IF EXISTS surveys;
DROP TABLE IF EXISTS categories;
DROP TABLE IF EXISTS answers;

DELETE FROM _sqlx_migrations WHERE version = 20260311131136;

CREATE TABLE answers (
    id BINARY(16) PRIMARY KEY,
    text VARCHAR(50) NOT NULL,
    img VARCHAR(255) NULL
);

CREATE TABLE categories(
    id BINARY(16) PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);

CREATE TABLE surveys (
    id BINARY(16) PRIMARY KEY,
    creator_id BINARY(16) NOT NULL,
    title VARCHAR(255) NOT NULL,
    img VARCHAR(255) NULL,
    up INTEGER DEFAULT 0,
    color VARCHAR(25) NULL,
    category_id BINARY(16) NULL,

    FOREIGN KEY (creator_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);

CREATE TABLE survey_answer(
    id BINARY(16) PRIMARY KEY,
    survey_id BINARY(16) NOT NULL,
    answer_id BINARY(16) NOT NULL,

    FOREIGN KEY (survey_id) REFERENCES surveys(id) ON DELETE CASCADE,
    FOREIGN KEY (answer_id) REFERENCES answers(id) ON DELETE CASCADE
);

CREATE TABLE groups(
    id BINARY(16) PRIMARY KEY,
    creator_id BINARY(16) NOT NULL,
    title VARCHAR(50) NOT NULL,
    code VARCHAR(255) NULL,
    lien VARCHAR(255) NULL,
    qr_code VARCHAR(255) NULL,
    is_community BOOLEAN DEFAULT FALSE,

    FOREIGN KEY (creator_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE user_answer(
    id BINARY(16) PRIMARY KEY,
    survey_answer_id BINARY(16) NOT NULL,
    user_id BINARY(16) NOT NULL,

    FOREIGN KEY (survey_answer_id) REFERENCES survey_answer(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE group_answer(
    id BINARY(16) PRIMARY KEY,
    survey_answer_id BINARY(16) NOT NULL,
    group_id BINARY(16) NOT NULL,

    FOREIGN KEY (survey_answer_id) REFERENCES survey_answer(id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE
);

CREATE TABLE user_group(
    id BINARY(16) PRIMARY KEY,
    user_id BINARY(16) NOT NULL,
    group_id BINARY(16) NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE
);

ALTER TABLE users MODIFY COLUMN id BINARY(16) NOT NULL;

ALTER TABLE users
    ADD COLUMN first_name VARCHAR(255) NULL,
    ADD COLUMN last_name VARCHAR(255) NULL,
    ADD COLUMN birth_date DATE NULL,
    ADD COLUMN pp VARCHAR(255) NULL,
    ADD COLUMN subscription BOOLEAN DEFAULT FALSE;

