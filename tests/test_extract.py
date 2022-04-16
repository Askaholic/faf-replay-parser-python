import io

from fafreplay import extract_scfa


def test_extract_v2(fafreplay, replay):
    data = extract_scfa(io.BytesIO(fafreplay))
    # The data from the FAF replay server seems to be missing the
    # SetCommandSource and EndGame commands at the end of the file
    assert data == replay[:-7]
