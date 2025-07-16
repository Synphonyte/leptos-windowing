# Basic Pagination Example

This example demonstrates the basic usage of leptos-windowing for efficiently rendering large lists with pagination. The styling is handled using Tailwind CSS classes.

To make this example work, you must download / fork the whole repo because this is in the dependencies: `leptos-windowing = { path = "../.." }`.

If you don't have already installed it, install [Trunk](https://trunkrs.dev/) and [Tailwind](https://tailwindcss.com/docs/installation)
as well as the `wasm32-unknown-unknown` target:

```bash
cargo install trunk
npm install -D tailwindcss
rustup target add wasm32-unknown-unknown
```

Then, open two terminals. In the first one, run:

```bash
npx tailwindcss -i ./input.css -o ./style/output.css --watch
```

In the second one, run:

```bash
trunk serve --open
```
