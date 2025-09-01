import * as path from "@std/path";
import { generatedCommentMarker, maxItems } from "common";

export type GeneratedIcons = {
  mod: string;
  src: Array<string>;
};

const useIcon = "use crate::Icon;\n";

type PartitionedSource = {
  modPath: string;
  src: Array<string>;
  header: string;
};

/**
 * This function takes a list of generated sources and writes them to appropriate files.
 * For optimization reasons, long Rust sources will be partitioned to no more than 200 items each.
 *
 * @remarks This function uses paths relative to the current working directory.
 *
 * @param rsSrc - The list of generated Rust sources.
 */
export async function writeConstIcons(rsSrc: Array<GeneratedIcons>) {
  const mods: Array<PartitionedSource> = [];
  const thisDir = Deno.cwd();
  for (const rs of rsSrc) {
    if (rs.src.length > maxItems) {
      const partitions = Math.ceil(rs.src.length / maxItems);
      const modRelPath = rs.mod.split("/");
      const rootMod: PartitionedSource = {
        modPath: path.join(thisDir, "src", ...modRelPath, "mod.rs"),
        src: [],
        header: "",
      };
      for (let i = 0; i < partitions; i++) {
        const mod = `part_${i.toString().padStart(2, "0")}`;
        rootMod.header += `mod ${mod};\npub use ${mod}::*;\n`;

        const modDir = path.join(thisDir, "src", ...modRelPath);
        await Deno.remove(modDir, { recursive: true });
        await Deno.mkdir(modDir, { recursive: true });

        const fileName = path.join(modDir, `${mod}.rs`);
        mods.push({
          modPath: fileName,
          src: rs.src.slice(
            i * maxItems,
            Math.min(rs.src.length, i * maxItems + maxItems),
          ),
          header: useIcon,
        });
      }
      mods.push(rootMod);
    } else {
      rs.mod = path.join(thisDir, "src", `${rs.mod}.rs`);
      mods.push({ modPath: rs.mod, src: rs.src, header: useIcon });
    }

    for (const m of mods) {
      await Deno.writeTextFile(
        m.modPath,
        generatedCommentMarker + m.header + m.src.join(""),
      );
    }
  }
  console.debug("Generated Icon constants via", mods.length, "modules");
}
