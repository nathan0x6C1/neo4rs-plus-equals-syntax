#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uri = "bolt://127.0.0.1:17687";
    let username = "neo4j";
    let password = "password";
    let graph = neo4rs::Graph::new(uri, username, password).await?;
    let q = "RETURN 1+1";

    let mut result = graph.execute(neo4rs::query(q)).await?;
    match result.next().await {
        Ok(Some(row)) => {
            let value: i64 = row.get("1+1").unwrap();
            println!("Result of 1+1: {}", value);
        },
        Ok(None) => println!("No results returned from the query."),
        Err(e) => println!("Error fetching result: {}", e),
    }

    Ok(())
}
