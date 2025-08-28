import * as path from "@std/path";
import { generatedCommentMarker, maxItems } from "common";

export type GeneratedFinder = {
  mod: string;
  src: Array<string>;
  sig?: string;
  invoke?: string;
};

const usePyo3 = `
#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
`;
const finderEndPart = "        _ => None,\n    }\n}\n";
const finderEnd = "    result\n}\n";

/**
 * This function takes a generated source and writes it to appropriate files.
 * For optimization reasons, long Rust sources will be partitioned to no more than 200 items each.
 *
 * @remarks This function uses paths relative to the current working directory.
 *
 * @param rsSrc - The generated Rust source.
 */
export async function writeFinder(rsSrc: GeneratedFinder) {
  const finderStart = `use crate::Icon;

  #[cfg(feature = "pyo3")]
  use pyo3::prelude::*;

  #[cfg_attr(feature = "pyo3", pyfunction)]
  pub fn get_icon(${rsSrc.sig ?? "slug: &str"}) -> Option<Icon> {
  `;
  const mods = [];
  const thisDir = Deno.cwd();
  const rootDir = path.join(thisDir, "src", rsSrc.mod);
  await Deno.remove(rootDir, { recursive: true });
  await Deno.mkdir(rootDir, { recursive: true });

  const rootMod = path.join(rootDir, "mod.rs");
  const rootModImpl = [];

  const parts = Math.ceil(rsSrc.src.length / maxItems);
  const rootModSrc = [];
  mods.push(rootMod);

  for (let i = 0; i < parts; i++) {
    const modName = `part_${i.toString().padStart(2, "0")}`;
    const modPart = path.join(rootDir, `${modName}.rs`);
    mods.push(modPart);
    const funcName = `find_part_${i}`;
    const partition = rsSrc.src.slice(
      i * maxItems,
      Math.min(rsSrc.src.length, i * maxItems + maxItems),
    ).join("");
    let sig = rsSrc.sig ?? "slug: &str";
    if (sig.includes("py: ") && !partition.includes(".print(py)")) {
      sig = sig.replace("py: ", "_py: ");
    }
    const usePyDeprecated = partition.includes("PyDeprecationWarning::")
      ? `
#[cfg(feature = "pyo3")]
use pyo3::exceptions::PyDeprecationWarning;
`
      : "";
    const beginPart = `use crate::{Icon, icons};\n${usePyDeprecated}
${rsSrc.sig?.includes("Python") ? usePyo3 : ""}
pub(super) fn ${funcName}(${sig}) -> Option<Icon> {
    match slug {\n`;
    await Deno.writeTextFile(
      modPart,
      generatedCommentMarker + beginPart + partition + finderEndPart,
    );

    rootModSrc.push(`mod ${modName};\n`);
    if (i == 0) {
      rootModImpl.push(
        `    let mut result = ${modName}::${funcName}(${
          rsSrc.invoke ?? "slug"
        });\n`,
      );
    } else {
      rootModImpl.push(`    if result.is_none() {
        result = ${modName}::${funcName}(${rsSrc.invoke ?? "slug"});\n    }\n`);
    }
  }
  Deno.writeTextFile(
    rootMod,
    generatedCommentMarker + rootModSrc.join("") + finderStart +
      rootModImpl.join("") +
      finderEnd,
  );
  console.log("Generated finder API via", mods.length, "modules");
}
