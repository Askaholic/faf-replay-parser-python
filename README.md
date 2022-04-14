# FAF Replay Parser
![Build Status](https://github.com/Askaholic/faf-replay-parser-python/actions/workflows/test.yml/badge.svg?branch=main)
[![pypi](https://badge.fury.io/py/faf-replay-parser.svg)](https://badge.fury.io/py/faf-replay-parser)

A fast library for parsing Supreme Commander Forged Alliance replay files.

## Installation
Currently, pre-built packages are only available for Linux. You can install them
with:

```
pip install faf-replay-parser
```

## Documentation
Here are some examples of using the parser. Check `help(fafreplay)` for more
details on available functions.

### Gathering basic info
For the most basic uses there are a few special functions that should be
preferred over constructing a `Parser` object. These do only one job, but they
do it very quickly.

Current functions:
  - `body_offset` - Used for splitting replay data into header and body.
  - `body_ticks` - Used for extracting the game length.

#### Example
```python
from datetime import timedelta
from fafreplay import body_offset, body_ticks

# Split replay data into header and body
offset = body_offset(data)
header_data, body_data = data[:offset], data[offset:]

# Get replay length in ticks
ticks = body_ticks(body_data)
print("Game length:", timedelta(milliseconds=ticks*100))
```

### Using the Parser object
The `Parser` object can be used to get fine grained control over how replay
commands are parsed. Generally it's a good idea to parse only the minimum
commands needed, as conversion back to python dictionaries is quite expensive.

```python
from datetime import timedelta
from fafreplay import Parser, commands

parser = Parser(
    # Skip all commands except the ones defined here
    commands=[
        commands.Advance,                   # For the tick counter
        commands.VerifyChecksum,            # For desync detection
    ],
    # Throw away commands right after we parse them. Setting this to `True` will
    # significantly increase the parse time.
    save_commands=False,
    limit=None,
    stop_on_desync=False
)
# Or create a parser with default arguments (turn off save_commands though)
# parser = Parser(save_commands=False)

# Read replay to a `bytes` object
with open("12345.scfareplay", "rb") as f:
    data = f.read()

# Parse to a python dictionary. Data must of type `bytes` or `bytearray`
replay = parser.parse(data)
print("Game time:", timedelta(milliseconds=replay["body"]["sim"]["tick"]*100))
if replay["body"]["sim"]["desync_ticks"]:
    print("Replay desynced!")
```

### Benchmark comparison
To see how much faster the basic functions can be, consider this simple example
done on replay `6176549` (an almost 50 minute long Seton's game).

```python
>>> len(body_data)
5586339
>>> body_ticks(body_data)
28917
>>> parser = Parser(
...     commands=[commands.Advance],
...     save_commands=False
... )
>>> timeit.timeit("parser.parse_body(body_data)['sim']['tick']", globals=globals(), number=100)
1.4510237049980788
>>> timeit.timeit("body_ticks(body_data)", globals=globals(), number=100)
0.20173147800232982
```

In this case `body_ticks` turned out to be more than 7x faster than using a
`Parser`.
