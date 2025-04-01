use std::path::{Path, PathBuf};

use foundry_compilers::{solc::Solc, Project, ProjectPathsConfig};

pub fn compile_contract() -> Result<(), anyhow::Error> {
    // configure the project with all its paths, solc, cache etc.
    let project = Project::builder()
        .paths(
            ProjectPathsConfig::hardhat(
                &Path::new(env!("CARGO_MANIFEST_DIR"))
                    .to_owned()
                    .join(PathBuf::from("src/contracts")),
            )
            .unwrap(),
        )
        .build(Default::default())
        .unwrap();
    // https://github.com/foundry-rs/compilers/blob/main/crates/compilers/tests/project.rs#L2627

    println!("{:?}", project);
    let output = project.compile().unwrap();

    println!("{:?}\n", output);
    Ok(())
}
