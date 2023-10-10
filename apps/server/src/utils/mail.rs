use std::env;

use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    Address, Message, SmtpTransport, Transport,
};

use crate::{
    constant::{MAIL_FROM_DOMAIN, MAIL_FROM_NAME, MAIL_FROM_PASSWORD, MAIL_SMTP}, error::{ResponseError, email_format_error, env_error},
   
};

pub async fn send_email(to: String, context: String) -> Result<bool, ResponseError> {
    let from_user_result = env::var(MAIL_FROM_NAME);
    let from_domain_result = env::var(MAIL_FROM_DOMAIN);
    let from_password_result = env::var(MAIL_FROM_PASSWORD);
    let smtp_result = env::var(MAIL_SMTP);
    if let (Ok(from_user), Ok(from_domain), Ok(from_password), Ok(smtp)) = (
        from_user_result,
        from_domain_result,
        from_password_result,
        smtp_result,
    ) {
        let from_address = Address::new(&from_user, &from_domain)?;
        let from_mailbox = Mailbox::new(None, from_address);

        let to_split = to.split("@").collect::<Vec<&str>>();
        let to_user_option = to_split.get(0);
        let to_domain_option = to_split.get(1);
        let to_address;
        if let (Some(to_user), Some(to_domain)) = (to_user_option, to_domain_option) {
            to_address = Address::new(*to_user, *to_domain)?;
        } else {
            return Err(email_format_error());
        }
        let to_mailbox = Mailbox::new(None, to_address);
        // let aa = lettre::message::Mailbox::new("fzcode", "fzcode@126.com");
        let email = Message::builder()
            .from(from_mailbox)
            .to(to_mailbox)
            .subject("code")
            .header(ContentType::TEXT_PLAIN)
            .body(String::from(context))?;
        let creds = Credentials::new(
            format!("{}@{}", from_user, from_domain),
            from_password.to_owned(),
        );
        let smtp_server = SmtpTransport::relay(&smtp)?;
        
        // Open a remote connection to gmail
        let mailer = smtp_server.credentials(creds).build();
        let send_result =  mailer.send(&email)?;
       return Ok(send_result.is_positive())
    } else {
        return Err(env_error());
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::utils::mail::send_email;

//     #[tokio::test]
//     async fn test_send_mail() {
//         dotenv::dotenv().ok();
//         let hex = send_email(String::from("757566833@qq.com"), String::from("test3")).await;
//         println!("{:?}", hex);
//     }
// }
