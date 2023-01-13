const path = require("path");
const HtmlWebpackPlugin = require('html-webpack-plugin');
module.exports = {
    entry: "./source/index.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: 'index_bundle.js',
        publicPath: "/"
    },
    plugins: [new HtmlWebpackPlugin({
        template: './source/index.html'
    })],
    mode: "development",
    module: {
        rules: [
            {
                test: /\.jsx?$/,
                exclude: /node_modules/,
                use: {
                    loader: "babel-loader",
                    options: {
                        cacheDirectory: true,
                        cacheCompression: false,
                        envName: "development",
                        presets: ['@babel/preset-env', "@babel/preset-react"]
                    },



                }
            }
        ]
    },
    resolve: {
        extensions: [".js", ".jsx"]
    },
    experiments: {
        futureDefaults: true,
    },
    devServer: {
        static: {
            directory: path.join(__dirname, 'source')
        },
        port: 9000,
        open: true,
    },
};