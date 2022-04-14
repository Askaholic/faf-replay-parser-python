import fafreplay


def test_available_objects():
    assert fafreplay.Parser
    assert fafreplay.ReplayReadError
    assert fafreplay.ReplayDesyncedError
    assert fafreplay.body_offset
    assert fafreplay.body_ticks
    assert fafreplay.extract_scfa
