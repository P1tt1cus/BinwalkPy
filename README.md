# BinwalkPy

BinwalkPy is a Python wrapper for the Rust `binwalk` tool, designed to facilitate the analysis and extraction of firmware images. This project leverages the performance and safety of Rust while providing a convenient Python interface for users.

## Features

- Analyze firmware images for embedded files and executable code.
- Extract embedded files from firmware images.
- Support for various compression and file system formats.

## Requirements

- Rust (stable toolchain)
- Python 3.12

### Installing Dependencies

1. **Clone the repository:**

```sh
git clone https://github.com/P1tt1cus/BinwalkPy.git
cd BinwalkPy
```

2. **Set up a Python virtual environment:**

```sh
python3.12 -m venv .venv
source .venv/bin/activate
```

3. **Install Python dependencies:**

```sh
pip install maturin
```

4. **Build and install the Python package:**

```sh
maturin develop --release
```

## Usage

### Analyzing a Firmware Image

```python

import binwalkpy

#[pyfunction]
# fn scan_file(file_path: &str) -> PyResult<Vec<HashMap<String, String>>>

# Analyze a firmware image
results = binwalkpy.scan_file("path/to/your/firmware/image")
for result in results:
    print(result)

```

### Extracting Embedded Files

```python

import binwalkpy

# #[pyfunction]
# fn extract(
#     file_path: String,
#     output_path: Option<String>,
#     include: Option<Vec<String>>,
#     exclude: Option<Vec<String>>,
#     full_search: Option<bool>,
# ) -> PyResult<Vec<HashMap<String, String>>>

file_path = os.path.abspath("./path/to/image")
output_path = os.path.abspath("./extracted")

# Only extract PNG, HTML
results = binwalkpy.extract(file_path, include=["png", "html"], full_search=True)
for extract_result in results:
    print(extract_result)

```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Binwalk](https://github.com/ReFirmLabs/binwalk) - The original Binwalk tool.
- [Rust](https://www.rust-lang.org/) - The programming language used for the core implementation.
- [Maturin](https://github.com/PyO3/maturin) - A tool for building and publishing Rust-based Python packages.