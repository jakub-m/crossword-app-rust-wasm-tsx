# Crossword generator app (Rust + WASM + React + TS)

[**Working app HERE**](https://jakub-m.github.io/crossword)

A small side project to exercise Rust as WASM on frontend.

# How to build locally for development

1. Go to `crossword-wasm`
2. `make all`
3. Go to `crossword-app`
4. Remove reference to `FinalizationRegistry` from `crossword.js` (no idea why it fails)
5. `npm start` or for webpack `npm run webpack-start`. Webpack should pack wasm in a way that can be later handled by
   jekyll.
6. To build run `npm run webpack-build`. This produces `dist/` directory.

# Related

- [Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm)
- [Rust + WASM + React + TypeScript; Better way to do imports?](https://www.reddit.com/r/rust/comments/ug332s/rust_wasm_react_typescript_better_way_to_do/)
- [How can I make webpack embed my wasm for use in a web worker?](https://stackoverflow.com/questions/70420273/how-can-i-make-webpack-embed-my-wasm-for-use-in-a-web-worker)

# Known issues
- I get error around `FinalizationRegistry`, don't know why. 

```
Line 131:11:  'FinalizationRegistry' is not defined  no-undef
```

Work around is:

```js
+(globalThis as any).FinalizationRegistry = undefined;
+// eslint-disable-next-line import/first
 import init_crossword_wasm from './crossword_wasm/crossword'
 ```

And `.eslintrc.json`

 ```json
 {
  "overrides": [
    {
      "files": ["crossword.js"],
      "rules": {
        "no-undef": "off"
      }
    }
  ]
}
 ```

- The generated crosswords are not globally optimal. E.g. "burak", "bacz", "kark", "zlepk" words do not produce a nice rectangle, but a suboptimal shape.
