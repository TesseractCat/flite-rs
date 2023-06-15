use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::{WalkDir, DirEntry};

extern crate cc;

fn filter_c(entry: Result<DirEntry, walkdir::Error>) -> Option<PathBuf> {
    let entry = entry.unwrap();
    if entry.file_type().is_file() && entry.path().extension().is_some_and(|ext| ext == "c") {
        Some(entry.into_path())
    } else {
        None
    }
}

fn main() {
    if !Path::new("flite/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status();
    }

    // CC
    let paths =
        WalkDir::new("flite/src/synth").into_iter().filter_map(filter_c)
        .chain(WalkDir::new("flite/src/utils").into_iter().filter_map(filter_c).filter(|e|
            e.file_name().is_some_and(|f| !f.to_string_lossy().starts_with("cst_file"))
            && e.file_name().is_some_and(|f| !f.to_string_lossy().starts_with("cst_mmap"))
        ))
        .chain(WalkDir::new("flite/src/hrg").into_iter().filter_map(filter_c))
        .chain(WalkDir::new("flite/src/cg").into_iter().filter_map(filter_c))
        .chain(WalkDir::new("flite/src/lexicon").into_iter().filter_map(filter_c))
        .chain(WalkDir::new("flite/src/regex").into_iter().filter_map(filter_c))
        .chain(WalkDir::new("flite/src/speech").into_iter().filter_map(filter_c))
        .chain(WalkDir::new("flite/src/stats").into_iter().filter_map(filter_c))
        .chain(WalkDir::new("flite/src/wavesynth").into_iter().filter_map(filter_c))
        .chain(WalkDir::new("flite/lang/cmu_us_slt").into_iter().filter_map(filter_c))
        .chain(WalkDir::new("flite/lang/cmu_us_kal").into_iter().filter_map(filter_c))
        .chain(WalkDir::new("flite/lang/usenglish").into_iter().filter_map(filter_c))
        .collect::<Vec<PathBuf>>();

    cc::Build::new()
        .define("CST_NO_SOCKETS", None)
        .file("flite/src/utils/cst_mmap_none.c")
        .file("flite/src/utils/cst_file_stdio.c")

        .file("flite/src/audio/auclient.c")
        .file("flite/src/audio/auserver.c")
        .file("flite/src/audio/audio.c")
        .file("flite/src/audio/au_streaming.c")
        .file("flite/src/audio/au_none.c")

        .file("flite/lang/cmulex/cmu_lts_rules.c")
        .file("flite/lang/cmulex/cmu_lts_model.c")
        .file("flite/lang/cmulex/cmu_lex.c")
        .file("flite/lang/cmulex/cmu_lex_entries.c")
        .file("flite/lang/cmulex/cmu_lex_data.c")
        .file("flite/lang/cmulex/cmu_postlex.c")

        .files(paths)
        .include("flite/include")
        .include("flite/src/cg")

        .include("flite/lang/usenglish")
        .include("flite/lang/cmulex")
        .include("flite/lang/cmu_us_kal")
        .include("flite/lang/cmu_us_slt")
        .compile("flite");

    // Bindgen
    /*println!("cargo:rustc-link-search=flite/include");
    println!("cargo:rustc-link-lib=flite");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");*/
}
