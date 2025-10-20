const webpack = require("webpack");

module.exports = function override(config, env) {
  config.resolve.fallback = {
    ...config.resolve.fallback,
    crypto: require.resolve("crypto-browserify"),
    stream: require.resolve("stream-browserify"),
    buffer: require.resolve("buffer"),
    process: require.resolve("process/browser.js"),
    util: require.resolve("util"),
    assert: require.resolve("assert"),
    url: require.resolve("url"),
    os: require.resolve("os-browserify"),
    path: require.resolve("path-browserify"),
    vm: require.resolve("vm-browserify"),
    fs: false,
    net: false,
    tls: false,
    child_process: false,
  };

  config.plugins = [
    ...config.plugins,
    new webpack.ProvidePlugin({
      process: "process/browser",
      Buffer: ["buffer", "Buffer"],
    }),
  ];

  config.ignoreWarnings = [
    {
      module: /@coral-xyz\/anchor/,
    },
    {
      message: /Failed to parse source map/,
    },
    {
      message: /Can't resolve 'process\/browser'/,
    },
    {
      message: /Can't resolve 'vm'/,
    },
  ];

  config.module.rules.push({
    test: /\.m?js/,
    resolve: {
      fullySpecified: false,
    },
  });

  return config;
};
