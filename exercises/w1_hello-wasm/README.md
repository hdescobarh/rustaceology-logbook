# Hello Wasm 🦀 🕸

Doing the tutorial [Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm) from MDN Web Docs.

I used the latest LTS version of Node.JS (v18.13.0) and exact versions for dependencies.

## Some troubles I had  🕵🏻‍♂️


- An error related to "digital envelope routines":
Solved using the latest LTS version of Node.js and using apropiate devDependencies versions.

- Webpack asyncWebAssembly:
Webpack 5 requieres setting "experiments" in Webpack config.

- "Cannot GET /" message:
Required to add "devServer" in Webpack config. Also a modification in package.json.


*webpack.config.js*
```js
module.exports = {
    entry: "./index.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: 'index.js',
    },
    mode: "development",
    experiments: {
        futureDefaults: true,
    },
    devServer: {
        static: {
            directory: path.join(__dirname, '/')
        }
    },
};
```

*package.json*
```json
"scripts": {
        "start": "webpack serve"
    },
```

### References used to solve them 📚

- [How to resolve "Error: error:0308010C:digital envelope routines::unsupported" Nodejs 18 error](https://stackoverflow.com/questions/74548318/how-to-resolve-error-error0308010cdigital-envelope-routinesunsupported-no)

- [Error: error:0308010C:digital envelope routines::unsupported at new Hash (node:internal/crypto/hash:71:19)](https://stackoverflow.com/questions/73144960/error-error0308010cdigital-envelope-routinesunsupported-at-new-hash-nodei)

- [Webpack asyncWebAssembly Experiments](https://webpack.js.org/configuration/experiments/#experiments)

- [Webpack-dev-server "Cannot GET /"](https://stackoverflow.com/questions/71602863/webpack-dev-server-cannot-get)


