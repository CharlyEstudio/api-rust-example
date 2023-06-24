
## Init Config Setup Diesel
docker-compose exec app diesel setup

### Generate Table
docker-compose exec app diesel migration generate create_<name_table>

### After create files SQL
docker-compose exec app diesel migration run

### Revert migration
docker-compose exec app diesel migration revert

### Up App
docker-compose exec app cargo run

## Execute Bin into folder src/bin/cli.rs
docker-compose exec app cargo run --bin cli

### Up Test
docker-compose exec app cargo test

### Create Doc & Open
docker-compose exec app cargo doc
docker-compose exec app cargo doc --open
