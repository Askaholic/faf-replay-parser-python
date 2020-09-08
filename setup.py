import re

from setuptools import setup

import toml
from setuptools_rust import Binding, RustExtension

with open("Cargo.toml") as f:
    cargo = toml.load(f)
package = cargo["package"]

setup(
    name=package["name"],
    version=package["version"],
    author=re.match(r"(\w+).*(<.*>)?", package["authors"][0]).group(1),
    description=package["description"],
    long_description=open("src/python/README.md").read(),
    long_description_content_type='text/markdown',
    url="https://gitlab.com/Askaholic/faf-replay-parser",
    rust_extensions=[RustExtension(
        cargo["lib"]["name"],
        features=["python"],
        binding=Binding.PyO3
    )],
    classifiers=[
        "Programming Language :: Python :: 3",
        "Operating System :: POSIX :: Linux",
        "Development Status :: 2 - Pre-Alpha",
        "License :: OSI Approved :: GNU Lesser General Public License v3 (LGPLv3)"
    ],
    zip_safe=False,
)
