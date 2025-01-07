use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn run_chunking() -> Result<()> {

	let hc = httpc_test::new_client("http://localhost:8080")?;
	
	let req_chunking = hc.do_post(
		"/v1/run",
		json!({ "list_of_files": [
            "example--vesting.mdx",
            "example--gift-card.mdx"
        ]}),
	);
	req_chunking.await?.print().await?;
	Ok(())
}