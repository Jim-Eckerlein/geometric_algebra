use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    Command::new("cargo")
        .current_dir("./codegen")
        .arg("run")
        .arg("--")
        .arg(concat!(
            "pga3:0,1,1,1;",
            "Scalar:1;",
            "Plane:e1,e2,e3,e0;",
            "Flat:e1,e2,e3;",
            "Line:e01,e02,e03|e23,-e13,e12;",
            "IdealLine:e01,e02,e03;",
            "Branch:e23,-e13,e12;",
            "Point:-e023,e013,-e012,e123;",
            "Origin:e123;",
            "Dir:-e023,e013,-e012;",
            "Pseudoscalar:e0123;",
            "Translator:1,e01,e02,e03;",
            "Rotor:1,e23,-e13,e12;",
            "Motor:1,e23,-e13,e12|e0123,e01,e02,e03;",
            "Flector:e0,e1,e2,e3|-e023,e013,-e012,e123",
        ))
        .spawn()?;

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
