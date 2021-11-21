module.exports = {
  reactStrictMode: true,
  webpack(config) {
    config.experiments = {
      asyncWebAssembly: true,
    };
    config.module.rules.push({
      test: /\.(glsl|vs|fs|vert|frag)$/,
      loader: 'ts-shader-loader',
    });
    return config;
  },
};
