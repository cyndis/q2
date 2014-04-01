BEGIN;
INSERT INTO session VALUES (
  0
);
INSERT INTO network VALUES (
  0,
  last_insert_rowid(),
  "ASCII",
  "UTF-8",
  NULL,
  NULL
);
COMMIT;
