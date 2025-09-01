from material_design_icons_pack import get_icon, ALERT


def test_svg() -> None:
    fetched = get_icon("alert")
    assert fetched is not None
    assert ALERT.svg == fetched.svg
