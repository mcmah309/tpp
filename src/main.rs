#![deny(clippy::expect_used)]
#![deny(clippy::unwrap_used)]

mod opts;
mod context_builder;

use clap::{crate_name, crate_version, Parser};
use color_eyre::eyre::{bail, Context as EyreContext, ContextCompat, Result};
use opts::*;
use std::{fs, fs::canonicalize, fs::File, io::Write};
use tera::{Context, Tera};

fn main() -> Result<()> {
	color_eyre::install()?;

	let opts = Opts::parse();

	let is_debug = opts.debug;
	if is_debug {
		println!("Running {} v{}", crate_name!(), crate_version!());
		println!("opts:\n{:#?}", opts);
	}

	let template = fs::read_to_string(&opts.template_file).context("failed to read the template")?;
	if is_debug {
		println!("template:\n{}", template);
	}

	let mut include_paths = Vec::with_capacity(opts.include.len());
	for path in &opts.include {
		let canonical_path = canonicalize(path)
			.context(format!("failed to get absolute path for `include` path of {:?}", path))?;
		if opts.debug {
			println!("Found canonical path {:?} from {:?}", canonical_path, path);
		}
		include_paths.push(canonical_path);
	}

	let mut tera = Tera::default();
	for path_buf in &include_paths {
		if path_buf.is_file() {
			tera.add_template_file(path_buf, None)
				.context(format!("Could not add {:?} as a template.", path_buf))?;
			if opts.debug {
				println!("Loaded {:?} as template file", path_buf);
			}
		} else if path_buf.is_dir() {
			let glob = format!("{}/**/*", path_buf.to_str()
				.context(format!("Invalid directory UTF8 string of {:?}.", path_buf))?);
			let tera_from_dir = Tera::parse(&glob)
				.context(format!("Could not add templates in dir glob {}.", glob))?;
			tera.extend(&tera_from_dir)
				.context(format!("Could not extend tera instance created form templates in dir glob {}.",
								 glob))?;
			if opts.debug {
				println!("Loaded file from glob {} as template files", glob);
			}
		} else {
			bail!(format!("Could not add {:?} to tera, not a file or a directory.", path_buf));
		}
	}

	let context_builder = context_builder::ContextBuilder::new(opts.clone());
	let context: Context = context_builder.build().context("failed to build context")?;

	if is_debug {
		println!("context:\n{:#?}", context);
	}

	tera.build_inheritance_chains().context("Building inheritance chain failed.")?;

	if !opts.escape {
		tera.autoescape_on(vec![])
	};

	let rendered = tera.render_str(&template, &context).context("failed to render")?;

	if let Some(out_file) = opts.out {
		if is_debug {
			println!("Saving to {}", out_file.display());
		}
		let mut file = File::create(out_file).context("failed to open output file")?;
		file.write_all(rendered.as_bytes()).context("failed to write to output file")?;
	} else {
		println!("{rendered}");
	}

	Ok(())
}
