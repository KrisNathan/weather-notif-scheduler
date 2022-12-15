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

async fn troll() -> Result<String, reqwest::Error> {
    let xd = reqwest::get("http://127.0.0.1:8080/100/100")
        .await?
        .text()
        .await?;
    Ok(xd)
}
