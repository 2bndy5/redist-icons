# This script shall act as a auto-updater since
# dependabot lacks support for deno.
#
# If deno-specific updates are detected, then
# this script will
#
# 1. create a branch and apply updates to that branch
# 2. create a Pull Request for review
#
# This script requires the following tools installed:
#
# - git
# - deno
# - gh-cli (https://cli.github.com)
# - uv (https://docs.astral.sh/uv)
# - cargo (https://rustup.rs/)
use ./common.nu *
use ../dyn-ci-matrix/generate_matrix.nu list-changed-files

# Translate a given `pkg` name into a deno task
def translate-pkg-to-task [
    pkg: string, # The npm package name to translate
] {
    match $pkg {
        "@fortawesome/fontawesome-free" => "fa",
        "@mdi/svg" => "mdi",
        "@primer/octicons" => "oct",
        "simple-icons" => "si",
        _ => null
    }
}

# Apply the given `update`.
#
# Returns a record summarizing the applied changes.
def apply-update [
    updated_crates: list<string>, # the dependency update info to apply.
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

# Create a new branch, applies updates, and opens a Pull Request.
def push-updates [] {
    run-cmd git add .
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
    let updated_crates = list-changed-files true
    if ($updated_crates | is-empty) {
        print $"(ansi green)No updates found(ansi reset)"
        exit 0
    }
    apply-update $updated_crates
    push-updates
}
