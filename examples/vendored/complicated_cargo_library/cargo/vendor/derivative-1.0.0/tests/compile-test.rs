#[cfg(feature = "compiletest_rs")]
extern crate compiletest_rs as compiletest;

#[cfg(feature = "compiletest_rs")]
fn run_mode(dir: &'static str, mode: &'static str) {
    use std::path::PathBuf;
    use std::env::var;

    let mut config = compiletest::default_config();

    let cfg_mode = mode.parse().expect("Invalid mode");
    config.target_rustcflags = Some("-L target/debug/ -L target/debug/deps".to_owned());
    if let Ok(name) = var::<&str>("TESTNAME") {
        let s: String = name.to_owned();
        config.filter = Some(s)
    }

    config.mode = cfg_mode;
    config.src_base = PathBuf::from(format!("tests/{}", dir));

    compiletest::run_tests(&config);
}

#[cfg(feature = "compiletest_rs")]
#[test]
fn compile_test() {
    run_mode("compile-fail", "compile-fail");
}
