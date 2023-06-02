use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Style {
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
    let cmd = clap::Command::new("mutan-admin")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            clap::Command::new("training")
                .arg_required_else_help(true)
                .arg(clap::Arg::new("path").required(true)),
        );
    let matches = cmd.get_matches();

    let url = std::env::var("DATABASE_URL").unwrap();
    let db = sqlx::PgPool::connect(&url).await.unwrap();

    match matches.subcommand() {
        Some(("training", matches)) => {
            let path = matches.get_one::<String>("path").unwrap();
            let file = std::fs::read_to_string(&path).unwrap();
            let style = toml::from_str::<Style>(&file).unwrap();

            let trainings = sqlx::query!("SELECT COUNT(id) FROM train")
                .fetch_one(&db)
                .await
                .unwrap();

            let count = trainings.count.unwrap() as usize;

            for (i, training) in style.trainings.iter().enumerate() {
                if i < count {
                    sqlx::query!(
                        "UPDATE train SET name = $1, description = $2, weight = $3, times = $4, tags = $5",
                        training.name,
                        training.description,
                        training.weight,
                        training.times,
                        &training.tags,
                    )
                    .execute(&db)
                    .await
                    .unwrap();
                } else {
                    sqlx::query!(
                        "INSERT INTO train (name, description, weight, times, tags) VALUES ($1, $2, $3, $4, $5)",
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

            let trainings =
                sqlx::query!("SELECT id, name, description, weight, times, tags FROM train",)
                    .fetch_all(&db)
                    .await
                    .unwrap();

            println!("id, name, weight, times");
            for training in trainings {
                println!(
                    "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
                    training.id,
                    training.name,
                    training.description,
                    training.weight,
                    training.times,
                    training.tags,
                );
            }
        }
        _ => unreachable!(),
    }
}
