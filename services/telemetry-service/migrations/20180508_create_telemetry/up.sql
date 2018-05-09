CREATE TABLE telemetry (
    timestamp INTEGER PRIMARY KEY NULL,
    subsystem VARCHAR(255) NOT NULL,
    param VARCHAR(255) NOT NULL,
    value INTEGER NOT NULL
);
