pub static CREATE_PROBLEMS_IF_NOT_EXISTS: &str = r#"
  CREATE TABLE IF NOT EXISTS problems (
    category TEXT NOT NULL,
    fid INTEGER NOT NULL,
    id INTEGER NOT NULL PRIMARY KEY,
    level INTEGER NOT NULL,
    locked BOOLEAN NOT NULL DEFAULT 0,
    name TEXT NOT NULL,
    percent FLOAT NOT NULL,
    slug TEXT NOT NULL,
    starred BOOLEAN NOT NULL DEFAULT 0,
    status TEXT NOT NULL,
    desc TEXT NOT NULL DEFAULT ""
  )
"#;

pub static CREATE_TAGS_IF_NOT_EXISTS: &str = r#"
  CREATE TABLE IF NOT EXISTS tags (
    tag TEXT NOT NULL,
    refs TEXT NOT NULL
  )
"#;

// pub static DROP_PROBLEMS: &'static str = r#"DROP TABLE problems"#;
