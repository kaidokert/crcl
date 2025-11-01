#!/usr/bin/env python3
"""
Setup script for CRCL Python package

This package provides Python dataclasses for CRCL (Canonical Robot Command Language)
generated from XSD schemas using xsdata.
"""

from setuptools import setup, find_packages
from pathlib import Path

# Read the README if it exists
this_directory = Path(__file__).parent
readme_path = this_directory.parent / "README.md"
if readme_path.exists():
    long_description = readme_path.read_text()
else:
    long_description = """
    # CRCL Python

    Python dataclasses for CRCL (Canonical Robot Command Language) messages.
    Generated from official CRCL XSD schemas.
    """

setup(
    name="crcl-python",
    version="0.1.0",
    author="Your Name",
    author_email="your.email@example.com",
    description="Python dataclasses for CRCL (Canonical Robot Command Language)",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/kaidokert/crcl",
    packages=find_packages(where="."),
    package_dir={"": "."},
    python_requires=">=3.7",
    install_requires=[
        "xsdata>=25.0",
    ],
    extras_require={
        "dev": [
            "pytest>=7.0",
            "black>=22.0",
            "mypy>=0.990",
        ],
    },
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Intended Audience :: Manufacturing",
        "Topic :: Software Development :: Code Generators",
        "Topic :: Scientific/Engineering :: Interface Engine/Protocol Translator",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
    ],
    keywords="CRCL, robotics, robot control, XSD, dataclasses, serialization",
    project_urls={
        "Bug Reports": "https://github.com/kaidokert/crcl/issues",
        "Source": "https://github.com/kaidokert/crcl",
    },
)