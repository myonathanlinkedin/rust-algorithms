mod core;
use core::{Database, Value, Vector};
use std::time::Instant;

fn main() {
    // Basic unit tests
    let mut db = Database::new();
    db.create_table(
        "users",
        vec![
            core::Column {
                name: "id".to_string(),
                col_type: core::ColumnType::Int,
            },
            core::Column {
                name: "name".to_string(),
                col_type: core::ColumnType::Str,
            },
            core::Column {
                name: "embedding".to_string(),
                col_type: core::ColumnType::Vec,
            },
        ],
    )
    .unwrap();

    let vec1 = Vector {
        data: vec![0.1, 0.2, 0.3],
    };
    let vec2 = Vector {
        data: vec![0.4, 0.5, 0.6],
    };
    let vec3 = Vector {
        data: vec![0.7, 0.8, 0.9],
    };

    db.insert(
        "users",
        vec![
            Value::Int(1),
            Value::Str("Alice".to_string()),
            Value::Vec(vec1.clone()),
        ],
    )
    .unwrap();
    db.insert(
        "users",
        vec![
            Value::Int(2),
            Value::Str("Bob".to_string()),
            Value::Vec(vec2.clone()),
        ],
    )
    .unwrap();
    db.insert(
        "users",
        vec![
            Value::Int(3),
            Value::Str("Carol".to_string()),
            Value::Vec(vec3.clone()),
        ],
    )
    .unwrap();

    db.commit("Initial commit");

    // Vector search test
    let query = Vector {
        data: vec![0.1, 0.2, 0.3],
    };
    let results = db.vector_search("users", "embedding", &query, 2);
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0, 1); // Alice should be most similar

    // Benchmark: insert 10,000 rows
    let mut db_bench = Database::new();
    db_bench
        .create_table(
            "bench",
            vec![
                core::Column {
                    name: "id".to_string(),
                    col_type: core::ColumnType::Int,
                },
                core::Column {
                    name: "vec".to_string(),
                    col_type: core::ColumnType::Vec,
                },
            ],
        )
        .unwrap();

    let start = Instant::now();
    for i in 0..10_000 {
        let vec = Vector {
            data: vec![i as f32, (i + 1) as f32],
        };
        db_bench
            .insert(
                "bench",
                vec![
                    Value::Int(i),
                    Value::Vec(vec),
                ],
            )
            .unwrap();
    }
    let duration = start.elapsed();
    println!("Inserted 10,000 rows in {:?}", duration);

    // Benchmark: vector search on 10,000 rows
    let query = Vector {
        data: vec![5000.0, 5001.0],
    };
    let start = Instant::now();
    let _ = db_bench.vector_search("bench", "vec", &query, 5);
    let duration = start.elapsed();
    println!("Vector search on 10,000 rows took {:?}", duration);

    // All tests passed
    println!("All tests passed.");
}
