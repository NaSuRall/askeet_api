-- Add migration script here

CREATE TABLE user_category(
    id BINARY(16),
    user_id BINARY(16),
    category_id BINARY,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
)
