const fs = require("fs");
const path = require("path");
const mjml2html = require("mjml");

if (process.argv.length < 3) {
  console.info("usage: npm start -- path/to/file.mjml");
  process.exit(1);
}

const filename = path.resolve(process.argv[2]);
const content = fs.readFileSync(filename).toString("utf8");

console.time("mjml");
for (let i = 0; i < 100; i++) {
  mjml2html(content);
}
console.timeEnd("mjml");
