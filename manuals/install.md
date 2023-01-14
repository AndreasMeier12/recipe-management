# Installing diesel

cargo install diesel_cli --no-default-features --features sqlite

If your installed sqlite happens to be too old, you can try adding the sqlite-bundled feature:

cargo install diesel_cli --no-default-features --features sqlite,sqlite-bundled

# Setting up Database

- Add `DATABASE_URL=[]` to .env file

- `diesel setup`
- `disel migration run`
