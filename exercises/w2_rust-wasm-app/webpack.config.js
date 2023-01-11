const path = require("path");
module.exports = {
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: 'bootstrap.js',
    },
    mode: "development",
    experiments: {
        futureDefaults: true,
    },
    devServer: {
        static: {
            directory: path.join(__dirname, '/')
        },
        port: 9000,
    },
};