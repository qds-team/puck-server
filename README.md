<h1 align="center">puck</h1>  
<p align="center">  
 <b> Powerful media server backend application, written in Rust, for downloading and reading manga.</b>  
</p>  

Setup Database

Install `sqlx-cli`
```
cargo install sqlx-cli
```

- Make sure to use `touch` to create `.env` file in project root.
- Then add db url for your `.db` file in `src/db` to it.
Create database at url
```
sqlx database create
```
Add pre-defined schema to `.db`
```
sqlx migrate run
```
Run/Build Project
```
cargo run
```
or
```
cargo build
```