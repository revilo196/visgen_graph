/// build script used to pre-compile spriv shaders using glslc
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::process::Command;
use std::str::from_utf8;

fn main() -> Result<(), Box<dyn Error>> {
    //println!("cargo:rerun-if-changed=shader_src");
    let folder = Path::new("shader_src");
    let folder_out = Path::new("shader");

    // Create destination path if necessary
    std::fs::create_dir_all(&folder_out)?;

    build_shader_folder(folder, folder_out)?;

    Ok(())
}

fn build_shader_folder(input: &Path, out_path: &Path) -> Result<(), Box<dyn Error>> {
    for entry in std::fs::read_dir(input)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let in_path = entry.path();
            if let Some(p) = in_path.extension() {
                if p == "frag" || p == "vert" {
                    build_shader(&in_path, out_path)?;
                }
            }
        }
    }

    Ok(())
}
#[derive(Debug, Clone)]
struct CompileError;
impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not compile shader")
    }
}
impl Error for CompileError {}

/// compile a shaders to spriv using glslc
fn build_shader(input: &Path, out_path: &Path) -> Result<(), Box<dyn Error>> {
    let filename = input.file_name().unwrap();
    // let extens = input.extension().unwrap();
    let output_name = filename.to_str().unwrap().replace(".", "_") + ".spv";
    let output_path = out_path.join(output_name);
    let out = Command::new(&"glslc")
        .arg(input.as_os_str())
        .arg("-I")
        .arg("shader_src/include") //include folder
        .arg("-o")
        .arg(output_path.as_os_str())
        .output()
        .expect("failed to compile shader");

    if !out.status.success() {
        eprintln!(
            "failed to compile shader {} {}",
            &out.status,
            from_utf8(&out.stderr)?
        );
        Err(Box::new(CompileError {}))
    } else {
        Ok(())
    }
}
