# ashmaize-py

Python bindings for the ashmaize cryptographic hash function.

## Overview

ashmaize-py provides Python bindings to the ashmaize library, a memory-hard hash function that uses large ROMs (Read-Only Memory tables) for secure password hashing and key derivation.

## Features

- **Fast Rust implementation** with Python bindings via PyO3
- **Batch hashing** for processing multiple preimages efficiently
- **Configurable parameters** (loops, instructions, ROM size)
- **Two ROM generation methods**:
  - `TwoStep`: Faster generation, suitable for most use cases
  - `FullRandom`: More thorough but slower generation

## Quick Start

### Installation

```bash
# Create virtual environment
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install setuptools-rust wheel

# Build and install
pip install -e .
```

### Usage

```python
import ashmaize_py

# Build a ROM (1GB with TwoStep method - fast)
rom = ashmaize_py.build_rom_twostep("my_secret_key")

# Hash a single preimage
hash_result = rom.hash("my password")
print(hash_result)  # Hex-encoded hash

# Hash multiple preimages efficiently
results = rom.hash_batch(["password1", "password2", "password3"])

# Use custom parameters
custom_hash = rom.hash_with_params("password", nb_loops=16, nb_instrs=512)
```

## Platform-Specific Build Instructions

- **macOS**: See [BUILD_MACOS.md](BUILD_MACOS.md)
- **Linux**: Standard `pip install -e .` should work
- **Windows**: Requires Rust toolchain and MSVC build tools

## Building from Source

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))
- Python 3.8+
- pip and setuptools

### Build Steps

1. Clone the repository with submodules:
   ```bash
   git clone <repository-url>
   cd ashmaize-python
   ```
2. Install the ce-ashmaize submodule:

  ```bash
  git submodule update --init --recursive
   ```

3. Build and install:
   ```bash
   pip install -e .
   ```

4. Test:
   ```bash
   python test_ashmaize.py
   ```

### Midnight Miner

If you are using [Midnight Miner](https://github.com/djeanql/MidnightMiner), you need to:
- Exit the venv with `deactivate`
- Copy `target/release/libashmaize_py.so` to MidnightMiner's directory
- Rename to `ashmaize_py.so`

## API Reference

### `build_rom(key, size=1073741824)`
Build a ROM using FullRandom generation.
- `key` (str): Secret key for ROM generation
- `size` (int): ROM size in bytes (default: 1GB)

### `build_rom_twostep(key, size=1073741824, pre_size=16777216, mixing_numbers=4)`
Build a ROM using TwoStep generation (faster).
- `key` (str): Secret key for ROM generation
- `size` (int): ROM size in bytes (default: 1GB)
- `pre_size` (int): Pre-ROM size (default: 16MB)
- `mixing_numbers` (int): Mixing iterations (default: 4)

### `PyRom` Class

#### `hash(preimage)`
Hash a single preimage with default parameters (8 loops, 256 instructions).

#### `hash_with_params(preimage, nb_loops, nb_instrs)`
Hash with custom parameters.

#### `hash_batch(preimages)`
Hash multiple preimages efficiently (recommended for bulk operations).

#### `hash_batch_with_params(preimages, nb_loops, nb_instrs)`
Batch hash with custom parameters.

## Performance Tips

- Use **release builds** for production: `pip install -e . --config-settings=build-args="--release"`
- Use **batch hashing** when processing multiple preimages
- Start with **TwoStep ROM generation** for faster initialization
- Larger ROMs provide more security but require more memory

## License

See the LICENSE files in the ce-ashmaize directory.

## Credits

Built with [PyO3](https://pyo3.rs/) - Rust bindings for Python.
