use std::sync::Arc;

use shaku::HasComponent;
use tokio_cron_scheduler::JobScheduler;

use crate::app_state::AppState;
use crate::database::{DBConnection, DatabaseConnectionPool, ConnectionPool};
use crate::errors::BootError;
use crate::cron::handler::CronJob;


pub async fn create_cron_jobs(app_state: &AppState) -> Result<JobScheduler, BootError> {
    let sched = JobScheduler::new().await;
    if sched.is_err() {
        return Err(BootError::CronJobInit);
    }
    let sched = sched.unwrap();
    let p: &dyn ConnectionPool = app_state.module.resolve_ref();
    let db = p.initialize();
    let pool = db.0.lock().await.to_owned();

    let cron_job = Arc::new(CronJob {} );
    let res = sched
        .add(cron_job.job(pool).unwrap())
        .await;
    if res.is_err() {
        return Err(BootError::CronJobRun);
    }
    Ok(sched)
}