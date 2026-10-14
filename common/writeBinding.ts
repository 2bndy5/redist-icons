import * as path from "node:path";
import * as fs from "node:fs/promises";
import which from "which";
import { generatedCommentMarker, maxItems } from "./mod";

const endBindPart = "    Ok(())\n}\n";

export type GeneratedBinding = {
  lib: string;
  src: Array<string>;
};

/**
 * This function takes a generated source and writes it to appropriate files.
 * For optimization reasons, long Rust sources will be partitioned to no more than 200 items each.
 *
 * @remarks This function uses paths relative to the current working directory.
 *
 * @param rsSrc - The generated Rust source.
 */
export async function writePyBinding(rsSrc: GeneratedBinding) {
  const rootDir = path.join("src", "py_binding");
  await fs.rm(rootDir, { recursive: true, force: true });
  await fs.mkdir(rootDir, { recursive: true });

  const rootMod = path.join(rootDir, "mod.rs");
  const rootModImpl = [];

  const parts = Math.ceil(rsSrc.src.length / maxItems);
  const rootModSrc = [];

  const rootBegin = `use pyo3::prelude::*;
use crate::{Icon, finder::get_icon};

#[pymodule]
pub fn ${rsSrc.lib}(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_icon, m)?)?;
    m.add_class::<Icon>()?;
`;

  const mods = [rootMod];
  for (let i = 0; i < parts; i++) {
    const modName = `part_${i.toString().padStart(2, "0")}`;
    const modPart = path.join(rootDir, `${modName}.rs`);
    mods.push(modPart);

    const funcName = `bind_part_${i}`;

    const beginPart = `use pyo3::prelude::*;
use crate::icons;

pub(super) fn ${funcName}(m: &Bound<PyModule>) -> PyResult<()> {
`;
    const partition = rsSrc.src
      .slice(i * maxItems, Math.min(rsSrc.src.length, i * maxItems + maxItems))
      .join("");
    await fs.writeFile(
      modPart,
      generatedCommentMarker + beginPart + partition + endBindPart,
      "utf-8",
    );

    rootModSrc.push(`mod ${modName};\n`);
    rootModImpl.push(`    ${modName}::${funcName}(m)?;\n`);
  }
  await fs.writeFile(
    rootMod,
    generatedCommentMarker +
      rootModSrc.join("") +
      rootBegin +
      rootModImpl.join("") +
      endBindPart,
    "utf-8",
  );
  console.debug("Generated Python binding API via", mods.length, "modules");
}

export type GeneratedStubs = { lib: string; src: string };

/**
 * This function writes the given Python `stubs` to a .pyi file.
 *
 * @remarks This function uses a path relative to the current working directory.
 *
 * @param stubs The source of the Python type stubs.
 */
export async function writeStubs(stubs: GeneratedStubs) {
  const marker = generatedCommentMarker.replace("// ", "# ");
  const thisDir = process.cwd();
  const typeStubPath = path.join(thisDir, stubs.lib + ".pyi");
  await fs.writeFile(typeStubPath, marker + stubs.src, "utf-8");
  const uv = await which("uv");
  const result = Bun.spawnSync([
    uv,
    "run",
    "--no-sync",
    "ruff",
    "format",
    "-q",
    typeStubPath,
  ]);
  const stdout = result.stdout.toString().trim();
  const stderr = result.stderr.toString().trim();
  if (stdout.length) {
    console.log(stdout);
  }
  if (stderr.length) {
    console.log(stderr);
  }
  console.log("Generated and formatted Python type stubs");
}
