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
sea-orm-cli migrate up
```
```bash
sea-orm-cli migrate down
```

## Start
Launch App
```bash
cargo run --bin app
```
