CREATE TABLE
  `user_group` (
    `user_group_pk` BINARY(16),
    `id` BINARY(16) NOT NULL,
    `name` VARCHAR(16) NOT NULL,
    PRIMARY KEY (`user_group_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE UNIQUE INDEX `unique__id` ON `user_group` (`id`);

CREATE UNIQUE INDEX `unique__name` ON `user_group` (`name`);

CREATE TABLE
  `user_group__has__user` (
    `user_group_pk` BINARY(16),
    `user_pk` BINARY(16),
    `id` BINARY(16) NOT NULL,
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`user_group_pk`, `user_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE UNIQUE INDEX `unique__id` ON `user_group__has__user` (`id`);

CREATE INDEX `index__group_pk` ON `user_group__has__user` (`user_group_pk`);

CREATE INDEX `index__user_pk` ON `user_group__has__user` (`user_pk`);

CREATE TABLE
  `user_group__has__role` (
    `user_group_pk` BINARY(16),
    `role_pk` BINARY(16),
    `id` BINARY(16) NOT NULL,
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`user_group_pk`, `role_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE UNIQUE INDEX `unique__id` ON `user_group__has__role` (`id`);

CREATE INDEX `index__group_pk` ON `user_group__has__role` (`user_group_pk`);

CREATE INDEX `index__role_pk` ON `user_group__has__role` (`role_pk`);

CREATE TABLE
  `user_group__has__permission` (
    `user_group_pk` BINARY(16),
    `permission_pk` BINARY(16),
    `id` BINARY(16) NOT NULL,
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`user_group_pk`, `permission_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE UNIQUE INDEX `unique__id` ON `user_group__has__permission` (`id`);

CREATE INDEX `index__group_pk` ON `user_group__has__permission` (`user_group_pk`);

CREATE INDEX `index__permission_pk` ON `user_group__has__permission` (`permission_pk`);
