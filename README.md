# TPP (Tera Pre-Processor)

[![crates.io](https://img.shields.io/crates/v/tpp.svg)](https://crates.io/crates/tpp)
[![License: MIT](https://img.shields.io/badge/license-MIT-purple.svg)](https://opensource.org/licenses/MIT)

`tpp` (Tera Pre-Processor) is a versatile CLI (Command Line Interface) tool crafted for preprocessing files using the 
Tera templating engine. Drawing inspiration from pre-processors like [cpp](https://linux.die.net/man/1/cpp) 
and [gpp](https://github.com/logological/gpp/), `tpp` is the next evolution with its powerful expressive toolset.

Learn more about [Tera](https://keats.github.io/tera/docs/)


## Simple Example
Create a `Dockerfile` from a template:

#### Dockerfile.in
```dockerfile
FROM {{ base_image }}

LABEL maintainer="{{ maintainer }}"

RUN apt-get update && apt-get install -y \
{{ packages | join(sep=' ') }}

COPY . /app
WORKDIR /app

ENV PORT {{ port }}
EXPOSE {{ port }}

CMD ["{{ entrypoint }}"]
```
#### context.json
```json
{
  "base_image": "python:3.8-slim",
  "maintainer": "dev@example.com",
  "packages": [
    "build-essential",
    "libpq-dev"
  ],
  "port": 8080,
  "entrypoint": "python app.py"
}
```
#### Command
```bash
tpp Dockerfile.in -c context.json -o Dockerfile
```
#### Dockerfile
```dockerfile
FROM python:3.8-slim

LABEL maintainer="dev@example.com"

RUN apt-get update && apt-get install -y \
    build-essential libpq-dev

COPY . /app
WORKDIR /app

ENV PORT 8080
EXPOSE 8080

CMD ["python app.py"]
```

## Install
```bash
cargo install --git https://github.com/mcmah309/tpp
```

## Usage
```
Usage: tpp [OPTIONS] <TEMPLATE_FILE>

Arguments:
  <TEMPLATE_FILE>  Path to the template file you wish to render

Options:
  -c, --context-file <CONTEXT_FILE>  Optional: Specify the path to context data in JSON, YAML, or TOML format
      --stdin                        Optional: Enable passing context data via standard input. Useful for merging different context files or processing context data with tools like `jq`
  -i, --include <INCLUDE>            Optional: Define directories (and their subdirectories) to search for additional templates referenced in `<TEMPLATE_FILE>`. Necessary for templates that import or include other files. Note: any relative paths specified in the `import` or `include` statements within templates are resolved relative to the directories indicated by `--include`
      --env                          Optional: Use current environment variables as context data. This can be merged with data from `--context-file` or `--stdin`. Merging occurs after, unless `--env-first` is set. Useful for dynamic template data population
  -e, --env-key <ENV_KEY>            Optional: Designate a specific key under which all environment variables will be nested in the context data. Requires `--env` to be set
      --env-first                    Optional: Apply environment variable context before any other context. Allows another context to override the env context. Requires `--env` to be set
      --fail-on-collision            Optional: Command will terminate if there's a conflict between environment variables and other context data. Requires `--env` to be set
  -o, --out <OUT>                    Optional: Specify an output file to write the rendered template. If omitted, the output is directed to standard output (stdout)
      --escape                       Optional: Enable auto-escaping of rendered content, which is particularly useful for HTML
      --debug                        Optional: Enable debug mode to print detailed debug information to standard output (stdout)
  -h, --help                         Print help
  -V, --version                      Print version
```