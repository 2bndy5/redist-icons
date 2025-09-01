from fontawesome_free_pack import get_icon, BRANDS_GITHUB


def test_svg() -> None:
    fetched = get_icon("brands/github")
    assert fetched is not None
    assert BRANDS_GITHUB.svg == fetched.svg
