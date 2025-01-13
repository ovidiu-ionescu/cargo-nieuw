# cargo nieuw

The tool is a wrapper around cargo new to generate two more files:

- rustfmt.toml
- justfile

## rustfmt.toml
Some default configuration to format the source code

## justfile
Contains commands for the regulat build, release, etc but more importantly has 
- configuration for env_logger
- configuration running tests sequentially with output enabled

## Justification
I wrote this tool because I found myself creating these files again and again and always struggling to remember the syntax.
