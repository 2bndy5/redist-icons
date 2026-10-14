# This script detects changes in workspace crates (e.g. from Dependabot PRs)
# and regenerates the rust sources (& python type stubs) accordingly.
#
# If any changes to the workspace's crates are detected,
# then this script will
#
# 1. run the appropriate `bun run gen:<task>` to regenerate the rust sources (& python type stubs)
# 2. push a commit with the updated sources back to the remote.
#
# This script requires the following tools installed:
#
# - git
# - bun (https://bun.sh/)
# - uv (https://docs.astral.sh/uv)
# - cargo (https://rustup.rs/)
use ./common.nu *
use ../dyn-ci-matrix/generate_matrix.nu list-changed-files

# Translate a given `crate` name into a generator task
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
def regenerate-source [
    updated_crates: list<string>, # the crate names to regenerate.
] {
    for pkg in $updated_crates {
        let task = translate-pkg-to-task $pkg
        if ($task | is-empty) {
            print $"($pkg) is not a source of generation"
        } else {
            run-cmd bun run $"gen:($task)"
        }
    }
}

# Create a new commit, and pushes it to the remote.
def push-regenerated-sources [] {
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
    let is_ci = (is-in-ci)
    # Check for changes that might instigate a rust source regeneration.
    let updated_crates = list-changed-files true true
    if ($updated_crates | is-empty) {
        print $"(ansi green)No crates need to be regenerated(ansi reset)"
        if $is_ci {
            print "::notice::No crates need to be regenerated"
        }
        exit 0
    }
    regenerate-source $updated_crates
    push-regenerated-sources
}

