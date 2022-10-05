(() => {
  const { opSync } = globalThis.__bootstrap.core;

  globalThis.Armada = {
    get version() {
      return opSync("op_version");
    },
  };

  delete globalThis.__bootstrap;
})();
