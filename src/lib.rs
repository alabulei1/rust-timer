use schedule_flows::{schedule_cron_job, schedule_handler};
use slack_flows::send_message_to_channel;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    // Note: the cron syntax uses UTC time.
    // Adjust the time according to your desired timezone.
    schedule_cron_job(String::from("0 6 00 10 *"), String::from("Please remember to shut down the AWS machine on 2023/10/31.")).await;
}

#[schedule_handler]
async fn handler(body: Vec<u8>) {
    send_message_to_channel(
        "secondstate",
        "ai",
        String::from_utf8_lossy(&body).into_owned(),
    ).await;
}
