# This script shall act as a auto-regenerator for
# dependabot updates to the deno ecosystem.
#
# If any changes to the workspace's crates are detected,
# then this script will
#
# 1. run the appropriate `deno task` to regenerate the rust sources (& python type stubs)
# 2. push a commit with the updated sources back to the remote.
#
# This script requires the following tools installed:
#
# - git
# - deno (https://deno.com/)
# - uv (https://docs.astral.sh/uv)
# - cargo (https://rustup.rs/)
use ./common.nu *
use ../dyn-ci-matrix/generate_matrix.nu list-changed-files

# Translate a given `crate` name into a deno task
def translate-pkg-to-task [
    crate: string, # The workspace crate path to translate
] {
    match $crate {
        "fontawesome" => "fa",
        "mdi" => "mdi",
        "octicons" => "oct",
        "simple-icons" => "si",
        _ => null
    }
}

# Regenerates the rust and python sources of the given list of crate names.
def apply-update [
    updated_crates: list<string>, # the crate names to regenerate.
] {
    # normalize pkg name
    for pkg in $updated_crates {
        let deno_task = translate-pkg-to-task $pkg
        if ($deno_task | is-empty) {
            print $"($pkg) is not source of generation"
        } else {
            run-cmd deno task $"gen:($deno_task)"
        }
    }
}

# Create a new commit, and pushes it to the remote.
def push-updates [] {
    run-cmd git add --all
    let has_changes = (
        ^git status -s
        | lines
        | each { $in | str trim }
        | is-not-empty
    )
    if not $has_changes {
        print $"(ansi green)No git updates to push!(ansi reset)"
        return
    }
    let is_ci = (is-in-ci)
    if ($is_ci) {
        run-cmd git config --global user.name $"($env.GITHUB_ACTOR)"
        run-cmd git config --global user.email $"($env.GITHUB_ACTOR_ID)+($env.GITHUB_ACTOR)@users.noreply.github.com"
    }

    run-cmd uv run pre-commit run --all-files

    # commit changes
    run-cmd git commit -m "build: regenerate rust sources"
    run-cmd git push
}

def main [] {
    let updated_crates = list-changed-files true true
    if ($updated_crates | is-empty) {
        print $"(ansi green)No updates found(ansi reset)"
        exit 0
    }
    apply-update $updated_crates
    push-updates
}
