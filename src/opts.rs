use clap::{crate_authors, crate_version, Parser};
use std::path::PathBuf;

// Tera Pre-Processor command line utility leveraging the Tera template engine. The only required input is the <TEMPLATE_FILE>;
// all other parameters are optional but offer extended functionality.
#[derive(Debug, Clone, Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Opts {
	/// Path to the template file you wish to render.
	#[clap(index = 1)]
	pub template_file: PathBuf,

	/// Optional: Specify the path to context data in JSON, YAML, or TOML format.
	#[clap(short, long,  conflicts_with_all = &["stdin"])]
	pub context_file: Option<PathBuf>,

	/// Optional: Enable passing context data via standard input. Useful for merging
	/// different context files or processing context data with tools like `jq`.
	#[clap(long, conflicts_with_all = &["context_file"])]
	pub stdin: bool,

	/// Optional: Define directories (and their subdirectories) to search for
	/// additional templates referenced in `<TEMPLATE_FILE>`.
	/// Necessary for templates that import or include other files.
	/// Note: any relative paths specified in the `import` or `include` statements within templates
	/// are resolved relative to the directories indicated by `--include`.
	#[clap(short, long, number_of_values = 1)]
	pub include: Vec<PathBuf>,

	/// Optional: Use current environment variables as context data. This can be merged
	/// with data from `--context-file` or `--stdin`. Merging occurs after, unless `--env-first` is set.
	/// Useful for dynamic template data population.
	#[clap(long)]
	pub env: bool,

	/// Optional: Designate a specific key under which all environment variables will be nested
	/// in the context data. Requires `--env` to be set.
	#[clap(short, long, requires = "env")]
	pub env_key: Option<String>,

	/// Optional: Apply environment variable context before any other context. Allows another context to
	/// override the env context. Requires `--env` to be set.
	#[clap(long, requires = "env")]
	pub env_first: bool,

	/// Optional: Command will terminate if there's a conflict between environment variables and
	/// other context data. Requires `--env` to be set.
	#[clap(long, requires = "env")]
	pub fail_on_collision: bool,

	/// Optional: Specify an output file to write the rendered template. If omitted,
	/// the output is directed to standard output (stdout).
	#[clap(short, long)]
	pub out: Option<PathBuf>,

	/// Optional: Enable auto-escaping of rendered content, which is particularly useful for HTML.
	#[clap(long)]
	pub escape: bool,

	/// Optional: Enable debug mode to print detailed debug information to standard output (stdout).
	#[clap(long)]
	pub debug: bool,
}
