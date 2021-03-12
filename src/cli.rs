//! Utilities interfacing with misc cli tools

use log::warn;

/// Gets user-friendly node name
///
/// # Examples
///
/// ```
///    let node = kraken_utils::cli::get_node_name();
///    assert_ne!(node.len(), 0);
/// ```
pub fn get_node_name() -> String {
    let get_name = || -> Result<String, String> {
        let res = std::process::Command::new("uname")
            .arg("-n")
            .output()
            .expect("err");
        let mut name = String::from(std::str::from_utf8(&res.stdout).expect("err in parse"));
        if name.ends_with('\n') || name.ends_with('\r') {
            name.pop();
        }
        Ok(String::from(name))
    };
    match get_name() {
        Ok(n) => n,
        _ => {
            warn!("Unable to find node name, using default");
            String::from("unknown-model")
        }
    }
}
