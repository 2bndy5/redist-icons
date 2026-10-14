import { styleText } from "node:util";
import { createRequire } from "node:module";
import * as path from "node:path";
import * as fs from "node:fs/promises";
import { parse, stringify } from "smol-toml";
import which from "which";

const red = (str: string) => styleText("red", str);
const yellow = (str: string) => styleText("yellow", str);

type CrateToml = {
  package: {
    license: string;
    version: string;
    name: string;
    [key: string]: unknown;
  };
  lib: {
    name: string;
    [key: string]: unknown;
  };
  [key: string]: unknown;
};

type NpmPkgJson = {
  license: string;
  version: string;
};

export type CrateInfo = { lib: string; crate: string };

/**
 * Get the filesystem directory for an installed npm package, resolved relative to cwd.
 */
export function getPkgDir(npmPkg: string): string {
  const req = createRequire(path.join(process.cwd(), "package.json"));
  const pkgJsonPath = req.resolve(`${npmPkg}/package.json`);
  return path.dirname(pkgJsonPath);
}

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
  const thisDir = process.cwd();
  const pkgDir = getPkgDir(npmPkg);
  const licensePath = path.join(pkgDir, licenseFile);
  await fs.copyFile(licensePath, path.join(thisDir, "LICENSE"));
  console.log(`Copied ${licenseFile} from ${npmPkg} dist`);

  const cargoPath = path.join(thisDir, "Cargo.toml");
  const crateMeta = await fs.readFile(cargoPath, "utf-8");
  const parsedCrateMeta = parse(crateMeta) as unknown as CrateToml;
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
    console.log("Crate license is already up-to-date with", npmPkg, "info.");
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
    await fs.writeFile(
      cargoPath,
      stringify(parsedCrateMeta).trimStart(),
      "utf-8",
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
  const result = Bun.spawnSync([cargo, "fmt", "-p", crateName]);
  const stderr = result.stderr.toString().trim();
  const stdout = result.stdout.toString().trim();
  if (result.exitCode > 0) {
    throw new Error(`Failed to format ${crateName};\n${stderr}`);
  } else {
    if (stdout) console.log(crateName, red(stdout));
    if (stderr) console.log(crateName, yellow(stderr));
  }
  console.log("Formatted Rust sources");
}
