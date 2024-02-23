CREATE TABLE
  `role` (
    `role_pk` BINARY(16),
    `id` BINARY(16) NOT NULL,
    `name` VARCHAR(16) NOT NULL,
    PRIMARY KEY (`role_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE UNIQUE INDEX `unique__id` ON `role` (`id`);

CREATE UNIQUE INDEX `unique__name` ON `role` (`name`);

CREATE TABLE
  `permission` (
    `permission_pk` BINARY(16),
    `id` BINARY(16) NOT NULL,
    `name` VARCHAR(16) NOT NULL,
    PRIMARY KEY (`permission_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE UNIQUE INDEX `unique__id` ON `permission` (`id`);

CREATE UNIQUE INDEX `unique__name` ON `permission` (`name`);

CREATE TABLE
  `role_permission` (
    `role_pk` BINARY(16),
    `permission_pk` BINARY(16),
    `id` BINARY(16) NOT NULL,
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`role_pk`, `permission_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE UNIQUE INDEX `unique__id` ON `role_permission` (`id`);

CREATE INDEX `index__role_pk` ON `role_permission` (`role_pk`);

CREATE INDEX `index__permission_pk` ON `role_permission` (`permission_pk`);

CREATE TABLE
  `user_role` (
    `user_pk` BINARY(16),
    `role_pk` BINARY(16),
    `id` BINARY(16) NOT NULL,
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`user_pk`, `role_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE UNIQUE INDEX `unique__id` ON `user_role` (`id`);

CREATE INDEX `index__user_pk` ON `user_role` (`user_pk`);

CREATE INDEX `index__role_pk` ON `user_role` (`role_pk`);

CREATE TABLE
  `user_permission` (
    `user_pk` BINARY(16),
    `permission_pk` BINARY(16),
    `id` BINARY(16) NOT NULL,
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`user_pk`, `permission_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE UNIQUE INDEX `unique__id` ON `user_permission` (`id`);

CREATE INDEX `index__user_pk` ON `user_permission` (`user_pk`);

CREATE INDEX `index__permission_pk` ON `user_permission` (`permission_pk`);
