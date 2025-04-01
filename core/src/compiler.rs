use std::path::{Path, PathBuf};

use anyhow::Error;
use foundry_compilers::{Project, ProjectPathsConfig};

pub fn compile_contract() -> Result<(), anyhow::Error> {
    let contracts_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .to_owned()
        .join(PathBuf::from("src/solidity"));

    let project = Project::builder()
        .paths(ProjectPathsConfig::hardhat(&contracts_path).unwrap())
        .set_no_artifacts(false)
        .build(Default::default())
        .map_err(|e| Error::new(e).context("Failed to build project"))?;

    println!("Project:\n {:?}\n", project);
    let output = project
        .compile()
        .map_err(|e| Error::new(e).context("Failed to compile contracts"))?;

    println!("Output:\n {:?}\n", output);
    Ok(())
}
