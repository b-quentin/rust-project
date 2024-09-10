# Blog Api

## Database
Launch database
```bash
docker compose up -d
```
```bash
sea-orm-cli migrate init
```
```bash
export DATABASE_URL="postgresql://myuser:mypassword@localhost:5432/mydatabase"
sea-orm-cli migrate up
unset DATABASE_URL
```
```bash
sea-orm-cli migrate down
```

## Start
Launch App
```bash
cargo run --bin app
```
