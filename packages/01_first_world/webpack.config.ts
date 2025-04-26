const path=require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const { merge } = require('webpack-merge');
const baseConfig = require('../../config/webpack/webpack.base.ts'); // 调整路径以匹配你的目录结构

module.exports = merge(baseConfig, {
    plugins: [
        new HtmlWebpackPlugin(),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "../../crates/01-first-world"),
        }),
    ],
});
