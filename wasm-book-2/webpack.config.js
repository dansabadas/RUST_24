const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, "static")
    ]),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
  module: {
    rules: [
      {
        test: /\.wasm$/,
        use: 'wasm-loader',
      },
    ],
  },
  // experiments: {
  //   asyncWebAssembly: true, // Enable async WebAssembly support
  // },
  // module: {
  //   rules: [
  //     {
  //       test: /\.wasm$/,
  //       type: 'webassembly/async', // Process .wasm files as async modules
  //     },
  //   ],
  // }
};
