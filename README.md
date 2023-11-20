# TPP (Tera Pre-Processor)

[![crates.io](https://img.shields.io/crates/v/tpp.svg)](https://crates.io/crates/tpp)
[![License: MIT](https://img.shields.io/badge/license-MIT-purple.svg)](https://opensource.org/licenses/MIT)

`tpp` (Tera Pre-Processor) is a versatile CLI (Command Line Interface) tool crafted for preprocessing files using the 
Tera templating engine. Drawing inspiration from renowned pre-processors like [cpp](https://linux.die.net/man/1/cpp) 
and [gpp](https://github.com/logological/gpp/), `tpp` stands out with its user-friendly command-line interface that 
efficiently renders templates for diverse applications.

Learn more about [Tera](https://crates.io/crates/tera)


## Example
Create a `Dockerfile` from a template:

#### Dockerfile.in
```dockerfile
FROM {{ base_image }}

LABEL maintainer="{{ maintainer }}"

RUN apt-get update && apt-get install -y \
{{ packages | join(' ') }}

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

## Usage
```
Usage: tpp [OPTIONS] <TEMPLATE_FILE>

Arguments:
  <TEMPLATE_FILE>  Path to template file to render

Options:
  -c, --context-file <CONTEXT_FILE>  The path to the context data. The context needs to be of type json | yaml | toml. If you prefer to pass the data as stdin, use `--stdin`
      --stdin                        If provided, the context data will be passed using stdin. Note: consider using this and `jq` if you need to merge different context files or parse context files
  -i, --include <INCLUDE>            Specifies the directories, including their subdirectories, where additional templates can be found. It's necessary when the `<TEMPLATE_FILE>` will import or include other templates. Note: any relative paths specified in the `import` or `include` statements within templates are resolved relative to the directories indicated by `--include`
      --env                          If set, the current ENV will be used as context, and merged if `--context-file` or `--stdin` are also provided. Merging ENV context happens after unless `--env-first` is set. See also `--fail-on-collision`
  -e, --env-key <ENV_KEY>            If provided, all ENV data is put inside the key, instead of the root of the context
      --env-first                    If set, the ENV context will be applied before any other context. This is useful if you want your data to override the ENV
      --fail-on-collision            If set, the command will fail if ENV and another context conflict
  -o, --out <OUT>                    Optional output file. If not passed, stdout is used
      --escape                       Auto-escape rendered content. This is useful for HTML output
      --debug                        If set, prints debug information to stdout
  -h, --help                         Print help
  -V, --version                      Print version
```