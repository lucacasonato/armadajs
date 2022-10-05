use deno_core::error::AnyError;
use deno_core::include_js_files;
use deno_core::op;
use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;

#[tokio::main]
async fn main() {
    let path = std::env::args().nth(1).expect("entrypoint specified");
    let source_code = std::fs::read_to_string(&path).expect("entrypoint loaded");

    let armada_ext = Extension::builder()
        .js(include_js_files!(
            prefix "armada:",
            "main.js",
        ))
        .ops(vec![op_version::decl(), op_read_file::decl()])
        .build();

    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![armada_ext],
        ..Default::default()
    });
    runtime
        .execute_script(&path, &source_code)
        .expect("execution succeeds");
    runtime
        .run_event_loop(false)
        .await
        .expect("execution succeeds");
}

#[op]
fn op_version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

#[op]
async fn op_read_file(path: String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(&path).await?;
    Ok(contents)
}
