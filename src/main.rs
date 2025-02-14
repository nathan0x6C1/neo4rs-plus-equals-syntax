use neo4rs::{Graph, Node, Query, BoltType, BoltInteger, BoltString, BoltMap};
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

impl From<TestInput> for BoltType {
    fn from(input: TestInput) -> Self {
        let mut map = BoltMap::new();
        map.put(BoltString::from("id"), BoltType::Integer(BoltInteger::from(input.id)));
        map.put(BoltString::from("props"), BoltType::Map({
            let mut props_map = BoltMap::new();
            props_map.put(BoltString::from("name"), BoltType::String(BoltString::from(input.props.name)));
            props_map.put(BoltString::from("value"), BoltType::Integer(BoltInteger::from(input.props.value)));
            props_map
        }));
        BoltType::Map(map)
    }
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
                name: "EEEEExample3".to_string(),
                value: 15,
            },
        },
    ];

    let q = "
    UNWIND $listoftestinput as elt
    MERGE (t:Test {id: elt.id})
    SET t += elt.props
    RETURN t;
    ";

    let bolt_inputs: Vec<BoltType> = test_inputs.into_iter().map(|input| input.into()).collect();
    let mut result = graph.execute(Query::new(q.to_string()).param("listoftestinput", bolt_inputs)).await?;

    while let Ok(Some(row)) = result.next().await {
        let node: Node = row.get("t")?;
        // println!("Created/Updated Test node: {:?}", node);
        let id: i64 = node.get("id").unwrap();
        let name: String = node.get("name").unwrap();
        let value: i64 = node.get("value").unwrap();
        println!("id: {}, name: {}, value: {}", id, name, value);
    }

    Ok(())
}
