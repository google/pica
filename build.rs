// Copyright 2023, Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate_module(&Path::new("src/uci_packets.pdl").canonicalize()?)
}

fn generate_module(in_file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let file_name = in_file.file_name().ok_or("Input file has no filename")?;
    let mut out_file = File::create(out_dir.join(file_name).with_extension("rs"))?;

    println!("cargo:rerun-if-changed={}", in_file.display());

    let mut sources = pdl_compiler::ast::SourceDatabase::new();
    let in_file_str = in_file.to_str().ok_or("Filename is not UTF-8")?;
    let parsed_file = pdl_compiler::parser::parse_file(&mut sources, in_file_str)
        .map_err(|err| format!("{err:?}"))?;
    let analyzed_file =
        pdl_compiler::analyzer::analyze(&parsed_file).map_err(|err| format!("{err:?}"))?;
    let rust_source = pdl_compiler::backends::rust::generate(&sources, &analyzed_file, &[]);
    out_file.write_all(rust_source.as_bytes())?;
    Ok(())
}
