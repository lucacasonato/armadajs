(() => {
  const { opSync, opAsync, print } = globalThis.__bootstrap.core;
  const { Console } = globalThis.__bootstrap.console;

  globalThis.Armada = {
    get version() {
      return opSync("op_version");
    },
    readFile(path) {
      return opAsync("op_read_file", path);
    },
  };

  function printer(msg, logLevel) {
    const useStderr = logLevel > 1; // 0 = debug, 1 = info, 2 = warn, 3 = error
    print(msg, useStderr);
  }

  globalThis.console = new Console(printer);

  delete globalThis.__bootstrap;
})();
