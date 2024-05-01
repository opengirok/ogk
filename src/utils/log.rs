use crate::utils::config;
use crate::utils::slack::send_webhook_message;

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

            let _config = config::Config::load_or_new()?;

            match _config.slack_webhook_url {
                Some(url) => {
                    let _result = send_webhook_message(&url, message).await;
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
