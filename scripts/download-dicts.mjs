/**
 * 从 KyleBing/english-vocabulary 拉取全部「乱序」词表到项目 dict/ 目录（与源站格式一致：每行 单词\t释义）
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const ROOT = "https://raw.githubusercontent.com/KyleBing/english-vocabulary/master";
const OUT = path.join(__dirname, "..", "dict");

const FILES = [
  "1 初中-乱序.txt",
  "2 高中-乱序.txt",
  "3 四级-乱序.txt",
  "4 六级-乱序.txt",
  "5 考研-乱序.txt",
  "6 托福-乱序.txt",
  "7 SAT-乱序.txt",
];

async function main() {
  fs.mkdirSync(OUT, { recursive: true });
  for (const name of FILES) {
    const url = `${ROOT}/${encodeURIComponent(name)}`;
    process.stdout.write(`Fetching ${name} ... `);
    const res = await fetch(url);
    if (!res.ok) {
      console.error("failed", res.status);
      process.exit(1);
    }
    const buf = Buffer.from(await res.arrayBuffer());
    const dest = path.join(OUT, name);
    fs.writeFileSync(dest, buf);
    const lines = buf.toString("utf8").split(/\r?\n/).filter((l) => l.trim()).length;
    console.log(buf.length, "bytes,", lines, "lines");
  }
  console.log("Done ->", OUT);
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
