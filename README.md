# Mandelbrot Fractal with Rust and WebAssembly

You can view a live demo of this project hosted on GitHub Pages: [Mandelbrot Fractal Live Demo](https://bemayer.github.io/mandelbrot-wasm/).

This project is a simple experiment to generate and display the Mandelbrot fractal using Rust and WebAssembly (WASM). The goal is to perform the minimum in HTML and leverage Rust's performance to compute and render the fractal directly in the browser. Notably, the Document Object Model (DOM) is modified directly by the WebAssembly code itself, without relying on external JavaScript for DOM manipulation.

## Description
- **Rust and WebAssembly**: The entire logic, including computations and DOM manipulation, is written in Rust and compiled to WebAssembly for efficient performance in the browser.
- **Minimal HTML**: The HTML (index.html) is minimal, containing only the essential script to load and run the WebAssembly module.
- **Direct DOM Manipulation**: The WebAssembly code modifies the DOM directly. It creates and appends HTML elements like the button, canvas, and render time display entirely from Rust code.
- **Interactive Rendering**: A button allows users to render the Mandelbrot fractal on a canvas element.
- **Performance Display**: The time taken to render the fractal is displayed on the page.

## Usage

1. **Build the WebAssembly Module**

Ensure you have wasm-pack installed:
```bash
cargo install wasm-pack
```

2. **Build the project with:**

```bash
wasm-pack build --target web
```

This will generate a pkg folder containing the WebAssembly binary and JavaScript bindings.

3. **Serve the Files**

WebAssembly modules must be served over HTTP. Use a simple HTTP server:

Using Python 3:

```bash
python3 -m http.server 8080
```
Using Node.js http-server:

```bash
npx http-server . -p 8080
```

Navigate to http://localhost:8080 in your web browser.

4. **Render the Fractal**

Click the *Render* button to generate and display the Mandelbrot fractal. The render time will be shown below the canvas.

## Files

- `index.html`

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Mandelbrot Fractal</title>
</head>
<body>
    <script type="module">
        import init from './pkg/mandelbrot_wasm.js';

        async function run() {
            await init();
        }

        run();
    </script>
</body>
</html>
```

- `src/lib.rs`
  
  Contains the Rust code that:
  - Calculates the Mandelbrot set.
  - Directly manipulates the DOM to create and append elements like the button, canvas, and render time display.
  - Handles the click event for rendering the fractal.
  - Updates the page with the render time.

## How It Works

- DOM Manipulation in Rust:
  The Rust code uses wasm-bindgen and web-sys crates to interact with the browser's DOM APIs directly. This allows the WebAssembly module to:
  - Create HTML elements like `<button>` and `<canvas>`.
  - Append these elements to the document body.
  - Set event listeners (e.g., onclick for the render button).
  - Update content dynamically (e.g., display render time).

- Rendering the Fractal:
  When the Render button is clicked:
  - The Rust function render_mandelbrot is called.
  - It computes the Mandelbrot set for each pixel.
  - The resulting image data is rendered onto the canvas.
  - The time taken to render is calculated and displayed.

## License

This project is open-source and available under the MIT License.
