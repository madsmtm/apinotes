//! Run with something like:
//!     cargo run --example parse_apinotes -- /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/System/Library/Frameworks/**/*.apinotes
use std::path::PathBuf;

use apinotes::ApiNotes;

fn main() {
    let paths = std::env::args_os().skip(1).map(PathBuf::from);

    for path in paths {
        println!("parsing {path:?}");
        if path.ends_with("SceneKit.apinotes") {
            println!("skipping {path:?}");
            continue;
        }
        let _res = ApiNotes::from_path(&path).expect("failed parsing");
        println!("successfully parsed {path:?}");
    }
}
