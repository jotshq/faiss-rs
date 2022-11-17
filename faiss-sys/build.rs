#[cfg(feature = "build")]
use cmake;
use std::env;
use std::path::Path;

fn main() {
    #[cfg(feature = "build")]
    {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        let faiss_src_dir = Path::new(&manifest_dir).join("../../faiss");

        println!("Building faiss");

        let build_config = &mut cmake::Config::new(faiss_src_dir);
        build_config.very_verbose(true);

        build_config.define("CMAKE_BUILD_TYPE", "Release");
        build_config.define("FAISS_ENABLE_C_API", "ON");
        build_config.define("FAISS_ENABLE_GPU", "OFF");
        build_config.define("BUILD_SHARED_LIBS", "ON");
        build_config.define("FAISS_ENABLE_PYTHON", "OFF");
        build_config.define("BUILD_TESTING", "OFF");

        let dst = build_config.build();
        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("build/c_api").display()
        );
        println!("cargo:rustc-link-lib=faiss_c");
    }
    #[cfg(not(feature = "build"))]
    {
        println!("cargo:rustc-link-lib=faiss_c");
    }
}
