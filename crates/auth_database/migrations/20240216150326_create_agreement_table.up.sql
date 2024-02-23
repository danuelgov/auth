CREATE TABLE
  `policy` (
    `policy_pk` BINARY(16),
    `name` VARCHAR(16) NOT NULL,
    PRIMARY KEY (`policy_pk`)
  );

CREATE UNIQUE INDEX `unique__name` ON `policy` (`name`);

CREATE TABLE
  `agreement` (
    `agreement_pk` BINARY(16),
    `id` BINARY(16) NOT NULL,
    `policy_pk` BINARY(16) NOT NULL,
    `name` VARCHAR(16) NOT NULL,
    PRIMARY KEY (`agreement_pk`)
  );

CREATE UNIQUE INDEX `unique__id` ON `agreement` (`id`);

CREATE UNIQUE INDEX `unique__name` ON `agreement` (`name`);

CREATE INDEX `index__policy_pk` ON `agreement` (`policy_pk`);

CREATE TABLE
  `user_agreement` (
    `user_pk` BINARY(16) NOT NULL,
    `agreement_pk` BINARY(16) NOT NULL,
    `expired_at` DATETIME NOT NULL DEFAULT '9999-12-31 23:59:59',
    PRIMARY KEY (`user_pk`, `agreement_pk`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

CREATE INDEX `index__user_pk` ON `user_agreement` (`user_pk`);

CREATE INDEX `index__agreement_pk` ON `user_agreement` (`agreement_pk`);

INSERT INTO
  `policy` (`policy_pk`, `name`)
VALUES
  (
    0x018d9bd868347a69a7185d148310c7a1,
    "privacy_policy"
  ),
  (
    0x018d9bd868347a20bc0fb402d3205150,
    "terms_of_service"
  );

INSERT INTO
  `agreement` (`agreement_pk`, `id`, `policy_pk`, `name`)
VALUES
  (
    0x018d9bd868347d8e9e279e77f5a4fdfb,
    0x9aacb6a19f1940d2985401c33876fee5,
    0x018d9bd868347a69a7185d148310c7a1,
    "privacy_policy"
  ),
  (
    0x018d9bd868347bda8b6e2f979a47d57b,
    0xa866525497a54a9bab7d1bcd98ad7d59,
    0x018d9bd868347a20bc0fb402d3205150,
    "terms_of_service"
  );
