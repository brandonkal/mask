use assert_cmd::prelude::*;
use colored::*;
use predicates::str::contains;

mod common;
use common::MaskCommandExt;

#[test]
fn errors_when_no_lang_code_is_specified() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
## missing
~~~
echo "this won't do anything..."
~~~
"#,
    );

    common::run_mask(&maskfile_path)
        .command("missing")
        .assert()
        .code(1)
        .stderr(contains(format!(
            "{} Command script requires a lang code which determines which executor to use.",
            "ERROR:".red()
        )))
        .failure();
}

#[test]
fn sh() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
## sh
~~~sh
echo Hello, $name!
~~~
"#,
    );

    common::run_mask(&maskfile_path)
        .command("sh")
        .env("name", "World")
        .assert()
        .stdout(contains("Hello, World!"))
        .success();
}

#[test]
fn bash() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
## bash
~~~bash
echo Hello, $name!
~~~
"#,
    );

    common::run_mask(&maskfile_path)
        .command("bash")
        .env("name", "World")
        .assert()
        .stdout(contains("Hello, World!"))
        .success();
}

#[test]
fn node() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
## node
~~~js
const { name } = process.env;
console.log(`Hello, ${name}!`);
~~~
"#,
    );

    common::run_mask(&maskfile_path)
        .command("node")
        .env("name", "World")
        .assert()
        .stdout(contains("Hello, World!"))
        .success();
}

#[test]
fn python() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
## python
~~~py
import os
name = os.getenv("name", "WORLD")
print("Hello, " + name + "!")
~~~
"#,
    );

    common::run_mask(&maskfile_path)
        .command("python")
        .env("name", "World")
        .assert()
        .stdout(contains("Hello, World!"))
        .success();
}

#[test]
fn ruby() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
## ruby
~~~ruby
name = ENV["name"] || "WORLD"
puts "Hello, #{name}!"
~~~
"#,
    );

    common::run_mask(&maskfile_path)
        .command("ruby")
        .env("name", "World")
        .assert()
        .stdout(contains("Hello, World!"))
        .success();
}

#[test]
fn php() {
    let (_temp, maskfile_path) = common::maskfile(
        r#"
## php
~~~php
$name = getenv("name") ?: "WORLD";

echo "Hello, " . $name . "!\n";
~~~
"#,
    );

    common::run_mask(&maskfile_path)
        .command("php")
        .env("name", "World")
        .assert()
        .stdout(contains("Hello, World!"))
        .success();
}
