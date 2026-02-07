import click
from rich.console import Console

console = Console()

@click.group()
@click.version_option()
def cli():
    """A lightweight CSV processing toolkit."""
    pass

@cli.command()
@click.argument('input_file', type=click.Path(exists=True))
@click.option('--columns', '-c', help='Comma-separated column names to select')
@click.option('--output', '-o', type=click.Path(), help='Output file path')
def select(input_file, columns, output):
    """Select specific columns from a CSV file."""
    pass

@cli.command()
@click.argument('input_file', type=click.Path(exists=True))
@click.option('--filter', '-f', 'filter_expr', help='Filter expression (e.g., "age>30")')
def filter(input_file, filter_expr):
    """Filter rows based on an expression."""
    pass

@cli.command()
@click.argument('input_file', type=click.Path(exists=True))
def stats(input_file):
    """Show summary statistics for a CSV file."""
    pass
