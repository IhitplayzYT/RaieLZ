pub mod settings{
    use crate::parsers::models::models::MailSettings;
    use std::collections::HashMap;

    pub fn config_to_settings(config_content: &str) -> Result<MailSettings, String> {
        let mut ret = MailSettings::default();
        let mut buff = "";
        for line in config_content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with('[') && line.ends_with(']') {
                buff = &line[1..line.len()-1];
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                let key = k.trim();
                let value = v.trim();
                
                match buff {
                    "imap" => {
                        match key {
                            "server" => ret.imap_server = value.to_string(),
                            "port" => ret.imap_port = value.parse().unwrap_or(993),
                            "username" => ret.imap_username = value.to_string(),
                            "password" => ret.imap_password = value.to_string(),
                            "use_ssl" => ret.imap_use_ssl = value.parse().unwrap_or(true),
                            _ => {}
                        }
                    }
                    "smtp" => {
                        match key {
                            "server" => ret.smtp_server = value.to_string(),
                            "port" => ret.smtp_port = value.parse().unwrap_or(587),
                            "username" => ret.smtp_username = value.to_string(),
                            "password" => ret.smtp_password = value.to_string(),
                            "use_ssl" => ret.smtp_use_ssl = value.parse().unwrap_or(true),
                            _ => {}
                        }
                    }
                    "general" => {
                        match key {
                            "signature" => ret.default_signature = Some(value.to_string()),
                            "auto_check_interval" => ret.auto_check_interval = value.parse().ok(),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(ret)
    }

    pub fn parse_env_settings() -> Result<MailSettings, String> {
        let mut ret = MailSettings::default();
        ret.imap_server = std::env::var("IMAP_SERVER").unwrap_or_default();
        ret.imap_port = std::env::var("IMAP_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(993);
        ret.imap_username = std::env::var("IMAP_USERNAME").unwrap_or_default();
        ret.imap_password = std::env::var("IMAP_PASSWORD").unwrap_or_default();
        ret.imap_use_ssl = std::env::var("IMAP_USE_SSL").ok().and_then(|s| s.parse().ok()).unwrap_or(true);
        ret.smtp_server = std::env::var("SMTP_SERVER").unwrap_or_default();
        ret.smtp_port = std::env::var("SMTP_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(587);
        ret.smtp_username = std::env::var("SMTP_USERNAME").unwrap_or_default();
        ret.smtp_password = std::env::var("SMTP_PASSWORD").unwrap_or_default();
        ret.smtp_use_ssl = std::env::var("SMTP_USE_SSL").ok().and_then(|s| s.parse().ok()).unwrap_or(true);
        ret.default_signature = std::env::var("DEFAULT_SIGNATURE").ok();
        ret.auto_check_interval = std::env::var("AUTO_CHECK_INTERVAL").ok().and_then(|s| s.parse().ok());
        Ok(ret)
    }

    pub fn parse_commandline_settings(args: &HashMap<String, String>) -> MailSettings {
        let mut settings = MailSettings::default();
        if let Some(server) = args.get("imap-server") {
            settings.imap_server = server.clone();
        }
        if let Some(port) = args.get("imap-port") {
            settings.imap_port = port.parse().unwrap_or(993);
        }
        if let Some(username) = args.get("imap-username") {
            settings.imap_username = username.clone();
        }
        if let Some(password) = args.get("imap-password") {
            settings.imap_password = password.clone();
        }
        if let Some(use_ssl) = args.get("imap-use-ssl") {
            settings.imap_use_ssl = use_ssl.parse().unwrap_or(true);
        }
        if let Some(server) = args.get("smtp-server") {
            settings.smtp_server = server.clone();
        }
        if let Some(port) = args.get("smtp-port") {
            settings.smtp_port = port.parse().unwrap_or(587);
        }
        if let Some(username) = args.get("smtp-username") {
            settings.smtp_username = username.clone();
        }
        if let Some(password) = args.get("smtp-password") {
            settings.smtp_password = password.clone();
        }
        if let Some(use_ssl) = args.get("smtp-use-ssl") {
            settings.smtp_use_ssl = use_ssl.parse().unwrap_or(true);
        }
        if let Some(signature) = args.get("signature") {
            settings.default_signature = Some(signature.clone());
        }
        if let Some(interval) = args.get("auto-check-interval") {
            settings.auto_check_interval = interval.parse().ok();
        }
        settings
    }

    pub fn validate_settings(settings: &MailSettings) -> Result<(), String> {
        if settings.imap_server.is_empty() {
            return Err("IMAP server is required".to_string());
        }
        if settings.imap_username.is_empty() {
            return Err("IMAP username is required".to_string());
        }
        if settings.imap_password.is_empty() {
            return Err("IMAP password is required".to_string());
        }
        if settings.smtp_server.is_empty() {
            return Err("SMTP server is required".to_string());
        }
        if settings.smtp_username.is_empty() {
            return Err("SMTP username is required".to_string());
        }
        if settings.smtp_password.is_empty() {
            return Err("SMTP password is required".to_string());
        }
        Ok(())
    }

    pub fn settings_to_config(settings: &MailSettings) -> String {
        let mut ret = String::new();
        ret.push_str("[imap]\n");
        ret.push_str(&format!("server = {}\n", settings.imap_server));
        ret.push_str(&format!("port = {}\n", settings.imap_port));
        ret.push_str(&format!("username = {}\n", settings.imap_username));
        ret.push_str(&format!("password = {}\n", settings.imap_password));
        ret.push_str(&format!("use_ssl = {}\n", settings.imap_use_ssl));
        ret.push_str("\n");
        ret.push_str("[smtp]\n");
        ret.push_str(&format!("server = {}\n", settings.smtp_server));
        ret.push_str(&format!("port = {}\n", settings.smtp_port));
        ret.push_str(&format!("username = {}\n", settings.smtp_username));
        ret.push_str(&format!("password = {}\n", settings.smtp_password));
        ret.push_str(&format!("use_ssl = {}\n", settings.smtp_use_ssl));
        ret.push_str("\n");
        ret.push_str("[general]\n");
        if let Some(signature) = &settings.default_signature {
            ret.push_str(&format!("signature = {}\n", signature));
        }
        if let Some(interval) = settings.auto_check_interval {
            ret.push_str(&format!("auto_check_interval = {}\n", interval));
        }
        ret
    }
}