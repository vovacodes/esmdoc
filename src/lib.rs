mod utils;

use crate::utils::set_panic_hook;
use anyhow::anyhow;
use deno_doc::parser::DocFileLoader;
use deno_doc::{DocError, DocParser};
use futures::future::LocalBoxFuture;
use futures::FutureExt;
use std::io::ErrorKind;
use swc_ecmascript::parser::{Syntax, TsConfig};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = getDocAst)]
pub async fn get_doc_ast(
    specifier: String,
    resolve: js_sys::Function,
    load_source_code: js_sys::Function,
) -> Result<JsValue, JsValue> {
    set_panic_hook();

    let loader = Box::new(EsmdocLoader {
        resolve,
        load_source_code,
    });

    let parser = DocParser::new(loader, false);

    let doc_nodes = parser
        .parse_with_reexports(&specifier)
        .await
        .map_err(|err| {
            JsValue::from_str(&format!(
                "failed to generate doc AST the source file: {}",
                err.to_string()
            ))
        })?;

    JsValue::from_serde(&doc_nodes).map_err(|err| err.to_string().into())
}

struct EsmdocLoader {
    resolve: js_sys::Function,
    load_source_code: js_sys::Function,
}

impl DocFileLoader for EsmdocLoader {
    fn resolve(&self, specifier: &str, referrer: &str) -> Result<String, DocError> {
        let this = JsValue::null();
        self.resolve
            .call2(
                &this,
                &JsValue::from_str(&specifier),
                &JsValue::from_str(&referrer),
            )
            .map_err(|_| {
                DocError::Resolve(format!(
                    "error when calling `resolve` with specifier {} and referrer {}",
                    specifier, referrer
                ))
            })?
            .as_string()
            .ok_or_else(|| {
                DocError::Resolve(format!(
                    "expected `resolve` to return a string for specifier {} and referrer {}",
                    specifier, referrer
                ))
            })
    }

    fn load_source_code(
        &self,
        specifier: &str,
    ) -> LocalBoxFuture<'_, Result<(Syntax, String), DocError>> {
        let this = JsValue::null();
        let specifier = specifier.to_owned();

        async move {
            let return_value: JsValue = self
                .load_source_code
                .call1(&this, &JsValue::from_str(&specifier))
                .map_err(|_| {
                    DocError::Io(std::io::Error::new(
                        ErrorKind::Other,
                        anyhow!(
                            "error when calling `loadSourceCode` with specifier {}",
                            specifier
                        ),
                    ))
                })?;
            let promise = js_sys::Promise::from(return_value);

            let source_code = wasm_bindgen_futures::JsFuture::from(promise)
                .await
                .map_err(|_| {
                    DocError::Io(std::io::Error::new(
                        ErrorKind::Other,
                        anyhow!("failed to load the source file {}", specifier),
                    ))
                })?
                .as_string()
                .ok_or_else(|| {
                    DocError::Io(std::io::Error::new(
                        ErrorKind::Other,
                        anyhow!(
                            "`loadSourceCode` for {} is expected to be resolved to a string",
                            specifier
                        ),
                    ))
                })?;

            Ok((Syntax::Typescript(TsConfig::default()), source_code))
        }
        .boxed_local()
    }
}
