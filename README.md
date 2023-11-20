# TPP (Tera Pre-Processor)
tpp (Tera Pre-Processor) is a command line utility for preprocessing files with the tera templating engine. Inspired by
other pre-processors like cpp and gpp.

## Usage
```
Usage: tpp [OPTIONS] <TEMPLATE_FILE>

Arguments:
  <TEMPLATE_FILE>  Path to template file to render

Options:
  -i, --include <INCLUDE>            Paths from which to include templates. This is needed if the template imports or includes any other templates. Can be either a file or a directory. If a directory, subdirectories of the directory are used as well
  -c, --context-file <CONTEXT_FILE>  The path to the context data. The context needs to be of type json | yaml | toml. If you prefer to pass the data as stdin, use `--stdin`
      --stdin                        If provided, the context data will be passed using stdin, rather than `-c`. Note: consider using this and `jq` if you need to merge different context files or parse context files
      --env                          If set, the current ENV will be merged with the Context. Merging happens after all `context` unless `env_first` is set
  -e, --env-key <ENV_KEY>            If provided, Puts all ENV data inside the key, instead of the root of the Context
      --env-first                    If set, the ENV will be applied before the context. This is useful if you want your data to override the ENV
      --fail-on-collision            If set, the command will fail if ENV and context conflict
  -o, --out <OUT>                    Optional output file. If not passed, stdout is used
      --escape                       Auto-escape rendered content. This is useful for HTML output
      --debug                        If set, prints debug information during operations. Useful for resolving issues
  -h, --help                         Print help
  -V, --version                      Print version
```


## Examples
Create a `Containerfile` from a template:
```bash
tpp Containerfile.in -c context.json -o Containerfile
```