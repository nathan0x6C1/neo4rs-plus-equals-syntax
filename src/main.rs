use neo4rs::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uri = "bolt://127.0.0.1:17687";
    let graph = Graph::new(uri).await?;

    let mut result = graph.execute(query("RETURN 1+1")).await?;
    if let Some(row) = result.next().await {
        let row = row?;
        let value: i64 = row.get("1+1").unwrap();
        println!("Result of 1+1: {}", value);
    } else {
        println!("No results returned from the query.");
    }

    Ok(())
}
