use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn run_chunking() -> Result<()> {

	let hc = httpc_test::new_client("http://localhost:8080")?;
	
	let req_chunking = hc.do_post(
		"api/v1/run",
		json!({ "list_of_files": [
            "Upscaling_and_simulation_of_composite_ga.txt",
            "Guar_Gum_in_Hydraulic_Fracturing_in_Indi.txt",
            "Shale_Gas_An_Overview.txt",
            "Lacustrine_sediments_petroleum_deposits.txt",
            "The_Brief_Bibliometric_Analysis_of_the_T.txt"
        ]}),
	);

	let response = req_chunking.await?;
    response.print().await?;

    // Check the response body for success
    let body = response.json_body()?;
    assert_eq!(body["result"]["success"], true);

    Ok(())

}