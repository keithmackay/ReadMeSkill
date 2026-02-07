import { Command } from 'commander';

const program = new Command()
  .name('formstack')
  .description('Scaffold forms from schema files')
  .version('1.0.0');

program.command('init').description('Initialize a form schema').action(() => {});
program.command('generate').description('Generate form from schema').action(() => {});

program.parse();
