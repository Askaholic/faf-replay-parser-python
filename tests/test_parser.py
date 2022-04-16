from time import perf_counter
from typing import List

import pytest

from fafreplay import Parser, ReplayReadError, commands


@pytest.fixture
def replays_invalid() -> List[bytes]:
    return [
        bytes(100),  # All zeroes
        b"foo" * 10
    ]


@pytest.fixture
def parser() -> Parser:
    return Parser()


@pytest.fixture
def parser_save() -> Parser:
    return Parser(save_commands=True)


@pytest.fixture
def parser_full() -> Parser:
    return Parser(
        save_commands=True,
        stop_on_desync=False,
    )


def test_constructor_list():
    Parser(
        limit=10,
        commands=[
            commands.Advance,
            commands.VerifyChecksum,
        ],
        save_commands=False,
        stop_on_desync=False
    )


def test_constructor_tuple():
    Parser(
        commands=(
            commands.Advance,
            commands.VerifyChecksum,
        ),
    )


def test_constructor_set():
    Parser(
        commands={
            commands.Advance,
            commands.VerifyChecksum,
        },
    )


def test_constructor_iterable():
    class TestIterable:
        def __init__(self):
            self.data = iter((commands.Advance, commands.VerifyChecksum))

        def __iter__(self):
            return self

        def __next__(self):
            return next(self.data)

    Parser(
        commands=TestIterable(),
    )


def test_constructor_none():
    Parser(
        limit=None,
        commands=None,
        save_commands=None,
        stop_on_desync=None
    )


def test_constructor_error_limit():
    with pytest.raises(TypeError):
        Parser(limit="something")
    with pytest.raises(OverflowError):
        Parser(limit=-10)


def test_constructor_error_commands():
    with pytest.raises(TypeError):
        Parser(commands=10)
    with pytest.raises(TypeError):
        Parser(commands=object())


def test_constructor_error_save_commands():
    with pytest.raises(TypeError):
        Parser(save_commands=10.1)
    with pytest.raises(TypeError):
        Parser(save_commands="something")


def test_constructor_error_stop_on_desync():
    with pytest.raises(TypeError):
        Parser(stop_on_desync=(90, 100))
    with pytest.raises(TypeError):
        Parser(stop_on_desync="something")


def test_parse(parser: Parser, replay: bytes):
    result = parser.parse(replay)

    assert result["header"]["scfa_version"] == "Supreme Commander v1.50.3698"
    assert result["header"]["replay_version"] == "Replay v1.9"
    assert result["header"]["mods"] == {}
    assert result["header"]["cheats_enabled"] is False
    assert result["header"]["army_count"] == 10
    assert result["header"]["seed"] == 22557011
    assert result["body"]["sim"]["tick"] == 28917
    assert result["body"]["sim"]["desync_tick"] is None


def test_parse_full(parser_save: Parser, replay: bytes):
    result = parser_save.parse(replay)
    assert len(result["body"]["commands"]) == 260421


def test_parse_header(parser: Parser, replay: bytes):
    result = parser.parse_header(replay)

    assert result["scfa_version"] == "Supreme Commander v1.50.3698"
    assert result["replay_version"] == "Replay v1.9"


def test_parse_desynced(parser: Parser, replay_desynced: bytes):
    with pytest.raises(ReplayReadError):
        parser.parse(replay_desynced)


def test_parse_full_desynced(parser_full: Parser, replay_desynced: bytes):
    result = parser_full.parse(replay_desynced)

    assert result["body"]["sim"]["desync_tick"] == 9105
    assert len(result["body"]["sim"]["desync_ticks"]) == 268


def test_parse_corrupt(parser: Parser, replays_invalid: List[bytes]):
    for data in replays_invalid:
        with pytest.raises(ReplayReadError):
            parser.parse_body(data)


def test_parse_corrupt_body(parser: Parser, replays_invalid: List[bytes]):
    for data in replays_invalid:
        with pytest.raises(ReplayReadError):
            parser.parse_body(data)


@pytest.mark.release
def test_parse_performance(parser: Parser, replay: bytes):
    start = perf_counter()

    parser.parse(replay)

    end = perf_counter()
    elapsed = end - start

    assert elapsed < 0.12, "Make sure to compile in release mode"
