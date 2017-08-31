CREATE TABLE todos (
  id SERIAL PRIMARY KEY,
  text TEXT NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO todos (text) VALUES ('Learn web development with Rocket.');
INSERT INTO todos (text) VALUES ('Find inner peace.');
