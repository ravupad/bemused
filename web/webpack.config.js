const path = require('path');
const webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const dev = (argv) => argv.mode === 'development';

module.exports = (env, argv) => ({
  mode: argv.mode,
  devtool: dev(argv) ? 'inline-source-map' : false,
  entry: path.join(__dirname, 'src/index.tsx'),
  output: {
    path: path.join(__dirname, 'dist'),
    filename: '[name].[hash].js',
  },
  resolve: {
    extensions: [
      '.js',
      '.jsx',
      '.ts',
      '.tsx',
      '.css',
      '.scss',
    ],
    alias: {
      css: path.resolve(__dirname, 'src/css'),
    },
    modules: [path.resolve(__dirname, 'src'), "node_modules"],
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: [              
              "@babel/preset-env",
              "@babel/preset-react",
            ],
            plugins: [
            ].filter(Boolean),
          },
        },
      },
      {
        test: /\.(css|scss)$/,
        exclude: /node_modules/,
        use: dev(argv)
           ? 'style-loader'
           : MiniCssExtractPlugin.loader
      },
      {
        test: /\.(css|scss)$/,
        exclude: /node_modules/,
        use: {
          loader: 'css-loader',
          options: {
            modules: {
              localIdentName: "[local]_[hash:base64:5]",
            },
          },
        },
      },
      {
        test: /\.(scss)$/,
        exclude: /node_modules/,
        use: ['sass-loader']
      }
    ],
  },
  plugins: [
    new MiniCssExtractPlugin(),
    new HtmlWebpackPlugin(),
  ].filter(Boolean),
  devServer: {
    contentBase: path.join(__dirname, 'dist'),
    compress: true,
    port: 9000,
    historyApiFallback: {
      index: '/',
    },
    proxy: {
      '/api': {
        target: 'http://localhost:8081',
      },
    },
  }
});
