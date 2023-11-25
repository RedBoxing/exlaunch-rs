extern crate dotenvy;

use std::{env, fs::File, io::Write, path::Path};

fn main() {
    let env_file = Path::new(
        env::var("RUST_TARGET_PATH")
            .expect("No target directory!")
            .as_str(),
    )
    .join(".env");
    let out_dir = env::var("OUT_DIR").expect("No output directory!");
    let dest_path = Path::new(&out_dir).join("constants.rs");
    let mut f = File::create(&dest_path).expect("Could not create file!");

    dotenvy::from_path(env_file).ok();

    write!(&mut f, "use crate::LoadKind;\n\n").expect("Failed to write file");

    write!(
        &mut f,
        "pub const FAKE_HEAP_SIZE: usize = {};\n",
        env::var("FAKE_HEAP_SIZE").expect("FAKE_HEAP_SIZE not defined in .env")
    )
    .expect("Could not write file");
    write!(
        &mut f,
        "pub const LOAD_KIND: LoadKind = LoadKind::{};\n",
        env::var("LOAD_KIND").expect("LOAD_KIND not defined in .env")
    )
    .expect("Could not write file");
}
