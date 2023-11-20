use clap::{crate_authors, crate_version, Parser};
use std::path::PathBuf;

// Tera Pre-Processor command line utility for the tera template engine.
#[derive(Debug, Clone, Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Opts {
	/// Path to template file to render.
	#[clap(index = 1)]
	pub template_file: PathBuf,

	/// Specifies the directory, including its subdirectories,
	/// where additional templates can be found.
	/// It's necessary when the `<TEMPLATE_FILE>` will import or include other templates.
	/// Note that any relative paths specified in the `import` or `include` statements within templates
	/// are resolved relative to the directories indicated by `--include`.
	#[clap(short, long, number_of_values = 1)]
	pub include: Vec<PathBuf>,

	/// The path to the context data. The context needs to be of type json | yaml | toml.
	/// If you prefer to pass the data as stdin, use `--stdin`.
	#[clap(short, long,  conflicts_with_all = &["stdin"], required_unless_present_any = &["stdin", "env"])]
	pub context_file: Option<PathBuf>,

	/// If provided, the context data will be passed using stdin, rather than `-c`. Note: consider using this and `jq`
	/// if you need to merge different context files or parse context files.
	#[clap(long, conflicts_with_all = &["context_file"], required_unless_present_any = &["context_file", "env"])]
	pub stdin: bool,

	/// If set, the current ENV will be merged with the Context. Merging happens after all `context` unless
	/// `env_first` is set.
	#[clap(long)]
	pub env: bool,

	/// If provided, Puts all ENV data inside the key, instead of the root of the Context.
	#[clap(short, long, requires = "env")]
	pub env_key: Option<String>,

	/// If set, the ENV will be applied before the context. This is useful
	/// if you want your data to override the ENV.
	#[clap(long, requires = "env")]
	pub env_first: bool,

	/// If set, the command will fail if ENV and context conflict.
	#[clap(long, requires = "env")]
	pub fail_on_collision: bool,

	/// Optional output file. If not passed, stdout is used.
	#[clap(short, long)]
	pub out: Option<PathBuf>,

	/// Auto-escape rendered content. This is useful for HTML output.
	#[clap(long)]
	pub escape: bool,

	/// If set, prints debug information during operations. Useful for resolving issues.
	#[clap(long)]
	pub debug: bool,
}
