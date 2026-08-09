pub mod smtp{

    pub fn send_mail() -> Result<()> {
    let email = Message::builder()
        .from("alice@example.com".parse()?)
        .to("bob@example.com".parse()?)
        .subject("Hello")
        .body("Hello from Rust!")?;

    let creds = Credentials::new(
        "alice@example.com".to_string(),
        "password".to_string(),
    );

    let mailer = SmtpTransport::relay("smtp.example.com")?
        .credentials(creds)
        .build();

    mailer.send(&email)?;

    Ok(())


    }
    

}