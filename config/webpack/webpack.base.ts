
const path=require('path');

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

    ],
    mode: 'development',
    experiments: {
        asyncWebAssembly: true
    }
};