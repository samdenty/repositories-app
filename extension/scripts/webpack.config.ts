import * as webpack from "webpack";
import ForkTsCheckerWebpackPlugin from "fork-ts-checker-webpack-plugin";
import path from "path";
import MiniCssExtractPlugin from "mini-css-extract-plugin";
import { sass } from "svelte-preprocess-sass";
const ExtensionReloader = require("webpack-extension-reloader");

// import HtmlWebpackPlugin from "html-webpack-plugin";
// import MiniCssExtractPlugin from "mini-css-extract-plugin";

const config: webpack.Configuration = {
  target: "web",
  context: path.resolve(".."),
  entry: {
    background: "./src/background/background",
    content: "./src/content/content",
  },
  devtool: "source-map",

  output: {
    path: path.resolve("../dist"),
    filename: "[name].js",
    globalObject: "globalThis",
    libraryTarget: "umd",
    umdNamedDefine: true,
  },
  resolve: {
    alias: {
      svelte: path.resolve("../node_modules/svelte"),
    },
    mainFields: ["svelte", "browser", "module", "main"],
    extensions: [".ts", ".tsx", ".js", ".mjs", ".svelte", ".json"],
  },

  module: {
    rules: [
      {
        test: /\.s?css$/i,
        use: [MiniCssExtractPlugin.loader, "css-loader", "sass-loader"],
      },
      {
        test: /\.tsx?$/,
        exclude: /node_modules/,
        use: [
          {
            loader: "ts-loader",
            options: {
              transpileOnly: true,
            },
          },
        ],
      },
      {
        test: /\.(html|svelte)$/,
        exclude: /node_modules/,
        use: {
          loader: "svelte-loader",
          options: {
            emitCss: true,
            preprocess: require("svelte-preprocess")({
              style: sass(),
            }),
          },
        },
      },
      {
        test: /\.woff$/,
        use: [
          {
            loader: "file-loader",
            options: {
              name: "[name].[ext]",
            },
          },
        ],
      },
    ],
  },
  plugins: [
    new MiniCssExtractPlugin({
      filename: "styles.css",
    }),
    new ExtensionReloader({
      reloadPage: true,
    }),
    new ForkTsCheckerWebpackPlugin({
      typescript: {
        build: true,
        configFile: path.resolve("../tsconfig.json"),
      },
    }),
  ],

  stats: {
    warnings: false,
  },
  node: false,
};

export default config;
