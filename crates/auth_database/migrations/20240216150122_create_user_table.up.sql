CREATE TABLE
  `user` (
    `user_pk` BINARY(16),
    `id` BINARY(16) NOT NULL,
    `deactivated_at` DATETIME NOT NULL DEFAULT '9999-12-31 23:59:59',
    PRIMARY KEY (`user_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE UNIQUE INDEX `unique__id` ON `user` (`id`);

CREATE TABLE
  `user_profile` (
    `user_pk` BINARY(16),
    `handle` VARCHAR(20) NOT NULL,
    `name` VARCHAR(64) NOT NULL,
    `bio` TEXT NOT NULL,
    `image` VARCHAR(255) NOT NULL,
    PRIMARY KEY (`user_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE UNIQUE INDEX `unique__handle` ON `user_profile` (`handle`);

CREATE UNIQUE INDEX `unique__name` ON `user_profile` (`name`);
