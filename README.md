![Core](https://github.com/OpenMined/syft_experimental/workflows/Core/badge.svg)
![Python](https://github.com/OpenMined/syft_experimental/workflows/Python/badge.svg)

# Syft Experiment

This repo is a coordinated effort to build a Rust alternative for PySyft, with native
host language bindings.

This README.md is formatted with remark:
mrmlnc.vscode-remark

## Monorepo Structure

The folder structure looks like this:

```
├── platforms
│   └── python    <- Syft Python Host Code
├── protos        <- Shared Proto definitions
└── syft          <- Syft Core Rust Code
    ├── src
    └── target
```

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

Rust comes with an opinionated formatter and linter so we will mandate that these
are used.

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

We use a virtual environment to isolate the syft core python wheel development and
build process.

We include support for Pipenv, Conda and pip with virtualenv.

### Formatting and Linting

To keep code clean and bug free we mandate all code inside syft core python, uses an
agreed upon set of linting and formatting standards.

- black - https://github.com/psf/black
- isort - https://github.com/timothycrosley/isort
- mypy - http://mypy-lang.org/

```
$ pip install black isort mypy
```

### VSCode Configuration

Add these to your settings.json, making sure to update the paths as necessary to your
platform.

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
$ conda create --name syft --file requirements.txt
```

#### pip and virtualenv

Create a virtualenv in the /platforms/python folder and install the packages inside
requirements.txt

### Virtualenv

Make sure to enable your virtualenv from Pipenv, conda or other virtualenv system when
doing any commands relating to maturin or python.

If you are using pipenv:

```
$ cd platforms/python
$ pipenv shell
```

If you are using conda:

```
$ cd platforms/python
$ conda activate syft
```

## Python Development

You can compile and install the python library from the virtualenv in one command:

```
$ maturin develop
```

## Python Tests

We are using pytest which is listed in the Pipfile / requirements.txt.

Run tests from the platforms/python directory inside your virtualenv:

```
$ cd platforms/python
$ maturin develop; pytest
```

## Mixed Python & Rust Module Imports

The rust crate pyo3 allows us to mix compiled Rust code as a CPython module and vanilla
python code in the same wheel. The vanilla python code must go into a folder named the
same as the module and must contain at least a single `__init__.py` file in that folder.
That is why you will see this inside /platforms/python:

```
├── src           <--- Rust Code
│   └── ffi
├── syft          <--- Python Code
│   ├── message
│   ├── node
│   └── protos
├── target
│   ├── debug
│   ├── rls
│   └── wheels
└── tests         <--- Python Tests
```

To allow for a nice consistent import interface there is some code inside the vanilla
python source which acts to convert the awkward issues with CPython module names
and the way the Rust pyo3 submodules are defined.

Without this fix, importing a Rust module defined as syft.message might look like:

```
from syft.syft import message
```

Importing further nested items results in an error:

```
from syft.syft.message import run_class_method_message
>>> ModuleNotFoundError: No module named 'syft.syft.message';
```

However by re-exporting the modules using a matching directory structure and
`__init__.py` files we can provide a clean interface:

```
# import from rust in platforms/python/src
from syft.message import run_class_method_message
# import from vanilla python in platforms/python/syft
```

https://github.com/PyO3/maturin/issues/326

## Build Python Wheel

During this step:

- The syft core rust library is built
- The synthesized python interface to the protos are compiled with protoc
- The python platform ffi code in platforms/python/src is compiled for your system arch
- The vanilla python code inside platforms/python/syft is added
- Code inside the `__init__.py` files allows for a consistent module import syntax
- A wheel is created with both these mixed source files

Build wheel and install wheel:

```
$ maturin build -i python
$ pip install `find -L ./target/wheels -name "*.whl"`
```

# Hello World Demo

## Start Worker from Python

```
$ pipenv shell
$ maturin develop
$ python -i examples/worker.py
```

You should see:

```
Starting node on [::1]:50051
Tokio thread started
Capability registered: hello
Capability registered: sum
Capability registered: sum_np
>>>
```

## Start Client from Python

```
$ pipenv shell
$ maturin develop
$ python -i examples/client.py
```

You should see:

```
Tokio thread started
Capabilities returned: ["sum", "hello", "sum_np"]
Node at: http://[::1]:50051 has capabilities: ['sum', 'hello', 'sum_np']
Hello: Client 1
6
6
>>>
```

Try issuing a command like:

```
>>>  execute_capability(target_addr, "sum", [i for i in range(0, 100)])
4950
>>>
```

## Jupyter Notebook

You can run the Hello World demo with jupyter by opening the two notebooks.

```
$ pipenv shell
$ maturin develop
$ jupyter notebook
```

Make sure to initialize the worker and register capability functions before sending requests from the client.

## Zero Config Port Forwarding

If you want to test this between computers on different networks over the internet try ngrok.

https://ngrok.com/

### Linux

### MacOS

Install ngrok:

```
$ brew cask install ngrok
```

Start your worker and pick a port then run ngrok like so:

```
$ ngrok tcp 50051
```

You should see output like:

```
Forwarding                    tcp://0.tcp.ngrok.io:12345 -> localhost:50051
```

In your client use the address like so:

```
target_addr = "tcp://0.tcp.ngrok.io:12345"
execute_capability(target_addr, "sum", [i for i in range(0, 10)])
45
>>>
```

## Apple

This section describes usage on Apple platforms and as such requires MacOS.

### Setup

- Xcode
- Xcode Command Line Tools

```
$ xcode-select --install
```

- Enable iphoneos SDK

```
$ sudo xcode-select --switch /Applications/Xcode.app
```

- swift-protobuf

```
$ brew install swift-protobuf
```

#### Rust

- Cargo Lipo

```
$ cargo install cargo-lipo
```

- cbindgen

```
$ cargo install --force cbindgen
```

- Apple Rust Targets

```
$ rustup target add aarch64-apple-ios x86_64-apple-ios
```

### Build Static Library

```
$ cd platforms/apple
$ ./build.sh
```

### Integrating Xcode Project

- Add libsyft.a
- Add Build Settings > Objective-C Bridging Header: `$(PROJECT_DIR)/include/syft.h`

### Open Xcode Example
