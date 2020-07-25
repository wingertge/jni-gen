use jni_gen::java_class;
use compiletest_rs as compiletest;
use std::path::PathBuf;
use compiletest_rs::common::Mode;

fn run_mode(mode: &'static str) {
    let mut config = compiletest::Config::default();

    config.mode = Mode::CompileFail;
    config.src_base = PathBuf::from(format!("tests/{}", mode));
    config.link_deps(); // Populate config.target_rustcflags with dependencies on the path
    config.clean_rmeta(); // If your tests import the parent crate, this helps with E0464

    compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode("compile-fail");
}

struct TestClass;

#[java_class("com.example.package")]
impl TestClass {
    pub fn test_fn() {}
    fn private_fn() {}
}

#[test]
fn generates_most_basic_example() {
    Java_com_example_package_TestClass_test_fn();
}

// Should fail to compile
/*#[test]
fn ignores_private_fields() {
    Java_com_example_package_TestClass_private_fn();
}*/