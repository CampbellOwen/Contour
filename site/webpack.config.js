const CopyWebpackPlugin = require('copy-webpack-plugin');
const path = require('path');

module.exports = {
  entry: path.resolve(__dirname, "src", "index.ts"),
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bundle.js",
  },
  mode: "development",
  module: {
      rules: [
          {
              test: /\.tsx?$/,
              use: 'ts-loader',
              exclude: /node_modules/,
          },
      ]
  },
  resolve: {
      extensions: ['.tsx', '.ts', '.js']
  },
  plugins: [
      new CopyWebpackPlugin({
          patterns: [
              {
                  from: path.resolve(__dirname, 'src', 'index.html'), 
                  to: path.resolve(__dirname, 'dist', 'index.html')
               }]})
  ],
  devServer: {
      contentBase: path.join(__dirname, 'dist'),
      compress: true,
      port: 9000
  },
  experiments: {
      syncWebAssembly: true,
      topLevelAwait: true
  }
};