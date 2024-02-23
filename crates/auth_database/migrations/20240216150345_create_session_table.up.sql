CREATE TABLE
  `user_session` (
    `user_session_pk` BINARY(16),
    `id` BINARY(16) NOT NULL,
    `user_pk` BINARY(16) NOT NULL,
    -- IPv4: BINARY(4)
    -- IPv6: BINARY(16)
    `ip_address` VARBINARY(16) NOT NULL,
    `expired_at` DATETIME NOT NULL,
    PRIMARY KEY (`user_session_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE UNIQUE INDEX `unique__id` ON `user_session` (`id`);

CREATE INDEX `index__user_pk` ON `user_session` (`user_pk`);
