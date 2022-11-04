<h2 align="center">
  <img src="https://raw.githubusercontent.com/mxcop/wax/main/.github/icon.svg" width="80px">
  <div>
    Wax 
  </div>
  <div>
    <sup>* Collapse your HTML *</sup>
  </div>
  <a href="https://crates.io/crates/wax-cli">
    <img align="right" src="https://img.shields.io/crates/v/wax-cli?color=blueviolet">
    <img align="right" src="https://img.shields.io/crates/l/wax-cli">
    <img align="right" src="https://img.shields.io/crates/d/wax-cli">
  </a>
</h2>

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

## Project Structure

```graphql
./<my-wax-site>/* 
  │
  ├─ .wax       - # Wax file cache
  ├─ dist       - # Wax build output
  │
  ├─ src/*      - # Your codebase (html, css, and js)
  │  ├─ lib/       - # Html wax components
  │  ├─ pages/     - # Your html pages
  │  └─ ...
  │
  └─ wax.toml   - # Wax config file
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

## Config

<div><sup>* <strong>?</strong> : means it is optional</sup></div>

```toml
# wax.toml

[website]
pages = "RelativePath"  # Path to the directory containing your index.html.

[build]
minify = "Boolean?"     # If enabled, will minify the collapsed HTML files.
```

<br>

## Testing

<div><sup>* Using the testing project in <code>./assets</code></sup></div>

```
$ cargo run build ./assets
```
