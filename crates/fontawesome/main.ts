import { loadConfig, optimize } from "npm:svgo";
import {
  copyPkgMeta,
  GeneratedBinding,
  GeneratedFinder,
  GeneratedIcons,
  GeneratedStubs,
  rustFmtSrc,
  writeConstIcons,
  writeFinder,
  writePyBinding,
  writeStubs,
} from "common";
import metadata from "@fortawesome/fontawesome-free/metadata/icon-families.json" with {
  type: "json",
};
import npmPkgInfo from "@fortawesome/fontawesome-free/package.json" with {
  type: "json",
};

const npmPkg = "@fortawesome/fontawesome-free";

type IconData = {
  raw: string;
  lastModified: number;
  width: number;
  height: number;
};

type faFamilies = "brands" | "regular" | "solid";

const rsIcons: Record<faFamilies, GeneratedIcons> = {
  brands: { mod: "icons/brands", src: [] },
  regular: { mod: "icons/regular", src: [] },
  solid: { mod: "icons/solid", src: [] },
};
const rsLibFinder: GeneratedFinder = {
  mod: "finder",
  src: [],
};

// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  console.log("Generating crate from", npmPkg, "source.");
  const crateInfo = await copyPkgMeta(npmPkg, npmPkgInfo, "LICENSE.txt");

  // get total icons
  let total = 0;
  for (const entry of Object.entries(metadata)) {
    total += entry[1].familyStylesByLicense.free.length;
  }
  let progress = 0;

  const svgoConfig = await loadConfig("../../svgo.config.mjs");

  const textEncoder = new TextEncoder();

  const rsBinding: GeneratedBinding = {
    lib: crateInfo.lib,
    src: [],
  };

  const pyStub: GeneratedStubs = {
    lib: crateInfo.lib,
    src: `from typing import NamedTuple, Final

class Icon(NamedTuple):
    svg: str
    slug: str
    last_modified: int
    family: str
    width: int
    height: int
    label: str

def get_icon(slug: str) -> Icon | None: ...

`,
  };

  for (const entry of Object.entries(metadata)) {
    const slug = entry[0];

    for (const svgs of Object.values(entry[1].svgs)) {
      for (const style of Object.entries(svgs)) {
        progress += 1;

        if (Deno.stdin.isTerminal()) {
          // only write progress indicator if in interactive terminal
          const percent = Math.round(Number(progress / total * 100))
            .toString().padStart(3);
          await Deno.stdout.write(
            textEncoder.encode(`Generating... ${percent}%\r`),
          );
        }

        const data = <IconData> style[1];
        const svgData = data.raw;
        const family = <string> style[0];
        const optimized = optimize(svgData, svgoConfig);

        const constName = (`${family}-${slug}`).toUpperCase().replaceAll(
          "-",
          "_",
        );

        rsLibFinder.src.push(`        #[cfg(feature = "${family}")]
        "${family}/${slug}" => Some(icons::${constName}),\n`);
        rsBinding.src.push(`    #[cfg(feature = "${family}")]
    m.add("${constName}", icons::${constName})?;\n`);
        pyStub.src += `${constName}: Final[Icon] = ...\n`;

        const key = <faFamilies> family;
        rsIcons[key].src.push(`
/// ${family}/${slug}
pub const ${constName}: Icon = Icon {
    svg: r#"${optimized.data}"#,
    slug: "${slug}",
    last_modified: ${data.lastModified},
    family: "${family}",
    width: ${data.width},
    height: ${data.height},
    label: "${entry[1].label}",
};\n`);
      }
    }
  }

  if (Deno.stdin.isTerminal()) console.log();
  for (const entry of Object.entries(rsIcons)) {
    console.log("Encountered", entry[1].src.length, entry[0], "icons");
  }
  console.log("Generated", total, "icons");

  await writeConstIcons(Object.values(rsIcons));
  await writePyBinding(rsBinding);
  await writeFinder(rsLibFinder);
  await writeStubs(pyStub);
  await rustFmtSrc(crateInfo.crate);
}
