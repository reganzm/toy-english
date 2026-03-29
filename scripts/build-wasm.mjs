import { spawnSync } from "node:child_process";
import path from "node:path";
import fs from "node:fs";
import os from "node:os";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const exe = process.platform === "win32" ? "wasm-pack.exe" : "wasm-pack";
const cargoWasmPack = path.join(os.homedir(), ".cargo", "bin", exe);
const cmd = fs.existsSync(cargoWasmPack) ? cargoWasmPack : exe;

// out-dir 相对于 crate 根目录（wasm-game/），写 pkg 即得到 wasm-game/pkg/
const args = ["build", "wasm-game", "--target", "web", "--out-dir", "pkg"];
const r = spawnSync(cmd, args, {
  stdio: "inherit",
  cwd: root,
  shell: cmd === exe,
});

if (r.error) {
  console.error(r.error.message);
  console.error(
    "\n请先安装：rustup + wasm32 目标，并执行：cargo install wasm-pack\n" +
      "Windows 也可：irm https://rustwasm.github.io/wasm-pack/installer/init.ps1 | iex\n"
  );
  process.exit(1);
}
process.exit(r.status ?? 1);
