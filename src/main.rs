fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    let on_cron = move |name: &str| {
        match rt.block_on(troll()) {
            Ok(o) => println!("Job {} successfully ran: {}", name, o),
            Err(e) => eprintln!("Job {} encountered error: {}", name, e),
        };
    };
    let mut cron = cronjob::CronJob::new("fetch", on_cron);
    cron.seconds("0");
    cron.minutes("*/10");
    cron.start_job();
    Ok(())
}

async fn troll() -> Result<String, Box<dyn std::error::Error>> {
    let xd = reqwest::get(format!("{}", std::env::var("URL")?))
        .await?
        .text()
        .await?;
    Ok(xd)
}
