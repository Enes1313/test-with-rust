/// Represents the `package.metadata.foreigntest` configuration table
///
/// The foreigntest crate can be configured through a `package.metadata.foreigntest` table
/// in the `Cargo.toml` file of the kernel. This struct represents the parsed configuration
/// options.
#[derive(Debug, Default)]
struct Config {
    /// The project path to test
    ///
    /// The path must be `absolute`.
    pub project_path: String,
    /// The run command that is invoked on `bootimage run` or `bootimage runner`
    ///
    /// The path must be `relative` to the `project_path`.
    pub compile_commands_path: String,
    /// Additional arguments passed to the runner for test binaries
    ///
    /// The paths must be `relative` to the `project_path`.
    pub support_header_files_path: Option<String>,
    /// Additional arguments passed to the runner for not-test binaries
    ///
    /// The paths must be `relative` to the `project_path`.
    pub exclude_header_files_paths: Option<Vec<String>>,
    /// Additional arguments passed to the runner for test binaries
    ///
    /// The paths must be `relative` to the `project_path`.
    pub extra_header_files_paths: Option<Vec<String>>,
    /// Compiler args
    pub compile_args: Option<Vec<String>>,
    /// Linker args
    pub linker_args: Option<Vec<String>>,
}

fn read_config(manifest_path: &std::path::Path) -> Option<Config> {
    let cargo_toml = {
        let mut manifest_file =
            std::fs::File::open(manifest_path).expect("Failed to open Cargo.toml");

        let mut content = String::new();

        std::io::Read::read_to_string(&mut manifest_file, &mut content)
            .expect("Failed to read Cargo.toml");

        content
            .parse::<toml::Value>()
            .expect("Failed to parse Cargo.toml")
    };

    let foreigntest = cargo_toml
        .get("package")
        .and_then(|table| table.get("metadata"))
        .and_then(|table| table.get("foreigntest"))
        .expect("Failed to find configuration")
        .as_table()
        .expect("Failed to parse configuration");

    let mut config = Config::default();

    for (key, value) in foreigntest {
        match (key.as_str(), value.clone()) {
            ("project_path", toml::Value::String(project_path)) => {
                config.project_path = project_path;
            }
            ("compile_commands_path", toml::Value::String(compile_commands_path)) => {
                config.compile_commands_path = compile_commands_path;
            }
            ("support_header_files_path", toml::Value::String(support_header_files_path)) => {
                config.support_header_files_path = Some(support_header_files_path);
            }
            ("exclude_header_files_paths", toml::Value::Array(exclude_header_files_paths)) => {
                config.exclude_header_files_paths =
                    Some(parse_string_array(exclude_header_files_paths));
            }
            ("extra_header_files_paths", toml::Value::Array(extra_header_files_paths)) => {
                config.extra_header_files_paths =
                    Some(parse_string_array(extra_header_files_paths));
            }
            ("compile_args", toml::Value::Array(compile_args)) => {
                config.compile_args = Some(parse_string_array(compile_args));
            }
            ("linker_args", toml::Value::Array(linker_args)) => {
                config.linker_args = Some(parse_string_array(linker_args));
            }
            _ => {
                return None;
            }
        }
    }
    Some(config)
}

fn parse_string_array(array: Vec<toml::Value>) -> Vec<String> {
    let mut parsed = Vec::new();
    for value in array {
        match value {
            toml::Value::String(s) => parsed.push(s),
            _ => (),
        }
    }
    parsed
}

fn main() {
    // Cargo sets a CARGO_MANIFEST_DIR environment variable for all runner
    // executables. This variable contains the path to the Cargo.toml of the
    // crate that the executable belongs to (i.e. not the project root
    // manifest for workspace projects)
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("Failed to find manifest dir.");
    let manifest_path = std::path::Path::new(&manifest_dir).join("Cargo.toml");
    let config = read_config(&manifest_path).expect("Failed to read config");

    // println!("config: {:#?}", config);

    let project_path = std::path::PathBuf::from(config.project_path)
        .canonicalize()
        .expect("Failed to canonicalize path");
    println!("project_path: {:#?}", project_path.to_str());

    let compile_commands_path = project_path.join(config.compile_commands_path);
    println!(
        "compile_commands_path: {:#?}",
        compile_commands_path.to_str()
    );

        // TODO: find headers from project paths and use compile_commands_path
        let c_header_files_paths_to_binding_str = vec![
            "source/app/app_example.h",
            "source/util/util_example.h",
            "lib/lib_example.h",
        ];

        // bindings

        let bindings_path = std::path::PathBuf::from("bindings");

        // bindgen
        // https://github.com/rust-lang/rust-bindgen/issues/1949
        // install clang
        
        for c_file_path_str in c_header_files_paths_to_binding_str {
            let c_file_to_bind_path = project_path.join(c_file_path_str);

            bindgen::Builder::default()
                .header(
                    c_file_to_bind_path
                        .to_str()
                        .expect("Path is not a valid string"),
                )
                .clang_arg("-I")
                .use_core()
                .allowlist_file(project_path.to_str().unwrap().to_string() + ".*")
                .layout_tests(false)
                .parse_callbacks(Box::new(bindgen::CargoCallbacks))
                .generate()
                .unwrap()
                .write_to_file(
                    bindings_path.join(
                        c_file_to_bind_path
                            .file_stem()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string()
                            + ".rs",
                    ),
                )
                .unwrap();
        }

        // mocks
        
        {
            let mocks_path = std::path::PathBuf::from("mocks");

            let bindings_str_without_functions = bindgen::Builder::default()
                .header(
                    project_path
                        .join("source/util/util_example.h")
                        .to_str()
                        .expect("msg"),
                )
                .blocklist_function(".*")
                .layout_tests(true)
                .parse_callbacks(Box::new(bindgen::CargoCallbacks))
                .generate()
                .expect("Unable to generate bindings")
                .to_string();

            let mut bindings_str = "#[cfg(test)]\nuse mockall::automock;\n".to_owned();
            bindings_str.push_str("#[cfg_attr(test, automock)]\npub(crate) mod ffi {\nuse super::*;\n");

            bindings_str.push_str(
                &bindgen::Builder::default()
                    .header(
                        project_path
                            .join("source/util/util_example.h")
                            .to_str()
                            .expect("msg"),
                    )
                    .blocklist_type(".*")
                    .allowlist_function(".*")
                    .layout_tests(false)
                    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
                    .generate()
                    .expect("Unable to generate bindings")
                    .to_string(),
            );
            bindings_str.push_str("}\n");
            bindings_str.push_str(&bindings_str_without_functions);

            std::fs::write(mocks_path.join("mock_util_example.rs"), bindings_str.as_bytes())
                .expect("Couldn't write bindings!");
        }
        {
            let mocks_path = std::path::PathBuf::from("mocks");

            let bindings_str_without_functions = bindgen::Builder::default()
                .header(
                    project_path
                        .join("lib/lib_example.h")
                        .to_str()
                        .expect("msg"),
                )
                .blocklist_function(".*")
                .layout_tests(true)
                .parse_callbacks(Box::new(bindgen::CargoCallbacks))
                .generate()
                .expect("Unable to generate bindings")
                .to_string(); 

            let mut bindings_str = "#[cfg(test)]\nuse mockall::automock;\n".to_owned();
            bindings_str.push_str("#[cfg_attr(test, automock)]\npub(crate) mod ffi {\nuse super::*;\n");

            bindings_str.push_str(
                &bindgen::Builder::default()
                    .header(
                        project_path
                            .join("lib/lib_example.h")
                            .to_str()
                            .expect("msg"),
                    )
                    .blocklist_type(".*")
                    .allowlist_function(".*")
                    .layout_tests(false)
                    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
                    .generate()
                    .expect("Unable to generate bindings")
                    .to_string(),
            );
            bindings_str.push_str("}\n");
            bindings_str.push_str(&bindings_str_without_functions);

            std::fs::write(mocks_path.join("mock_lib_example.rs"), bindings_str.as_bytes())
                .expect("Couldn't write bindings!");
        }
        // compile and linking

        // TODO: 
        // for util_example : use x
        // for app_example : use y

        // let x = project_path.join("source/util/util_example.c");
        let y = project_path.join("source/app/app_example.c");
        // sources
        let c_modules_path_for_test = vec![y.to_str().expect("Path is not a valid string")];

        cc::Build::new()
            .files(c_modules_path_for_test)
            // .define(var, val)
            // .extra_warnings(false)
            // .flag_if_supported("-Wall")
            // .flag_if_supported("-Wextra")
            // .flag_if_supported("-m32") //  apt-get install gcc-multilib for m32
            .flag_if_supported("-std=c99")
            .flag_if_supported("-funsigned-char")
            .flag_if_supported("-Wno-missing-field-initializers")
            // .flag_if_supported("-fprofile-arcs")
            // .flag_if_supported("-ftest-coverage")
            .includes(vec![project_path
                .join("source/app/app_example.h")
                .parent()
                .unwrap()
                .to_str()
                .expect("msg"),
                project_path
                .join("lib/lib_example.h")
                .parent()
                .unwrap()
                .to_str()
                .expect("msg"),
                project_path
                .join("source/util/util_example.h")
                .parent()
                .unwrap()
                .to_str()
                .expect("msg")])
            // .object(obj)
            //.expand();
            .compile("foo");
    return;
    /*
    if !std::process::Command::new("clang")
        .arg("-std=c99")
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-fprofile-arcs")
        .arg("-ftest-coverage")
        //.arg("-m32")
        .arg("-Wno-missing-field-initializers")
        .arg("-funsigned-char")
        .arg("-c")
        .arg("-o")
        .arg(&obj_path)
        .arg(sources_path_str)
        .output()
        .expect("could not spawn `clang`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not compile object file : {}", sources_path_str);
    }

    // Run `ar` to generate the `libhello.a` file from the `hello.o` file.
    // Unwrap if it is not possible to spawn the process.
    if !std::process::Command::new("ar")
        .arg("rcs")
        .arg(lib_path)
        .arg(obj_path)
        .output()
        .expect("could not spawn `ar`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not emit library file");
    }
    */
}
