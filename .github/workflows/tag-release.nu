# This script automates the release process for all of the packages in this repository.
# This script creates a GitHub Release using a new tag with
# auto-generated notes for a description.
#
# Requires `gh-cli` (see https://cli.github.com) to be installed to
# create the release and push the tag.
#
# When a tag is pushed to the remote, the CI builds are triggered and
# packages are published to crates.io and pypi.org
#
# NOTE: In a CI run, the GITHUB_TOKEN env var to authenticate access.
# Locally, you can use `gh login` to interactively authenticate the user account.
#
# The GITHUB_TOKEN permissions shall include:
# - write (and inherently read) access to the repository "Contents"
#   for publishing a GitHub release.
use ./common.nu *

# Run an external command and output it's elapsed time.
#
# Not useful if you need to capture the command's output.
export def --wrapped run-cmd [...cmd: string] {
    let app = if (($cmd | first) == 'gh') {
        ($cmd | first 2) | str join ' '
    } else {
        ($cmd | first)
    }
    print $"(ansi blue)\nRunning(ansi reset) ($cmd | str join ' ')"
    let elapsed = timeit {|| ^($cmd | first) ...($cmd | skip 1)}
    print $"(ansi magenta)($app) took ($elapsed)(ansi reset)"
}

# The main function of this script.
#
# The `pkg` parameter is a required CLI option:
#     nu .github/workflows/tag-release.nu simple-icons
#
# The acceptable `pkg` values are defined as the path to the crates/<pkg>/Cargo.toml manifest:
# - mdi
# - fontawesome
# - octicons
# - simple-icons
def main [
    pkg: string, # The crate's path name to the Cargo.toml manifests
] {
    let ver = (
        open $"crates/($pkg)/Cargo.toml"
        | get package
        | get version
    )
    let tag = $"($pkg)/v($ver)"
    if not (is-on-main) {
        print $"(ansi yellow)Not checked out on default branch!(ansi reset)"
        exit 1
    }
    if not (is-in-ci) {
        print $"(ansi yellow)Not deploying from local clone.(ansi reset)"
        exit 1
    }
    print $"Deploying ($tag)"
    run-cmd gh release create $tag '--title' $"($pkg) v($ver)" --generate-notes
}
