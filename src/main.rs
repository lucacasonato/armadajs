use std::path::Path;

use deno_core::error::AnyError;
use deno_core::include_js_files;
use deno_core::op;
use deno_core::url::Url;
use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_fetch::FetchPermissions;
use deno_web::BlobStore;
use deno_web::TimersPermission;

struct AllowAllPermissions;

impl TimersPermission for AllowAllPermissions {
    fn allow_hrtime(&mut self) -> bool {
        true
    }

    fn check_unstable(&self, _state: &deno_core::OpState, _api_name: &'static str) {
        // allow unstable apis
    }
}

impl FetchPermissions for AllowAllPermissions {
    fn check_net_url(&mut self, _url: &Url, _api_name: &str) -> Result<(), AnyError> {
        Ok(())
    }

    fn check_read(&mut self, _path: &Path, _api_name: &str) -> Result<(), AnyError> {
        Ok(())
    }
}

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
        .state(|state| {
            state.put(AllowAllPermissions);
            Ok(())
        })
        .build();

    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![
            deno_console::init(),
            deno_webidl::init(),
            deno_url::init(),
            deno_web::init::<AllowAllPermissions>(BlobStore::default(), None),
            deno_fetch::init::<AllowAllPermissions>(deno_fetch::Options {
                user_agent: format!("armadajs/{}", env!("CARGO_PKG_VERSION")),
                ..Default::default()
            }),
            armada_ext,
        ],
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
