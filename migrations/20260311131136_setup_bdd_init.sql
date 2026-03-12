-- Add migration script here
CREATE TABLE users (
    id BINARY(16) PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    pseudo varchar(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    birth_date DATE NULL,
    phone VARCHAR(20),
    pp VARCHAR(255),
    subscription BOOLEAN default FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

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

CREATE TABLE `groups`(
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
    FOREIGN KEY (group_id) REFERENCES `groups`(id) ON DELETE CASCADE
);

CREATE TABLE user_group(
    id BINARY(16) PRIMARY KEY,
    user_id BINARY(16) NOT NULL,
    group_id BINARY(16) NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES `groups`(id) ON DELETE CASCADE
);