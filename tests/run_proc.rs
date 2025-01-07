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

	let response = req_chunking.await?;
    response.print().await?;

    // Check the response body for success
    let body = response.json_body()?;
    assert_eq!(body["result"]["success"], true);

    Ok(())

}