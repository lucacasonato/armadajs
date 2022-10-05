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
          "numbers",
          "like",
          1,
          "and",
          2n,
        ],
      },
    },
  });
}

main();
