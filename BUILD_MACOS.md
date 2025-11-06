# Building ashmaize-py on macOS

This guide provides step-by-step instructions for compiling the ashmaize-py Python extension on macOS.

## Prerequisites

### 1. Install Homebrew (if not already installed)
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### 2. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify installation:
```bash
rustc --version
cargo --version
```

### 3. Install Python 3
```bash
brew install python3
```

Verify installation:
```bash
python3 --version
pip3 --version
```

### 4. Install Python Build Dependencies
```bash
pip3 install --upgrade pip
pip3 install setuptools setuptools-rust wheel maturin
```

## Building the Extension

### Option 1: Using setup.py (Recommended for Development)

1. Create a virtual environment (recommended):
```bash
python3 -m venv venv
source venv/bin/activate
```

2. Install setuptools-rust:
```bash
pip install setuptools-rust wheel
```

3. Build and install the extension:
```bash
python setup.py develop
```

Or for a production build:
```bash
python setup.py install
```

### Option 2: Using pip (Simpler)

1. Create and activate virtual environment:
```bash
python3 -m venv venv
source venv/bin/activate
```

2. Install in development mode:
```bash
pip install -e .
```

### Option 3: Using maturin (Alternative)

If you prefer using maturin (a fast PyO3 builder):

1. Install maturin:
```bash
pip install maturin
```

2. Build and install:
```bash
maturin develop --release
```

Or just build wheels:
```bash
maturin build --release
```

## Testing the Installation

After building, test the extension:

```python
python3 -c "import ashmaize_py; print('Success! ashmaize_py loaded correctly')"
```

Or run a quick test:

```python
import ashmaize_py

# Build a ROM (this may take a moment)
rom = ashmaize_py.build_rom_twostep("test_key", size=1024*1024)  # 1MB for quick test

# Hash a preimage
result = rom.hash("hello world")
print(f"Hash result: {result}")
```

## Troubleshooting

### Issue: "error: can't find Rust compiler"
**Solution**: Make sure Rust is installed and in your PATH:
```bash
source $HOME/.cargo/env
rustc --version
```

### Issue: "No module named 'setuptools_rust'"
**Solution**: Install setuptools-rust:
```bash
pip install setuptools-rust
```

### Issue: Python.h not found
**Solution**: Install Python development headers:
```bash
brew install python3
```

### Issue: Architecture mismatch (Intel vs Apple Silicon)
**Solution**: Make sure your Python and Rust toolchain match your Mac's architecture:
```bash
# Check Python architecture
python3 -c "import platform; print(platform.machine())"

# Check Rust architecture
rustc -vV | grep host
```

For Apple Silicon Macs, you may need:
```bash
rustup target add aarch64-apple-darwin
```

For Intel Macs:
```bash
rustup target add x86_64-apple-darwin
```

## Cleaning Build Artifacts

To clean all build artifacts:
```bash
rm -rf build/ dist/ *.egg-info/ target/ *.so
cargo clean
```

## Building for Distribution

To create a wheel for distribution:

```bash
# Using setup.py
python setup.py bdist_wheel

# Using maturin (recommended)
maturin build --release
```

The wheel will be in the `dist/` or `target/wheels/` directory.

## Platform-Specific Notes

- **macOS 10.15+**: No special configuration needed
- **macOS with Apple Silicon (M1/M2/M3)**: All tools should work natively
- The compiled extension will have a `.dylib` extension on macOS (vs `.so` on Linux)

## Performance Notes

The release build (`--release` flag) is significantly faster than debug builds. Always use release mode for:
- Benchmarking
- Production use
- Building large ROMs

For development/debugging, you can omit the `--release` flag for faster compilation times.
