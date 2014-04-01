CREATE TABLE IF NOT EXISTS session(
    id INTEGER PRIMARY KEY AUTOINCREMENT
);
CREATE TABLE IF NOT EXISTS network(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,

    network_encoding STRING NOT NULL,
    message_encoding STRING NOT NULL,

    server STRING,
    nickname STRING,

    FOREIGN KEY(session_id) REFERENCES session(id)
);
CREATE TABLE IF NOT EXISTS buffer(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    network_id INTEGER NOT NULL,

    role INTEGER NOT NULL,
    name STRING NOT NULL,

    last_read INTEGER,

    FOREIGN KEY(network_id) REFERENCES network(id),
    FOREIGN KEY(last_read) REFERENCES message(id)
);
CREATE TABLE IF NOT EXISTS message(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    buffer_id INTEGER NOT NULL,

    time_ns INTEGER NOT NULL,
    type INTEGER NOT NULL,

    FOREIGN KEY(buffer_id) REFERENCES buffer(id)
);
CREATE TABLE IF NOT EXISTS message_information(
    message_id INTEGER UNIQUE,
    message STRING,

    FOREIGN KEY(message_id) REFERENCES message(id)
);
CREATE TABLE IF NOT EXISTS message_join(
    message_id INTEGER UNIQUE,
    who STRING,

    FOREIGN KEY(message_id) REFERENCES message(id)
);
CREATE TABLE IF NOT EXISTS message_privmsg(
    message_id INTEGER UNIQUE,
    who STRING,
    message STRING,

    FOREIGN KEY(message_id) REFERENCES message(id)
);
