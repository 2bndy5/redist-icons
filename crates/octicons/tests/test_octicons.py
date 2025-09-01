from octicons_pack import get_icon, GIT_BRANCH_24


def test_svg() -> None:
    fetched = get_icon("git-branch-24")
    assert fetched is not None
    assert GIT_BRANCH_24.svg == fetched.svg
