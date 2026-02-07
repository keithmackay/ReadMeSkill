import pytest
from click.testing import CliRunner
from csvkit_lite.cli.main import cli

def test_select_columns():
    runner = CliRunner()
    result = runner.invoke(cli, ['select', 'sample.csv', '-c', 'name,age'])
    assert result.exit_code == 0
