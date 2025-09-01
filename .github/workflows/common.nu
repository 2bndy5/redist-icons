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
