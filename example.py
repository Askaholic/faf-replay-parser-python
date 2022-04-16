"""
An example of how to use the replay parser from python. Requires Python 3.5+.
"""


from datetime import timedelta
from time import perf_counter

from fafreplay import Parser, commands


def main():
    # Create a parser object. This configuration would be optimal for parsing
    # only the game time
    parser = Parser(
        # Skip all commands except the ones defined here
        commands=[
            commands.Advance,
        ],
        # Throw away commands right after we parse them. We will not be able to
        # inspect them later. This is the default
        save_commands=False
    )

    # Read replay to a `bytes` object
    with open("tests/data/8653680.scfareplay", "rb") as f:
        data = f.read()

    # Parse replay
    start = perf_counter()
    replay = parser.parse(data)
    end = perf_counter()

    print(replay)
    print(f"Parsing took {end-start}s")
    print("Game time:", timedelta(milliseconds=replay["body"]["sim"]["tick"]*100))


if __name__ == '__main__':
    main()
