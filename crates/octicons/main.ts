import { walk } from "@std/fs";
import { loadConfig, optimize } from "npm:svgo";
import {
  copyPkgMeta,
  type GeneratedBinding,
  type GeneratedFinder,
  type GeneratedIcons,
  type GeneratedStubs,
  rustFmtSrc,
  writeConstIcons,
  writeFinder,
  writePyBinding,
  writeStubs,
} from "common";
import metadata from "@primer/octicons/build/data.json" with { type: "json" };
import npmPkgInfo from "@primer/octicons/package.json" with { type: "json" };

const npmPkg = "@primer/octicons";

// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  console.log("Generating crate from", npmPkg, "source.");
  const crateInfo = await copyPkgMeta(npmPkg, npmPkgInfo, "LICENSE");

  const svgoConfig = await loadConfig("../../svgo.config.mjs");

  const svgFiles = await Array.fromAsync(
    walk(`../../node_modules/${npmPkg}/build/svg`),
  );

  const textEncoder = new TextEncoder();

  const rsLibIcons: GeneratedIcons = {
    mod: "icons",
    src: [],
  };

  const rsLibFinder: GeneratedFinder = { mod: "finder", src: [] };

  const rsBinding: GeneratedBinding = { lib: crateInfo.lib, src: [] };

  const pyStub: GeneratedStubs = {
    lib: crateInfo.lib,
    src: `from typing import NamedTuple, Final

class Icon(NamedTuple):
    svg: str
    slug: str

def get_icon(slug: str) -> Icon | None: ...

`,
  };

  for (const entry of svgFiles.entries()) {
    if (entry[1].isDirectory) {
      continue;
    }
    const name = entry[1].name.substring(0, entry[1].name.length - 4);
    const slug = name.replace(/\-?\d*$/, "");
    const iconMeta = metadata[<keyof typeof metadata> slug];
    if (!iconMeta) {
      throw new Error(
        `Name "${slug}" was not found in the ${npmPkg} package metadata JSON`,
      );
    }

    const constName = name.toUpperCase().replaceAll("-", "_");

    rsLibFinder.src.push(`        "${name}" => Some(icons::${constName}),\n`);
    rsBinding.src.push(`    m.add("${constName}", icons::${constName})?;\n`);
    pyStub.src += `${constName}: Final[Icon] = ...\n`;

    if (Deno.stdin.isTerminal()) {
      // only write progress indicator if in interactive terminal
      const percent = Math.round(Number(entry[0] / svgFiles.length * 100))
        .toString().padStart(3);
      await Deno.stdout.write(
        textEncoder.encode(`Generating... ${percent}%\r`),
      );
    }

    const svgData = await Deno.readTextFile(`${entry[1].path}`);
    const optimized = optimize(svgData, svgoConfig);

    rsLibIcons.src.push(`
/// ${name}
pub const ${constName}: Icon = Icon {
    svg: r#"${optimized.data}"#,
    slug: "${name}",\n};\n`);
  }

  console.log("\nGenerated", svgFiles.length, "icons");

  await writeConstIcons([rsLibIcons]);
  await writeFinder(rsLibFinder);
  await writePyBinding(rsBinding);
  await writeStubs(pyStub);
  await rustFmtSrc(crateInfo.crate);
}
