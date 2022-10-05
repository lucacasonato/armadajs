async function main() {
  Deno.core.print("Hello, World!\n");

  Deno.core.print(`armadajs version: ${Armada.version}\n`);

  const text = await Armada.readFile("LICENSE");
  Deno.core.print(text);
}

main();
