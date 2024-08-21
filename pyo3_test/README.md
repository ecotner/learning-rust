# PyO3 test

## Setup instructions (from scratch)
(FYI these are the steps I tool to initialize the project, don't run this in an existing project.)
Run the following
```bash
mkdir pyo3_test # create a new empty directory
cd pyo3_test
python3.12 -m venv .venv # create a python virtual environment
source .venv/bin/activate # activate the environment
pip install maturin # install maturin
maturin init # initialize maturin; choose `pyo3` bindings when presented the choice
# if using poetry:
# update the pyproject.toml to add `authors`, `description`, `name`, and `version` fields under [tool.poetry]
poetry lock
```

This should create a skeleton of a project you can start developing in with a `src/lib.rs` file where you can put rust functions you'd like to call from python, and a `Cargo.toml` with the base dependencies you need to compile.
From now on, when you want to compile your rust code into something that can be run from python, you need to run
```bash
maturin develop
```

This installs your rust binary as an editable python package (in your virtual environment).
You can test that the sample code provided by the initialization works
```
$ python
Python 3.12.1 (main, Dec 10 2023, 15:16:41) [GCC 9.4.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> from pyo3_test import sum_as_string
>>> sum_as_string(100, 1)
'101'
```