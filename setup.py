import re

import toml
from setuptools import setup
from setuptools_rust import Binding, RustExtension

with open("Cargo.toml") as f:
    cargo = toml.load(f)
package = cargo["package"]

setup(
    name="faf-replay-parser",
    version=package["version"],
    author=re.match(r"(\w+).*(<.*>)?", package["authors"][0]).group(1),
    description=package["description"],
    long_description=open("README.md").read(),
    long_description_content_type='text/markdown',
    license="License :: OSI Approved :: GNU Lesser General Public License v3 (LGPLv3)",
    url="https://github.com/Askaholic/faf-replay-parser-python",
    rust_extensions=[RustExtension(
        cargo["lib"]["name"],
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
