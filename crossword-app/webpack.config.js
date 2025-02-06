const path = require('path');
const webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  cache: false,
  entry: './src/index.tsx',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        //use: 'babel-loader',
        exclude: /node_modules/,
      },
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader"],
      },
      {
        test: /\.wasm$/i,
        type: "asset/inline",
        //use: [
        //  {
        //    loader: 'raw-loader',
        //    options: {
        //      esModule: false,
        //    },
        //  },
        //],
      }
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  plugins: [
    new webpack.DefinePlugin({
      PUBLIC_URL: "foo",
    }),
    new HtmlWebpackPlugin({
      template: './public/index.html'
    })

  ],
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'dist'),
  },
};
