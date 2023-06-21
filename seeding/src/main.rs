use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Root {
    trainings: Vec<Training>,
}

#[derive(Debug, Clone, Deserialize)]
struct Training {
    name: String,
    description: Option<String>,
    weight: f64,
    times: i32,
    tags: Vec<String>,
}

#[tokio::main]
async fn main() {
    let file = std::fs::read_to_string("./seeding.toml").unwrap();
    let root = toml::from_str::<Root>(&file).unwrap();

    let url = std::env::var("DATABASE_URL").unwrap();
    let db = sqlx::PgPool::connect(&url).await.unwrap();

    let training_count = sqlx::query!("SELECT COUNT(id) FROM trainings")
        .fetch_one(&db)
        .await
        .unwrap()
        .count
        .unwrap() as usize;

    for (i, training) in root.trainings.iter().enumerate() {
        if i < training_count {
            sqlx::query!(
                "UPDATE trainings SET name = $1, description = $2, weight = $3, times = $4, tags = $5 WHERE id = $6",
                training.name,
                training.description,
                training.weight,
                training.times,
                &training.tags,
                i as i64,
            )
            .execute(&db)
            .await
            .unwrap();
        } else {
            sqlx::query!(
                "INSERT INTO trainings (name, description, weight, times, tags) VALUES ($1, $2, $3, $4, $5)",
                training.name,
                training.description,
                training.weight,
                training.times,
                &training.tags,
            )
            .execute(&db)
            .await
            .unwrap();
        }
    }
}
