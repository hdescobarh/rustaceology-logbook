const path = require("path");
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