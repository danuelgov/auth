CREATE TABLE
  `before_signup` (
    `before_signup_pk` BINARY(16),
    `id` BINARY(16) NOT NULL,
    `payload` TEXT NOT NULL,
    `expired_at` DATETIME NOT NULL,
    `completed_at` DATETIME NOT NULL DEFAULT '9999-12-31 23:59:59',
    PRIMARY KEY (`before_signup_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE UNIQUE INDEX `unique__id` ON `before_signup` (`id`);

CREATE TABLE
  `before_new_password` (
    `before_new_password_pk` BINARY(16),
    `id` BINARY(16) NOT NULL,
    `user_credential_pk` BINARY(16) NOT NULL,
    `expired_at` DATETIME NOT NULL,
    `completed_at` DATETIME NOT NULL DEFAULT '9999-12-31 23:59:59',
    PRIMARY KEY (`before_new_password_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE UNIQUE INDEX `unique__id` ON `before_new_password` (`id`);

CREATE INDEX `index__user_credential_pk` ON `before_new_password` (`user_credential_pk`);
