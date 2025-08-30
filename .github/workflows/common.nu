# Some common functions shared by nu scripts that run in CI

# Is the the default branch currently checked out?
export def is-on-main [] {
    let default_branch = (
        (^git rev-parse --abbrev-ref origin/HEAD)
        | str replace "origin/" ""
        | str trim
    )
    let branch = (
        ^git branch
        | lines
        | where {$in | str starts-with '*'}
        | first
        | str trim --left --char '*'
        | str trim
    ) == $default_branch
    $branch
}
