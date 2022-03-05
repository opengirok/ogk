use crate::utils::slack::send_webhook_message;
use std::env;

static ENV_VAR_SLACK_WEBHOOK_URL: &str = "OGK_SLACK_WEBHOOK_URL";

#[derive(Debug)]
pub enum PrintType {
  SLACK,
  DEFAULT,
}

pub async fn print(
  message: &str,
  print_type: &PrintType,
) -> Result<(), Box<dyn std::error::Error>> {
  match print_type {
    PrintType::SLACK => {
      println!("{}", message);

      let host = env::var(ENV_VAR_SLACK_WEBHOOK_URL);
      match host {
        Ok(h) => {
          let _result = send_webhook_message(&h, message).await;
        }
        _ => {}
      }
    }
    _ => {
      println!("{}", message);
    }
  };

  Ok(())
}
