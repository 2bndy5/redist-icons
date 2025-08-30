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

const PR_NOTES = ".github/workflows/PR-notes.md"

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

# Create a new branch, applies updates, and opens a Pull Request.
def create-pr [
    updates: table<package: string, current: string, update: string, latest: string>, # The possible updates
] {
    # create branch
    let sha_hash = $updates | str join | hash sha256 | str substring ..7
    let branch_name = $"deno/updates-($sha_hash)"
    run-cmd git checkout -b $branch_name

    # apply updates and aggregate table for PR description
    mut desc_table = []
    for bump in $updates {
        # normalize pkg name
        let package = $bump | get 'package'
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

        run-cmd deno update -r $"($pkg)@($bump | get latest)"

        let deno_task = translate-pkg-to-task $pkg
        if not $deno_task {
            print $"($pkg) is not source of generation"
        } else {
            run-cmd deno task $"gen:($deno_task)"
        }

        # now get repo of updated pkg
        let repo = (
            if ($bump | get package | str starts-with 'npm:') {
                let repo = open $"node_modules/($pkg)/package.json" | get "repository"
                if (($repo | describe) == 'string') {
                    $repo | url parse | $"https://($in.host)($in.path)"
                } else {
                    $repo | get 'url' | url parse | $"https://($in.host)($in.path)"
                }
            } else if ($bump | get package | str starts-with 'jsr:') {
                $"https://jsr.io/($pkg)@($bump | get latest | str trim)"
            } else {
                null
            }
        )
        $desc_table = $desc_table | append {
            package: (
                if $repo {
                    $"[`($pkg)`]\(($repo))"
                } else {
                    $pkg
                }
            ),
            from: $bump.current,
            to: $bump.latest
        }
    }

    run-cmd uv run pre-commit run --all-files

    # commit changes
    let title = $"build: bump ($updates | length) packages in deno group"
    run-cmd git add .
    run-cmd git commit -m $title
    run-cmd git push --set-upstream origin $branch_name

    # create PR
    $desc_table | to md | save $PR_NOTES
    run-cmd gh pr create --title $title --body-file $PR_NOTES
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
