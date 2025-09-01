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
use ./common.nu *

# Run an external command and output it's elapsed time.
#
# Not useful if you need to capture the command's output.
export def --wrapped run-cmd [...cmd: string] {
    let app = if (
        ($cmd | first) == 'git'
        or ($cmd | first) == 'gh'
        or ($cmd | first) == 'uv'
    ) {
        ($cmd | first 2) | str join ' '
    } else {
        ($cmd | first)
    }
    print $"(ansi blue)\nRunning(ansi reset) ($cmd | str join ' ')"
    let elapsed = timeit {|| ^($cmd | first) ...($cmd | skip 1)}
    print $"(ansi magenta)($app) took ($elapsed)(ansi reset)"
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

    let deno_task = translate-pkg-to-task $pkg
    if ($deno_task | is-empty) {
        print $"($pkg) is not source of generation"
    } else {
        run-cmd deno task $"gen:($deno_task)"
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
    let sha_hash = $updates | str join | hash sha256 | str substring ..6
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
                | $'\n## Found updates!\n\n($in)\n'
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
        print $"::notice::No changes pushed to ($branch_name)"
    }
}

def main [] {
    let updates = has-updates
    let is_on_main = is-on-main
    if ($updates | is-empty) {
        print $"(ansi green)No updates found(ansi reset)"
        exit 0
    }
    if not $is_on_main {
        print $"(ansi yellow)The current branch is not the default branch; aborting(ansi reset)"
        print "Found possible updates:"
        print $updates
        exit 1
    }
    create-pr $updates
}
