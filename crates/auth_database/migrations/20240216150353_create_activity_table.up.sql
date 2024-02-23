CREATE TABLE
  `activity` (
    `activity_pk` BINARY(16),
    `id` BINARY(16) NOT NULL,
    `name` VARCHAR(16) NOT NULL,
    PRIMARY KEY (`activity_pk`)
  );

CREATE UNIQUE INDEX `unique__id` ON `activity` (`id`);

CREATE UNIQUE INDEX `unique__name` ON `activity` (`name`);

CREATE TABLE
  `user_activity` (
    `user_pk` BINARY(16) NOT NULL,
    `activity_pk` BINARY(16) NOT NULL,
    `id` BINARY(16) NOT NULL,
    -- IPv4: BINARY(4)
    -- IPv6: BINARY(16)
    `ip_address` VARBINARY(16) NOT NULL,
    PRIMARY KEY (`user_pk`, `activity_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE UNIQUE INDEX `unique__id` ON `user_activity` (`id`);

CREATE INDEX `index__user_pk` ON `user_activity` (`user_pk`);
