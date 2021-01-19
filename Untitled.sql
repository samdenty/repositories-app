CREATE TABLE refs4 (
  owner TEXT NOT NULL,
  repo TEXT NOT NULL,
  ref TEXT NOT NULL,
  sha TEXT NOT NULL,
  FOREIGN KEY(owner, repo,ref,sha) REFERENCES repos(owner2,a,b,c),
  PRIMARY KEY(owner, repo, ref)
)