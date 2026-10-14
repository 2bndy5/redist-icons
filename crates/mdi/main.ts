import * as path from "node:path";
import * as fs from "node:fs/promises";
import { loadConfig, optimize } from "svgo";
import {
  copyPkgMeta,
  type GeneratedBinding,
  type GeneratedFinder,
  type GeneratedIcons,
  type GeneratedStubs,
  getPkgDir,
  rustFmtSrc,
  walk,
  writeConstIcons,
  writeFinder,
  writePyBinding,
  writeStubs,
} from "common";
import metadata from "@mdi/svg/meta.json" with { type: "json" };
import npmPkgInfo from "@mdi/svg/package.json" with { type: "json" };

const npmPkg = "@mdi/svg";

type Families = "deprecated" | "all";
const rsLibIcons: Record<Families, GeneratedIcons> = {
  deprecated: { mod: "icons/deprecated", src: [] },
  all: { mod: "icons/all", src: [] },
};

if (import.meta.main) {
  console.log("Generating crate from", npmPkg, "source.");
  const crateInfo = await copyPkgMeta(npmPkg, npmPkgInfo, "LICENSE");
  const pkgDir = getPkgDir(npmPkg);
  const svgFiles = await Array.fromAsync(walk(path.join(pkgDir, "svg")));
  let deprecatedTotal = 0;

  const svgoConfig = await loadConfig("../../svgo.config.mjs");

  const rsLibFinder: GeneratedFinder = {
    mod: "finder",
    src: [],
    sig: '#[cfg(feature = "pyo3")] py: Python, slug: &str',
    invoke: '#[cfg(feature = "pyo3")] py, slug',
  };

  const rsBinding: GeneratedBinding = { lib: crateInfo.lib, src: [] };

  const pyStub: GeneratedStubs = {
    lib: crateInfo.lib,
    src: `from typing import NamedTuple, Final

class Icon(NamedTuple):
    svg: str
    slug: str
    version: str
    deprecated: bool
    author: str

def get_icon(slug: str) -> Icon | None: ...

`,
  };

  for (const entry of svgFiles.entries()) {
    if (entry[1].isDirectory) {
      continue;
    }
    const slug = entry[1].name.substring(0, entry[1].name.length - 4);
    const iconMeta = metadata.find((data) => slug === data.name);
    if (!iconMeta) {
      throw new Error(
        `Name "${slug}" was not found in the ${npmPkg} package metadata JSON`,
      );
    }

    const constName = slug.toUpperCase().replaceAll("-", "_");

    const svgData = await fs.readFile(entry[1].path, "utf-8");
    const optimized = optimize(svgData, svgoConfig);

    const docComment = `\n/// ${slug}`;
    const appendSrc = `\npub const ${constName}: Icon = Icon {
    svg: r##"${optimized.data}"##,
    slug: "${slug}",
    version: "${iconMeta.version}",
    deprecated: ${iconMeta.deprecated},
    author: "${iconMeta.author}",\n};\n`;

    let srcPyi = "";
    let srcBinding = "";
    if (iconMeta.deprecated) {
      deprecatedTotal += 1;
      rsLibIcons.deprecated.src.push(`\n${docComment}\n///
/// DEPRECATED: This icon will be removed in a future version.
#[deprecated]${appendSrc}`);
      srcBinding += "    #[allow(deprecated)]\n";
      rsLibFinder.src.push(`        #[allow(deprecated)]
        "${iconMeta.name}" => {
            #[cfg(feature = "pyo3")]
            {
                PyDeprecationWarning::new_err(
                  "The icon '${iconMeta.name}' is deprecated.",
                )
                .print(py);
            }
            Some(icons::${constName})
        },\n`);
      srcPyi +=
        "# DEPRECATED: This icon will be removed in a future version.\n";
    } else {
      rsLibIcons.all.src.push(docComment + appendSrc);
      rsLibFinder.src.push(
        `        "${iconMeta.name}" => Some(icons::${constName}),\n`,
      );
    }

    srcBinding += `    m.add("${constName}", icons::${constName})?;\n`;
    srcPyi += `${constName}: Final[Icon] = ...\n`;
    rsBinding.src.push(srcBinding);
    pyStub.src += srcPyi;

    if (process.stdin.isTTY) {
      // only write progress indicator if in interactive terminal
      const percent = Math.round(Number((entry[0] / metadata.length) * 100))
        .toString()
        .padStart(3);
      process.stdout.write(`Generating... ${percent}%\r`);
    }
  }

  if (process.stdin.isTTY) console.log();
  console.log("Generated", metadata.length, "icons");
  console.log("Encountered", deprecatedTotal, "deprecated icons");

  await writeConstIcons(Object.values(rsLibIcons));
  await writeFinder(rsLibFinder);
  await writePyBinding(rsBinding);
  await writeStubs(pyStub);
  await rustFmtSrc(crateInfo.crate);
}
