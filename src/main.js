(() => {
  const { opSync, opAsync } = globalThis.__bootstrap.core;

  globalThis.Armada = {
    get version() {
      return opSync("op_version");
    },
    readFile(path) {
      return opAsync("op_read_file", path);
    },
  };

  delete globalThis.__bootstrap;
})();
