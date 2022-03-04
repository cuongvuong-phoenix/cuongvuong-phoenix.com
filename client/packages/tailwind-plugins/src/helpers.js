module.exports = {
  getSpacings: (theme) => {
    return Object.entries(theme('spacing')).map(([key, value]) => [key.replace('.', '\\.'), value]);
  },
};
