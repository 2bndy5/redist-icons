const PY_PKG_MAP = {
    "mdi": "mdi",
    "octicons": "oct"
    "fontawesome": "fa",
    "simple-icons": "si",
}

# Generates a json list of crates that have changed.
#
# This script uses the following env variables:
# - REF_FULL: The full git ref that triggered the workflow (e.g. refs/heads/main or refs/tags/v1.0.0)
# - REF_HEAD: The edge/tip commit of the current branch
# - REF_BASE:
#     The base commit of the current branch (for pull_request events)
#     or HEAD~1 (for push events).
export def main [] {
    mut result = []
    let is_for_python = $env.IS_FOR_PYTHON | into bool
    if ($env.REF_FULL | str starts-with "refs/tags/") {
        print $"(ansi green)Tag detected: ($env.REF_FULL)(ansi reset)"
        let crate = (
            $env.REF_FULL
            | parse "refs/tags/{crate}/{version}"
            | get crate
            | first
        )
        if ($crate | is-empty) {
            error make {
                msg: $"(ansi red)Invalid tag format: ($env.REF_FULL); Expected a crate name to prefix the version.(ansi reset)"
            }
        }
        if not ($"crates/($crate)" | path exists) {
            error make {
                msg: $"(ansi red)Crate ($crate) does not exist in the repository.(ansi reset)"
            }
        }
        $result = $result | append ($crate)
    } else {
        let changed_files = (
            (^git diff --name-only $env.REF_BASE $env.REF_HEAD)
            | lines
            | each { $in | str trim }
        )
        let crates = (
            $changed_files
            | where { ($in | str starts-with "crates/") and ($in != 'crates/README.md') }
        )
        for changed_file in $changed_files {
            let crate = (
                $changed_file
                | parse "crates/{crate}/{_}"
                | get crate
                | first
            )
            if ($crate | is-not-empty) and not ($crate in $result) {
                $result = $result | append [$crate]
            } else if ($changed_file == "Cargo.toml") and ($is_for_python) {
                print $"(ansi yellow)Cargo.toml changed at repo root; affects all python bindings.(ansi reset)"
                let all_crates = (
                    ls crates
                    | where { $in.type == "dir" }
                    | get name
                    | each { $in | path basename }
                )
                $result = ($all_crates)
                break
            }
        }
    }
    if ($result | is-not-empty) {
        print "Found changes in the following crates:"
        print $result
        (
            $"Found changes in the following crates:\n($result | to md)\n"
            | save --append $env.GITHUB_STEP_SUMMARY
        )
        if $is_for_python {
            let uv_groups = (
                $result
                | each {|pkg| $PY_PKG_MAP | get $"($pkg)" | $"--group ($in)" }
                | str join " "
            )
            $"uv-groups=($uv_groups)\n" | save --append $env.GITHUB_OUTPUT
            let pytest_paths = (
                $result
                | each { $"crates/($in)/tests/test_($in| str replace '-' '_').py" }
                | str join " "
            )
            $"pytest-paths=($pytest_paths)\n" | save --append $env.GITHUB_OUTPUT
        } else {
            let cargo_packages = (
                $result
                | each {
                    open $"crates/($in)/Cargo.toml"
                    | get package
                    | get name
                    | $"-p ($in)"
                }
                | str join " "
            )
            $"cargo-packages=($cargo_packages)\n" | save --append $env.GITHUB_OUTPUT
        }
    } else {
        let prompt = "Found no changes related to any crates."
        print $prompt
        $prompt | save --append $env.GITHUB_STEP_SUMMARY
    }

    $"crates=($result | to json --raw)" | save --append $env.GITHUB_OUTPUT
}
