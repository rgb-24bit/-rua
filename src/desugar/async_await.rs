// rustc +nightly --edition=2021 -Zunpretty=hir src/desugar/async_await.rs
async fn foo() -> u8 {
    5
}

async fn bar() -> u8 {
    let a = foo().await;
    let b = foo().await;
    a + b
}
