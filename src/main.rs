use deno_core::JsRuntime;
use deno_core::RuntimeOptions;

fn main() {
    let path = std::env::args().nth(1).expect("entrypoint specified");
    let source_code = std::fs::read_to_string(&path).expect("entrypoint loaded");

    let mut runtime = JsRuntime::new(RuntimeOptions {
        ..Default::default()
    });
    runtime
        .execute_script(&path, &source_code)
        .expect("execution succeeds");
}
