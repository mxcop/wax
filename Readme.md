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

Wax components are an extension of html.

<div><sub>Example : Component File</sub></div>

```html
~ src/lib/my-component.html

<p>
  Hello from my component ! :D
</p>
```

Importing / including wax components is done using the <code><wax!></code> tag.<br>
<i>e.g.</i> <code><wax! … src="[path]" … ></code>

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

Passing parameters to a component is done using html attributes.<br>
<i>e.g.</i> <code><wax! … [key]="[value]" … ></code>

<div><sub>Example : Importing</sub></div>

```html
~ src/routes/index.html

<body>
  …
  <wax! src="../lib/my-component.html" title="My dynamic title" />
  …
</body>
```

Each component has to declare its parameters using <code><params! … [key] … ></code><br>
Parameters can be inserted into the html using <code>{ [key] }</code>

<div><sub>Example : Component File</sub></div>

```html
~ src/lib/my-component.html

<params! title>

<h1>
  { title }
</h1>
```

<br>

## Testing

```
cargo run -- ./assets
```
