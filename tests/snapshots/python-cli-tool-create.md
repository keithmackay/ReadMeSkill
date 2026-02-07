# csvkit-lite

A lightweight CSV processing toolkit for the command line. Select columns, filter rows, and compute summary statistics on CSV files without leaving the terminal.

## Highlights

- **Column selection** — Extract specific columns from any CSV file
- **Row filtering** — Filter rows with simple expressions like `age>30`
- **Summary statistics** — Quick stats for any CSV file
- **Rich output** — Colorized terminal output with the Rich library
- **Tabulated display** — Clean table formatting with Tabulate

## Getting Started

### Prerequisites

- Python 3.9+

### Installation

```bash
pip install csvkit-lite
```

Or install from source:

```bash
git clone <repo-url>
cd csvkit-lite
pip install -e ".[dev]"
```

## Usage

```bash
# Select specific columns
csvk select data.csv --columns name,age

# Filter rows
csvk filter data.csv --filter "age>30"

# Show summary statistics
csvk stats data.csv

# Write output to a file
csvk select data.csv --columns name,age --output filtered.csv
```

## Development

```bash
git clone <repo-url>
cd csvkit-lite
pip install -e ".[dev]"
pytest
```

| Command | Description |
|---------|-------------|
| `pytest` | Run test suite |
| `pytest --cov` | Run tests with coverage |
| `ruff check .` | Lint source files |

## Contributing

Contributions are welcome. Fork the repo, create a feature branch, and open a pull request.

## License

[Apache-2.0](LICENSE)
