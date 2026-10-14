import * as path from "node:path";
import * as fs from "node:fs/promises";

export {
  type GeneratedBinding,
  type GeneratedStubs,
  writePyBinding,
  writeStubs,
} from "./writeBinding.ts";
export { type GeneratedFinder, writeFinder } from "./writeFinder.ts";
export { type GeneratedIcons, writeConstIcons } from "./writeIcons.ts";
export { copyPkgMeta, getPkgDir, rustFmtSrc } from "./houseKeeping.ts";
export const maxItems = 200;
export const generatedCommentMarker =
  "// This file was generated. DO NOT EDIT.\n";

export async function* walk(
  dir: string,
): AsyncIterable<{ path: string; name: string; isDirectory: boolean }> {
  const entries = await fs.readdir(dir, {
    recursive: true,
    withFileTypes: true,
  });
  for (const entry of entries) {
    yield {
      path: path.join(entry.parentPath, entry.name),
      name: entry.name,
      isDirectory: entry.isDirectory(),
    };
  }
}
