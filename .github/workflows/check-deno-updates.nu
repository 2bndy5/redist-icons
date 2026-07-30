# This script shall act as an auto-regenerator for
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

export def has-updates [] {
    let deno_out = (^deno outdated --latest -r -q) | lines
    if ($deno_out | is-empty) {
        return null
    }
    let updates = (
        $deno_out
        | skip 3
        | enumerate
        | where {$in.index mod 2 == 0}
        | get 'item'
        # | each {$in | parse "│ {package} │ {current} │ {update} │ {latest} │"}
        # the following regex is equivalent but strips whitespace from captured values
        | each {$in | parse --regex '(?s)[^\w]+(?P<package>[^\s]+)[^\d]+(?P<current>[\d.]+)[^\d]+(?P<update>[\d.]+)[^\d]+(?<latest>[\d.]+)[^$]+'}
        | flatten
    )
    $updates
}

# Regenerates the rust and python sources of the given list of crate names.
def regenerate-source [
    updated_crates: list<string>, # the crate names to regenerate.
] {
    # normalize pkg name
    for pkg in $updated_crates {
        let deno_task = translate-pkg-to-task $pkg
        if ($deno_task | is-empty) {
            print $"($pkg) is not a source of generation"
        } else {
            run-cmd deno task $"gen:($deno_task)"
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

# Apply the given `update`.
#
# Returns a record summarizing the applied changes.
def apply-update [
    update: record<package: string, current: string, update: string, latest: string>, # the dependency update info to apply.
] {
    # normalize pkg name
    let package = $update | get 'package'
    let pkg = (
        if (
            ($package | str starts-with 'npm:')
            or ($package | str starts-with 'jsr:')
        ) {
            $package | str substring 4..
        } else {
            $package
        }
    ) | str trim

    run-cmd deno update -r $"($pkg)@($update | get latest)"

    let crate = match $pkg {
        "@fortawesome/fontawesome-free" => "fontawesome",
        "@mdi/svg" => "mdi",
        "@primer/octicons" => "octicons",
        "simple-icons" => "simple-icons",
        _ => null
    }
    if ($crate | is-not-empty) {
        regenerate-source [$crate]
    }

    # now get repo of updated pkg
    let repo = (
        if ($update | get package | str starts-with 'npm:') {
            let repo = open $"node_modules/($pkg)/package.json" | get "repository"
            if (($repo | describe) == 'string') {
                $repo | url parse | $"https://($in.host)($in.path)"
            } else {
                $repo | get 'url' | url parse | $"https://($in.host)($in.path)"
            }
        } else if ($update | get package | str starts-with 'jsr:') {
            $"https://jsr.io/($pkg)@($update | get latest | str trim)"
        } else {
            null
        }
    )

    # return descriptive info for summary changes
    {
        package: (
            if ($repo | is-not-empty) {
                $"[`($pkg)`]\(($repo))"
            } else {
                $pkg
            }
        ),
        from: $update.current,
        to: $update.latest
    }
}

const PR_NOTES = ".github/workflows/PR-notes.md"

# Create a new branch, applies updates, and opens a Pull Request.
def create-pr [
    updates: table<package: string, current: string, update: string, latest: string>, # The possible updates
] {
    let is_ci = (is-in-ci)
    # create branch
    let sha_hash = $updates | to json --raw | hash sha256 | str substring ..6
    let branch_name = $"deno/updates-($sha_hash)"
    let branch_exists = (^git branch -r) | lines | where {$in | str ends-with $branch_name} | is-not-empty
    run-cmd git checkout -b $branch_name
    if ($is_ci) {
        run-cmd git config --global user.name $"($env.GITHUB_ACTOR)"
        run-cmd git config --global user.email $"($env.GITHUB_ACTOR_ID)+($env.GITHUB_ACTOR)@users.noreply.github.com"
    }
    if ($branch_exists) {
        print $"Branch ($branch_name) already exists"
        run-cmd git pull --rebase origin $branch_name
    }

    # apply updates and aggregate table for PR description
    print "Applying the following updates:"
    print $updates
    mut desc_table = []
    for bump in $updates {
        $desc_table = $desc_table | append (apply-update $bump)
    }

    run-cmd cargo update --workspace

    run-cmd uv run pre-commit run --all-files

    $desc_table | to md | save $PR_NOTES

    # commit changes
    let title = $"build: bump ($updates | length) packages in deno group"
    run-cmd git add --all
    let git_status = (^git status -s) | lines
    if ($git_status | is-not-empty) {
        run-cmd git commit -m $title
        run-cmd git push --set-upstream origin $branch_name --force
        if ($is_ci) {
            (
                $desc_table
                | to md
                | $"\n## Found updates!\n\n($in)\n"
                | save --append $env.GITHUB_STEP_SUMMARY
            )
        }

        # create PR
        let pr_args = [--title $title --body-file $PR_NOTES]
        if $branch_exists {
            let pr_list = (^gh pr list -H $branch_name --json "number") | from json
            if ($pr_list | is-not-empty) {
                let pr_number = $pr_list | first | get number
                run-cmd gh pr edit $pr_number ...$pr_args
            } else {
                run-cmd gh pr create ...$pr_args
            }
        } else {
            run-cmd gh pr create ...$pr_args
        }
    } else {
        print $"(ansi green)No changes pushed to ($branch_name)(ansi reset)"
        if ($is_ci) {
            print $"::notice::No changes pushed to ($branch_name)"
        }
    }
}

def main [] {
    let is_on_main = is-on-main
    let is_ci = (is-in-ci)
    if $is_on_main {
        # only check for (and apply) deno updates on main branch.
        let deno_updates = has-updates
        if ($deno_updates | is-empty) {
            print $"(ansi green)No deno updates found(ansi reset)"
            if $is_ci {
                print "::notice::No deno updates found"
            }
            exit 0
        }
        # includes regenerating sources as needed
        create-pr $deno_updates
    } else {
        # if not on main branch, check for changes that
        # might instigate a rust source regeneration.
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
}
