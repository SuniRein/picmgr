CREATE TYPE user_status AS ENUM ('active', 'banned');

CREATE TABLE app_user(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    avatar_url VARCHAR(255),
    status user_status DEFAULT 'active',
    created_at TIMESTAMP DEFAULT NOW()
);
