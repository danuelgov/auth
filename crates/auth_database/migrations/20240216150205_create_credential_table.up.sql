CREATE TABLE
  `credential` (
    `credential_pk` BINARY(16),
    `id` BINARY(16) NOT NULL,
    `name` VARCHAR(64) NOT NULL,
    PRIMARY KEY (`credential_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE UNIQUE INDEX `unique__id` ON `credential` (`id`);

CREATE UNIQUE INDEX `unique__name` ON `credential` (`name`);

CREATE TABLE
  `user_credential` (
    `user_credential_pk` BINARY(16),
    `user_pk` BINARY(16),
    `credential_pk` BINARY(16),
    `external_id` VARCHAR(255) NOT NULL,
    PRIMARY KEY (`user_credential_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE UNIQUE INDEX `unique__user_pk__credential_pk__external_id` ON `user_credential` (`user_pk`, `credential_pk`, `external_id`);

CREATE UNIQUE INDEX `unique__external_id` ON `user_credential` (`external_id`);

CREATE INDEX `index__user_pk` ON `user_credential` (`user_pk`);

CREATE INDEX `index__credential_pk` ON `user_credential` (`credential_pk`);

CREATE INDEX `index__credential_pk__external_id` ON `user_credential` (`credential_pk`, `external_id`);

CREATE INDEX `index__user_pk__credential_pk` ON `user_credential` (`user_pk`, `credential_pk`);

CREATE INDEX `index__user_pk__external_id` ON `user_credential` (`user_pk`, `external_id`);

INSERT INTO
  `credential` (`credential_pk`, `id`, `name`)
VALUES
  (
    0x018d938bdf7e7d27a440bc2ae08ed412,
    0xd2d8c8f225e14666a9007d241cc22c1b,
    'email'
  ),
  (
    0x018d938bdf7e73029d9fee4b47762169,
    0xa6ae875660b64b418356eb856bb5cca2,
    "onetime_password"
  );
