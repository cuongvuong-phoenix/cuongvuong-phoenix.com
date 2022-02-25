module.exports = {
  client: {
    service: {
      name: 'cvp-web',
      url: 'http://localhost:7878/api/graphql',
    },
    includes: ['src/**/*.js', 'src/**/*.ts', 'src/**/*.vue'],
  },
};
