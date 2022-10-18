### Wax Project Structure

```graphql
./<my-wax-site>/* 
  ├─ .wax/     - # Wax file cache
  │
  ├─ src/*         - # Your codebase (html, css, and js)
  │  ├─ lib/       - # Html components
  │  ├─ routes/    - # Location of all your html pages
  │  ├─ wax.toml   - # Wax config file
  │  └─ ...
  │
  └─ build/    - # Wax build output
```

### Useage

```
cargo run -- <path>
```

### Testing

```
cargo run -- ./assets
```
