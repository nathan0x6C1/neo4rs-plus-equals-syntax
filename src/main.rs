use neo4rs::{Graph, Node, Query};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TestInput {
    id: i64,
    props: TestProps,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestProps {
    name: String,
    value: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uri = "bolt://127.0.0.1:17687";
    let username = "neo4j";
    let password = "password";
    let graph = Graph::new(uri, username, password).await?;

    let test_inputs = vec![
        TestInput {
            id: 1234,
            props: TestProps {
                name: "Example1".to_string(),
                value: 42,
            },
        },
        TestInput {
            id: 5678,
            props: TestProps {
                name: "Example2".to_string(),
                value: 99,
            },
        },
        TestInput {
            id: 9101,
            props: TestProps {
                name: "Example3".to_string(),
                value: 15,
            },
        },
    ];

    let q = "
    UNWIND $whatever as elt
    MERGE (t:Test {id: elt.id})
    SET t += elt.props
    RETURN t;
    ";

    let mut result = graph.execute(Query::new(q.to_string()).param("whatever", &test_inputs)).await?;
    // let mut result = graph.execute(neo4rs::query(q).param("whatever", &test_inputs)).await?;

    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("t")?;
        println!("Created/Updated Test node: {:?}", node);
    }

    Ok(())
}
