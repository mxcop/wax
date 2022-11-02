## Wax Project Structure

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

<br>

## Useage

```
cargo run -- <path>
```

<br>

### Static Components

<div><sub>Example : Component File</sub></div>

```html
~ src/lib/my-component.html

<p>
  Hello from my component ! :D
</p>
```

<div><sub>Example : Importing</sub></div>

```html
~ src/routes/index.html

<body>
  …
  <wax! src="../lib/my-component.html" />
  …
</body>
```

### Dynamic Components *

<div><sub>Example : Component File</sub></div>

```html
~ src/lib/my-component.html

<params! title>

<h1>
  { title }
</h1>
```

<div><sub>Example : Importing</sub></div>

```html
~ src/routes/index.html

<body>
  …
  <wax! src="../lib/my-component.html" title="My dynamic title" />
  …
</body>
```

<br>

## Testing

```
cargo run -- ./assets
```
