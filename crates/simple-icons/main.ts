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
import metadata from "si-data" with { type: "json" };
import npmPkgInfo from "si-pkg-meta" with { type: "json" };

const npmPkg = "simple-icons";

// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  console.log("Generating crate from", npmPkg, "source.");
  const crateInfo = await copyPkgMeta(npmPkg, npmPkgInfo, "LICENSE.md");
  const svgFiles = await Array.fromAsync(
    walk(`../../node_modules/${npmPkg}/icons`),
  );

  const svgoConfig = await loadConfig("../../svgo.config.mjs");

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
    title: str
    hex: str
    source: str
    guidelines: str | None
    license: str | None

def get_icon(slug: str) -> Icon | None: ...

`,
  };

  for (const entry of svgFiles.entries()) {
    if (entry[1].isDirectory) {
      continue;
    }
    const slug = entry[1].name.substring(0, entry[1].name.length - 4);
    const iconMeta = metadata.find((data) => slug === data.slug);
    if (!iconMeta) {
      throw new Error(
        `Name "${slug}" was not found in the ${npmPkg} package metadata JSON`,
      );
    }

    const constName = "SI_" + slug.toUpperCase().replaceAll("-", "_");

    const svgData = await Deno.readTextFile(`${entry[1].path}`);
    const optimized = optimize(svgData, svgoConfig);

    const iconGuidelines = iconMeta.guidelines
      ? `Some("${iconMeta.guidelines}")`
      : "None";
    let iconLicense = "None";
    if (iconMeta.license) {
      if ("url" in iconMeta.license) {
        iconLicense = `Some("${iconMeta.license.url!}")`;
      } else if ("type" in iconMeta.license) {
        iconLicense = `Some("${iconMeta.license.type}")`;
      }
    }
    const appendSrc = `\n/// ${slug}
pub const ${constName}: Icon = Icon {
    svg: r#"${optimized.data}"#,
    slug: "${slug}",
    title: "${iconMeta.title}",
    hex: "${iconMeta.hex}",
    source: "${iconMeta.source}",
    guidelines: ${iconGuidelines},
    license: ${iconLicense},\n};\n`;

    rsLibIcons.src.push(appendSrc);
    rsLibFinder.src.push(
      `        "${iconMeta.slug}" => Some(icons::${constName}),\n`,
    );
    rsBinding.src.push(`    m.add("${constName}", icons::${constName})?;\n`);
    pyStub.src += `${constName}: Final[Icon] = ...\n`;

    if (Deno.stdin.isTerminal()) {
      // only write progress indicator if in interactive terminal
      const percent = Math.round(Number(entry[0] / metadata.length * 100))
        .toString().padStart(3);
      await Deno.stdout.write(
        textEncoder.encode(`Generating... ${percent}%\r`),
      );
    }
  }

  console.log("\nGenerated", metadata.length, "icons");

  await writeConstIcons([rsLibIcons]);
  await writeFinder(rsLibFinder);
  await writePyBinding(rsBinding);
  await writeStubs(pyStub);
  await rustFmtSrc(crateInfo.crate);
}
