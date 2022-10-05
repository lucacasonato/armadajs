async function main() {
  console.log("Hello, World!\n");

  console.log(`armadajs version: ${Armada.version}\n`);

  const text = await Armada.readFile("LICENSE");
  console.log(text);

  console.log({
    a: {
      deeply: {
        nested: [
          "object",
          "that",
          "includes",
          "primitives",
          "like",
          1,
          "and",
          true,
        ],
      },
    },
  });

  const resp = await fetch("https://hello.deno.dev");
  console.log(resp);
  console.log(await resp.text());
}

main();
