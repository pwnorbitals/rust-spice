use std::env;
use std::path::Path;

fn main() {
    // Detect if $CSPICE_DIR/lib/cspice.a exists but $CSPICE_DIR/lib/libcspice.a doesn't.
    // In this case, warn the user and suggest a command-line they can use

    let cspice_dir = env::var("CSPICE_DIR").unwrap_or_else(|_| {
        panic!("CSPICE_DIR environment variable must be set");
    });

    let cspice_dir = Path::new(&cspice_dir);

    let cspice_lib = cspice_dir.join("lib").join("cspice.a");
    let libcspice_lib = cspice_dir.join("lib").join("libcspice.a");

    if cspice_lib.exists() && !libcspice_lib.exists() {
        println!("cargo:warning=libcspice.a does not exist in $CSPICE_DIR/lib. You may need to run the following command to have it:");
        println!("cargo:warning=    cd $CSPICE_DIR/src/cspice && cp cspice.a libcspice.a");
    }
}
