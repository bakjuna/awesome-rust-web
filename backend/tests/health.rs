use anyhow::Result;
use httpc_test::Response;

#[tokio::test]
async fn health() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    let response: Response = hc.do_get("/healthz").await?;
    assert_eq!(response.status(), 200);
    Ok(())
}
