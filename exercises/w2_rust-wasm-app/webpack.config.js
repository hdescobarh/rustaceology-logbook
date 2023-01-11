const path = require("path");
module.exports = {
    entry: "./source/bootstrap.js",
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
            directory: path.join(__dirname, 'source')
        },
        port: 9000,
    },
};