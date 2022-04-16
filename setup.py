import re

import toml
from setuptools import setup
from setuptools_rust import Binding, RustExtension

with open("Cargo.toml") as f:
    cargo = toml.load(f)
package = cargo["package"]

setup(
    version=package["version"],
    author=re.match(r"(\w+).*(<.*>)?", package["authors"][0]).group(1),
    description=package["description"],
    rust_extensions=[RustExtension(
        "fafreplay._fafreplay",
        binding=Binding.PyO3
    )],
    zip_safe=False,
)
