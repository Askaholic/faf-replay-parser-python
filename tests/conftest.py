import pytest


def pytest_configure(config):
    config.addinivalue_line(
        "markers", "release: These tests only make sense in release mode."
    )


############
# Fixtures #
############


@pytest.fixture
def replay() -> bytes:
    with open("tests/data/8653680.scfareplay", "rb") as f:
        return f.read()


@pytest.fixture
def fafreplay() -> bytes:
    with open("tests/data/8653680.fafreplay", "rb") as f:
        return f.read()


@pytest.fixture
def replay_desynced() -> bytes:
    with open("tests/data/8748707-desynced.scfareplay", "rb") as f:
        return f.read()
