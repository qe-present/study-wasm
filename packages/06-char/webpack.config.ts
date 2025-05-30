const path=require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const { merge } = require('webpack-merge');
const baseConfig = require('../../config/webpack/webpack.base.ts');

module.exports = merge(baseConfig, {
    plugins: [
        new HtmlWebpackPlugin({
            template:"index.html",
        }),

        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "../../crates/06-char"),
        }),
    ]
})