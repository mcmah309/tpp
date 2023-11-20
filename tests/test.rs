#[cfg(test)]
mod cli_tests {

	#[cfg(test)]
	mod help {
		use assert_cmd::Command;

		#[test]
		fn it_shows_help() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let assert = cmd.arg("--help").assert();
			assert.success().code(0);
		}
	}

	#[cfg(test)]
	mod basic {
		use assert_cmd::Command;
		use predicates::prelude::*;

		#[test]
		fn it_process_json_to_stdout() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let assert = cmd.arg("tests/fixtures/data/basic/basic.tera").arg("-c").arg("tests/fixtures/data/basic/basic.json").assert();
			assert.success().stdout(predicate::str::contains("Bob likes orange"));
		}

		#[test]
		fn it_process_yaml_to_stdout() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let assert = cmd.arg("tests/fixtures/data/basic/basic.tera").arg("-c").arg("tests/fixtures/data/basic/basic.yaml").assert();
			assert.success().stdout(predicate::str::contains("Bob likes orange"));
		}

		#[test]
		fn it_process_yaml_with_dashes() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let assert = cmd.arg("tests/fixtures/data/tests/dash-test.tera").arg("-c").arg("tests/fixtures/data/tests/dash-test.yaml").assert();
			assert.success().stdout(predicate::str::contains(
				"{\"sso.auth.success\":{\"description\":\"Authentification success\"}}",
			));
		}

		#[test]
		fn it_process_toml_to_stdout() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let assert = cmd.arg("tests/fixtures/data/basic/basic.tera").arg("-c").arg("tests/fixtures/data/basic/basic.toml").assert();
			assert.success().stdout(predicate::str::contains("Bob likes orange"));
		}
	}

	#[cfg(test)]
	mod stdin {
		use std::fs;

		use assert_cmd::Command;
		use predicates::prelude::*;

		#[test]
		fn it_process_json_stdin() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let stdin = fs::read_to_string("tests/fixtures/data/basic/basic.json").unwrap();

			let assert = cmd.write_stdin(stdin).arg("tests/fixtures/data/basic/basic.tera").arg("--stdin").assert();
			assert.success().stdout(predicate::str::contains("Bob likes orange"));
		}

		#[test]
		fn it_process_toml_stdin() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let stdin = fs::read_to_string("tests/fixtures/data/basic/basic.toml").unwrap();

			let assert = cmd.write_stdin(stdin).arg("tests/fixtures/data/basic/basic.tera").arg("--stdin").assert();
			assert.success().stdout(predicate::str::contains("Bob likes orange"));
		}

		#[test]
		fn it_process_yaml_stdin() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let stdin = fs::read_to_string("tests/fixtures/data/basic/basic.yaml").unwrap();

			let assert = cmd.write_stdin(stdin).arg("tests/fixtures/data/basic/basic.tera").arg("--stdin").assert();
			assert.success().stdout(predicate::str::contains("Bob likes orange"));
		}
	}

	#[cfg(test)]
	mod env {
		use assert_cmd::Command;
		use predicates::prelude::*;

		#[test]
		fn it_process_env_sample() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let assert = cmd
				.arg("tests/fixtures/data/env-basic/env-sample.tera")
				.arg("--env")
				.env("HOME", "/home/bob")
				.env("EDITOR", "joe")
				.env("LOGNAME", "bob")
				.assert();
			assert.success().stdout(predicate::str::contains("Hello **bob**."));
		}

		#[test]
		fn it_process_env_key_sample() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let assert = cmd
				.arg("tests/fixtures/data/env-key/env-key.tera")
				.arg("--env")
				.arg("--env-key")
				.arg("env")
				.env("HOME", "/home/bob")
				.env("EDITOR", "joe")
				.env("LOGNAME", "bob")
				.assert();
			assert.success().stdout(predicate::str::contains("Home: /home/bob"));
		}

		#[test]
		fn it_fetches_env() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let assert = cmd
				.arg("tests/fixtures/data/env-basic/env-count.tera")
				.arg("--env")
				.arg("--env-key")
				.arg("env")
				.env("FOOBAR", "945727385")
				.assert();
			assert.success().stdout(predicate::str::contains("FOOBAR=945727385"));
		}
	}

	#[cfg(test)]
	mod collisions {
		use assert_cmd::Command;
		use predicates::prelude::*;

		#[test]
		fn it_process_json_with_collision() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let assert = cmd
				.arg("--env")
				.arg("tests/fixtures/data/basic/basic.tera")
				.arg("-c")
				.arg("tests/fixtures/data/basic/basic.json")
				.env("title", "foobar")
				.assert();
			assert.success().stdout(predicate::str::contains("<title> foobar </title>"));
		}

		#[test]
		fn it_process_json_with_collision_and_env_first() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let assert = cmd
				.arg("--env")
				.arg("--env-first")
				.arg("tests/fixtures/data/basic/basic.tera")
				.arg("-c")
				.arg("tests/fixtures/data/basic/basic.json")
				.env("title", "foobar")
				.assert();
			assert.success().stdout(predicate::str::contains("<title> Demo </title>"));
		}

		#[test]
		fn it_fails_on_collision() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let assert = cmd
				.arg("--env")
				.arg("--fail-on-collision")
				.arg("tests/fixtures/data/basic/basic.tera")
				.arg("-c")
				.arg("tests/fixtures/data/basic/basic.json")
				.env("title", "foobar")
				.assert();
			assert.failure().code(1);
		}
	}

	#[cfg(test)]
	mod big {
		use assert_cmd::Command;
		use predicates::prelude::*;

		#[test]
		fn it_process_big_json_v12_nodoc() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let assert = cmd
				.arg("--env")
				.arg("tests/fixtures/data/polkadot-metadata/metadata.tera")
				.arg("-c")
				.arg("tests/fixtures/data/polkadot-metadata/v12.json")
				.assert();
			assert.success().stdout(predicate::str::contains("Metadata version V12"));
		}

		#[test]
		fn it_process_big_json_v12_with_doc() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let assert = cmd
				.arg("--env")
				.arg("tests/fixtures/data/polkadot-metadata/metadata.tera")
				.arg("-c")
				.arg("tests/fixtures/data/polkadot-metadata/v12.json")
				.env("DOC", "true")
				.assert();
			assert.success().stdout(predicate::str::contains("Metadata version V12"));
		}

		#[test]
		fn it_process_big_json_v13() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let assert = cmd
				.arg("--env")
				.arg("tests/fixtures/data/polkadot-metadata/metadata.tera")
				.arg("-c")
				.arg("tests/fixtures/data/polkadot-metadata/v13.json")
				.assert();
			assert.success().stdout(predicate::str::contains("Metadata version V13"));
		}
	}

	#[cfg(test)]
	mod misc {
		use assert_cmd::Command;

		#[test]
		fn it_makes_markdown_from_cargo_toml() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let assert = cmd.arg("tests/fixtures/data/cargo-toml/cargo-toml.tera").arg("-c").arg("Cargo.toml").assert();
			assert.success().code(0);
		}

		#[test]
		fn it_handles_error() {
			let mut cmd = Command::cargo_bin("tpp").unwrap();
			let assert = cmd.arg("tests/fixtures/data/throw/throw.tera").arg("-c").arg
			("tests/fixtures/data/throw/throw.toml").assert();
			assert.failure().code(1);
		}
	}
}
