use neo4rs::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uri = "bolt://127.0.0.1:17687";
    let username = "neo4j";
    let password = "password";
    let graph = Graph::new(uri, username, password).await?;

    let mut result = graph.execute(query("RETURN 1+1")).await?;
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
