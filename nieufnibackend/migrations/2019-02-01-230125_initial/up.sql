CREATE TABLE authors (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  password_hash VARCHAR NOT NULL
);


CREATE TABLE articles (
  id VARCHAR NOT NULL,
  db_id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  date VARCHAR NOT NULL,
  authors_id INTEGER NOT NULL REFERENCES authors(id),
  markdown_text TEXT NOT NULL,
  rendered_text TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
);

INSERT INTO authors(name, password_hash)
  VALUES('Invalid Author', 'DEAD')