CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  author VARCHAR NOT NULL,
  body TEXT NOT NULL
)