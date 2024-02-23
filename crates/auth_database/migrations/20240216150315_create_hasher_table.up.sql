CREATE TABLE
  `hasher` (
    `hasher_pk` BINARY(16),
    `name` VARCHAR(64) NOT NULL,
    PRIMARY KEY (`hasher_pk`),
    CONSTRAINT `unique__name` UNIQUE (`name`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE TABLE
  `user_credential__has__hasher` (
    `user_credential_pk` BINARY(16) NOT NULL,
    `hasher_pk` BINARY(16) NOT NULL,
    `hash` TEXT NOT NULL,
    `salt` BINARY(16) NOT NULL,
    `expired_at` DATETIME NOT NULL DEFAULT '9999-12-31 23:59:59',
    PRIMARY KEY (`user_credential_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

INSERT INTO
  `hasher` (`hasher_pk`, `name`)
VALUES
  (0x018d938bdf7e761199bcf12458f48155, "argon2");
