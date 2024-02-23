default:
  sqlx migrate run --source=crates/auth_database/migrations

undo:
  sqlx migrate revert --source=crates/auth_database/migrations

new name:
  sqlx migrate add -r {{name}} --source=crates/auth_database/migrations

info:
  sqlx migrate info --source=crates/auth_database/migrations
