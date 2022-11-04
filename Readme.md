## Wax Project Structure

```graphql
./<my-wax-site>/* 
  ├─ src/*         - # Your codebase (html, css, and js)
  │  ├─ .wax       - # Wax file cache
  │  ├─ lib/       - # Html components
  │  ├─ routes/    - # Location of all your html pages
  │  ├─ wax.toml   - # Wax config file
  │  └─ ...
  │
  └─ build/    - # Wax build output
```

<br>

## Install

Install the Wax cli using cargo :
```
$ cargo install wax-cli
```

## Useage

<div><sup>Create new project</sup></div>

```md
$ wax create <NAME>
```

<div><sup>Build your project</sup></div>

```md
$ wax build <PATH>
```

<br>

### Static Components

Wax components are an extension of html.

<div><sup>Example : Component File</sup></div>

```html
~ src/lib/my-component.html

<p>
  Hello from my component ! :D
</p>
```
<br>

Importing / including wax components is done using the <code><wax!></code> tag.<br>
<i>e.g.</i> <code><wax! … src="[path]" … ></code>

<div><sup>Example : Importing</sup></div>

```html
~ src/routes/index.html

<body>
  …
  <wax! src="../lib/my-component.html" />
  …
</body>
```
<br>

### Dynamic Components *

Passing parameters to a component is done using html attributes.<br>
<i>e.g.</i> <code><wax! … [key]="[value]" … ></code>

<div><sup>Example : Importing</sup></div>

```html
~ src/routes/index.html

<body>
  …
  <wax! src="../lib/my-component.html" title="My dynamic title" />
  …
</body>
```
<br>

Each component has to declare its parameters using <code><params! … [key] … ></code><br>
Parameters can be inserted into the html using <code>{ [key] }</code>

<div><sup>Example : Component File</sup></div>

```html
~ src/lib/my-component.html

<params! title>

<h1>
  { title }
</h1>
```

<br>

## Testing

<div><sup>* Using the testing project in <code>./assets</code></sup></div>

```
$ cargo run build ./assets
```
