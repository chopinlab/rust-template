use std::error::Error;
// use std::fmt::Debug;
use config::{Config, ConfigError};
use configs::settings::Settings;
use job_scheduler::{Job, JobScheduler};
use std::time::Duration;
use rust_template::configs;
use rust_template::service::cost;

// fn run() -> Result<Settings, ConfigError> {
fn run() -> Result<Settings, ConfigError> {

    let settings = match initEnv() {
        Ok(settings) => settings,
        Err(e) => return Err(e),
    };
    runCron(&settings);
    Ok(settings)
}

fn initEnv() -> Result<Settings, ConfigError> {
    println!("main function");
    let settings = Settings::new();
    println!("{:?}", settings);
    settings
}

fn runCron(settings: &Settings) {

    let mut scheduler = JobScheduler::new();
    let cron = &settings.scheduler.cron;
    // let cron = settings.unwrap().scheduler.cron;
    // Adding a task to scheduler to execute it in every 2 minutes.
    scheduler.add(Job::new(cron.parse().unwrap(), || {
        cost::evaluate()
    }));

    // Adding a task to scheduler to execute it in every 2 minutes.
    // scheduler.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
    //     println!("Get executed every 10 seconds!");
    // }));
    loop {
        scheduler.tick();
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn main() {
    if let Err(err) = run() {
        panic!("{}", err);
    }
}