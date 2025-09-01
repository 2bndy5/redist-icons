from simple_icons_pack import get_icon, SI_GITHUB


def test_svg() -> None:
    fetched = get_icon("github")
    assert fetched is not None
    assert SI_GITHUB.svg == fetched.svg
