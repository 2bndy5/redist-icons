# Some common functions shared by nu scripts that run in CI

const DEFAULT_BRANCH = "main"

# Is the the default branch currently checked out?
export def is-on-main [] {
    let branch = (
        ^git branch
        | lines
        | where {$in | str starts-with '*'}
        | first
        | str trim --left --char '*'
        | str trim
    ) == $DEFAULT_BRANCH
    $branch
}

# Is this executed in a CI run?
#
# Uses env var CI to determine the resulting boolean
export def is-in-ci [] {
    $env | get --optional CI | default 'false' | (($in == 'true') or ($in == true))
}

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
