const path = require('path');
const webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const CopyPlugin = require("copy-webpack-plugin");
const WorkboxPlugin = require('workbox-webpack-plugin');

const dev = (argv) => argv.mode === 'development';

const styleLoader = (argv) => dev(argv) ? 'style-loader' : MiniCssExtractPlugin.loader;

const cssLoader = () => ({
  loader: 'css-loader',
  options: {
    modules: true,
  },
});

const babelLoader = () => ({
  loader: 'babel-loader',
  options: {
    presets: [              
      "@babel/preset-env",
      "@babel/preset-react",
    ],
    plugins: [
    ].filter(Boolean),
  },
});

module.exports = (env, argv) => ({
  mode: argv.mode,
  devtool: dev(argv) ? 'inline-source-map' : false,
  entry: path.join(__dirname, 'src/index.tsx'),
  output: {
    path: path.join(__dirname, 'dist'),
    filename: '[name].[hash].js',
    publicPath: '/',
  },
  resolve: {
    extensions: [
      '.js', '.jsx', '.ts', '.tsx', '.css', '.scss'
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
        // exclude: /node_modules/,
      },
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: babelLoader(),
      },
      {
        test: /\.(css)$/,
        exclude: /node_modules/,
        use: [styleLoader(argv), cssLoader()]
      },
      {
        test: /\.(scss)$/,
        exclude: /node_modules/,
        use: [styleLoader(argv), cssLoader(), 'sass-loader']
      }
    ],
  },
  plugins: [
    new MiniCssExtractPlugin({
      filename: '[name].[hash].css'
    }),
    new HtmlWebpackPlugin({
      template: 'index.html'
    }),
    new CopyPlugin({
      patterns: ["public"],
    }),
    dev(argv) ? false: new WorkboxPlugin.GenerateSW({
      clientsClaim: true,
      skipWaiting: true,
    }),
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
