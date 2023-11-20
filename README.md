# TPP (Tera Pre-Processor)
tpp (Tera Pre-Processor) is a command line utility for preprocessing files with the tera templating engine. Inspired by
other pre-processors like cpp and gpp.

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


## Examples
Create a `Containerfile` from a template:
```bash
tpp Containerfile.in -o Containerfile
```
Slightly more complex example:
```bash
tpp Containerfile.in -c context.json -i . -i ../more_templates --env -o Containerfile 
```