# Installing diesel

cargo install diesel_cli --no-default-features --features sqlite

# Setting up Database

- Add `DATABASE_URL=[]` to .env file

- `diesel setup`
- `disel migration run`
