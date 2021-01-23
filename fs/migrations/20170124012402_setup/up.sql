CREATE TABLE users (
  name TEXT NOT NULL PRIMARY KEY,
  description TEXT,
  UNIQUE(name)
);
CREATE TABLE repos (
  owner TEXT NOT NULL,
  name TEXT NOT NULL,
  default_branch TEXT NOT NULL,
  description TEXT,
  homepage TEXT,
  private BOOLEAN NOT NULL,
  fork BOOLEAN NOT NULL,
  FOREIGN KEY(owner) REFERENCES users(name),
  FOREIGN KEY(owner, name, default_branch) REFERENCES repo_branches(owner, repo, name),
  PRIMARY KEY(owner, name)
);
CREATE TABLE tags (name TEXT NOT NULL PRIMARY KEY);
CREATE TABLE repo_tags (
  owner TEXT NOT NULL,
  repo TEXT NOT NULL,
  tag_name TEXT NOT NULL,
  FOREIGN KEY(owner, repo) REFERENCES repos(owner, name),
  FOREIGN KEY(tag_name) REFERENCES tags(name),
  PRIMARY KEY(owner, repo, tag_name)
);
CREATE TABLE trees (
  sha TEXT NOT NULL,
  path TEXT NOT NULL,
  mode TEXT NOT NULL,
  blob_sha TEXT,
  FOREIGN KEY(blob_sha) REFERENCES blobs(sha),
  PRIMARY KEY(sha, path)
);
CREATE TABLE blobs (
  sha TEXT NOT NULL PRIMARY KEY,
  size INTEGER NOT NULL,
  data BLOB
);
CREATE TABLE repo_branches (
  owner TEXT NOT NULL,
  repo TEXT NOT NULL,
  name TEXT NOT NULL,
  tree_sha TEXT NOT NULL,
  FOREIGN KEY(owner, repo) REFERENCES repos(owner, name),
  FOREIGN KEY(tree_sha) REFERENCES trees(sha),
  PRIMARY KEY(owner, repo, name)
)
