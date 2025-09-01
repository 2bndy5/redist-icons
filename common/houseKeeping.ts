import { red, yellow } from "@std/fmt/colors";
import { parse, stringify } from "@std/toml";
import * as path from "@std/path";
import which from "which";

type CrateToml = {
  package: {
    license: string;
    version: string;
    name: string;
  };
  lib: {
    name: string;
  };
};

type NpmPkgJson = {
  license: string;
  version: string;
};

export type CrateInfo = { lib: string; crate: string };

/**
 * Copy the license and version info
 * from an npm package's distribution into the crate folder.
 *
 * @param npmPkg - The name of the npm package.
 * @param npmPkgInfo - The package.json object parsed from the npm package.
 * @param licenseFile - The name of the LICENSE file (from the npm package) to
 * copy into the crate.
 *
 * @returns string - The name of the rust library.
 */
export async function copyPkgMeta(
  npmPkg: string,
  npmPkgInfo: NpmPkgJson,
  licenseFile: string,
): Promise<CrateInfo> {
  const thisDir = Deno.cwd();
  const licensePath = path.join(
    thisDir,
    "..",
    "..",
    "node_modules",
    npmPkg,
    licenseFile,
  );
  await Deno.copyFile(licensePath, path.join(thisDir, "LICENSE"));
  console.log(`Copied ${licenseFile} from ${npmPkg} dist`);

  const cargoPath = path.join(thisDir, "Cargo.toml");
  const crateMeta = await Deno.readTextFile(cargoPath);
  const parsedCrateMeta = <CrateToml> parse(crateMeta);
  const libName = parsedCrateMeta.lib.name;
  const oldVersion = parsedCrateMeta.package.version;
  const oldLicense = parsedCrateMeta.package.license;
  let modCrateMeta = false;
  if (oldLicense != npmPkgInfo.license) {
    parsedCrateMeta.package.license = npmPkgInfo.license;
    modCrateMeta = true;
    console.log(
      "Changed crate license to",
      npmPkgInfo.license,
      "from",
      oldLicense,
    );
  } else {
    console.log(
      "Crate license is already up-to-date with",
      npmPkg,
      "info.",
    );
  }
  if (oldVersion != npmPkgInfo.version) {
    parsedCrateMeta.package.version = npmPkgInfo.version;
    modCrateMeta = true;
    console.log(
      "Changed crate version to",
      npmPkgInfo.version,
      "from",
      oldVersion,
    );
  } else {
    console.log(
      "Crate version",
      oldVersion,
      "is already up-to-date with",
      npmPkg,
      "info.",
    );
  }
  if (modCrateMeta) {
    await Deno.writeTextFile(
      cargoPath,
      stringify(parsedCrateMeta).trimStart(),
    );
  }
  return { lib: libName, crate: parsedCrateMeta.package.name };
}

/**
 * Run `rustfmt` on the given `crateName`.
 *
 * @param crateName - The `package.name` as described in the Cargo.toml.
 */
export async function rustFmtSrc(crateName: string) {
  const cargo = await which("cargo");
  const cmd = new Deno.Command(cargo, { args: ["fmt", "-p", crateName] });
  const result = await cmd.output();
  const decoder = new TextDecoder("utf-8");
  const stderr = decoder.decode(result.stderr).trim();
  const stdout = decoder.decode(result.stdout).trim();
  if (result.code > 0) {
    throw new Error(`Failed to format ${crateName};\n${stderr}`);
  } else {
    if (stdout) console.log(crateName, red(stdout));
    if (stderr) console.log(crateName, yellow(stderr));
  }
  console.log("Formatted Rust sources");
}
