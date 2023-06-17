
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