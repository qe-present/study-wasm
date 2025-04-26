const path=require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
    entry: './src/index.ts',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    resolve: {
        alias:{
            "@crates/*": path.resolve(__dirname, "../../crates/*"),
        }
    },
    devServer: {
        hot: true,
        static: {
            directory: path.join(__dirname, 'dist'),
        },
        port: 3000,
    },
    plugins: [
        new HtmlWebpackPlugin({
            template:"index.html",
        }),
        
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "../../crates/08-dom"),
        }),
    ],
    mode: 'development',
    experiments: {
        asyncWebAssembly: true
    }
};