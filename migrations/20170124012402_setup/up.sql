CREATE TABLE users (
  name TEXT NOT NULL PRIMARY KEY,
  description TEXT,
  UNIQUE(name)
);
CREATE TABLE repos (
  id INTEGER NOT NULL PRIMARY KEY,
  user_name TEXT NOT NULL,
  name TEXT NOT NULL,
  description TEXT,
  private BOOLEAN NOT NULL,
  fork BOOLEAN NOT NULL,
  FOREIGN KEY(user_name) REFERENCES users(name),
  UNIQUE(id),
  UNIQUE(user_name, name)
);
CREATE TABLE tags (
  name TEXT NOT NULL PRIMARY KEY,
  UNIQUE(name)
);
CREATE TABLE repo_tags (
  repo_id INTEGER NOT NULL PRIMARY KEY,
  tag_name TEXT NOT NULL,
  FOREIGN KEY(repo_id) REFERENCES repos(id),
  FOREIGN KEY(tag_name) REFERENCES tags(name),
  UNIQUE(tag_name, repo_id)
);
