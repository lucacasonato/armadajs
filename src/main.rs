use deno_core::include_js_files;
use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;

fn main() {
    let path = std::env::args().nth(1).expect("entrypoint specified");
    let source_code = std::fs::read_to_string(&path).expect("entrypoint loaded");

    let armada_ext = Extension::builder()
        .js(include_js_files!(
            prefix "armada:",
            "main.js",
        ))
        .build();

    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![armada_ext],
        ..Default::default()
    });
    runtime
        .execute_script(&path, &source_code)
        .expect("execution succeeds");
}
