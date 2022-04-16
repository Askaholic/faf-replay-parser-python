from fafreplay._fafreplay import *

import base64
import json
import zlib
try:
    import zstd
except ImportError:
    zstd = None


def extract_scfa(fobj):
    """extract_scfa(fobj: io.BytesIO) -> bytes

    Turns data from `.fafreplay` format into `.scfareplay` format. The zstd
    library needs to be installed in order to decode version 2 of the
    `.fafreplay` format.
    """
    header = json.loads(fobj.readline().decode())
    buf = fobj.read()
    version = header.get("version", 1)

    if version == 1:
        decoded = base64.decodebytes(buf)
        decoded = decoded[4:]  # skip the decoded size
        return zlib.decompress(decoded)
    elif version == 2:
        if zstd is None:
            raise RuntimeError(
                "zstd is required for decompressing this replay"
            )
        return zstd.decompress(buf)
