# Syft Experiment

This repo is a coordinated effort to build a Rust alternative for PySyft.

This README.md is formatted with remark:
mrmlnc.vscode-remark

## Setup

- python 3.7+ - https://www.python.org/
- rustup - https://rustup.rs/
- bloomrpc - https://github.com/uw-labs/bloomrpc
- protoc - https://github.com/protocolbuffers/protobuf
- vscode - https://github.com/microsoft/vscode

### Linux

### MacOS

Python

```
$ brew install python
```

rustup

```
$ brew install rustup
$ rustup-init
```

bloomrpc

```
$ brew cask install bloomrpc
```

protoc

```
$ brew install protobuf
```

### Windows

## Rust Toolchain

We are currently using nightly due to some rust dependencies.

```
$ rustup toolchain install nightly
$ rustup default nightly
```

### Formatting and Linting

Rust comes with an opinionated formatter and linter so we will mandate that these are used.

Install Rust Format:

```
$ rustup component add rustfmt
```

Install Rust Language Server:

```
$ rustup component add rls
```

Install Rust Linting:

```
$ rustup component add clippy
```

### VSCode Configuration

While VSCode is not required it is highly recommended.

Install Rust VSCode Extension:
https://marketplace.visualstudio.com/items?itemName=rust-lang.rust

```
$ code --install-extension rust-lang.rust
```

Install Even Better TOML Extension:
https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml

```
$ code --install-extension tamasfe.even-better-toml
```

Add to settings:

```
{
  "evenBetterToml.formatter.reorderKeys": false,
  "evenBetterToml.formatter.alignEntries": true
}
```

## Python

### Setup

Make sure you have `python3.7+`

We use a virtual environment to isolate the syft core python wheel development and build process.

We include support for Pipenv, Conda and pip with virtualenv.

### Formatting and Linting

To keep code clean and bug free we mandate all code inside syft core python, uses an agreed upon set of linting and formatting standards.

- black - https://github.com/psf/black
- isort - https://github.com/timothycrosley/isort
- mypy - http://mypy-lang.org/

```
$ pip install black isort mypy
```

### VSCode Configuration

Add these to your settings.json, making sure to update the paths as necessary to your platform.

```
{
  "python.linting.enabled": true,
  "python.linting.mypyEnabled": true,
  "python.formatting.provider": "black",
  "python.linting.mypyPath": "/usr/local/bin/mypy",
  "python.formatting.blackPath": "/usr/local/bin/black"
}
```

### Python Package Managers

#### Pipenv

Upgrade pip:

```
$ pip install --upgrade pip
```

Install pipenv:

```
$ pip install pipenv
```

Enter virtualenv:

```
$ cd platforms/python
$ pipenv shell
```

Install packages:

```
$ pipenv install --dev --skip-lock
```

#### Conda

Create your conda environment, navigate to the /platforms/python directory:

```
conda create --name syft --file spec-file.txt
```

#### pip and virtualenv

Create a virtualenv in the /platforms/python folder and install the packages inside requirements.txt

## Python Development

You can compile and install the python library from the virtualenv in one command:

```
$ maturin develop
```

## Python Tests

To test it out try:

```
$ python tests/message.py
```

## Build Python Wheel

During this step we:

- Build and install the Python wheel
- Compile the protos for Python use and output to the ./src/syft/protos directory

If you are using pipenv:

```
$ cd platforms/python
$ pipenv shell
$ touch build.rs
$ maturin build -i python
$ pip install `find -L ./target/wheels -name "*.whl"`
```

If you are using conda and have you conda environment activated (`conda activate syft`):

```
$ cd platforms/python
$ touch build.rs
$ maturin build -i python
$ pip install `find -L ./target/wheels -name "*.whl"`
```
