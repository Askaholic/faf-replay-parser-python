from fafreplay import body_offset, body_ticks


def test_body_offset(replay):
    assert body_offset(replay) == 4923


def test_body_offset_bytearray(replay):
    assert body_offset(bytearray(replay)) == 4923


def test_body_ticks(replay):
    assert body_ticks(replay[4923:]) == 28917


def test_body_ticks_bytearray(replay):
    assert body_ticks(bytearray(replay[4923:])) == 28917
