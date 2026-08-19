pub mod models{
    use std::collections::HashMap;

    pub struct Placeholder;

    #[derive(Debug, Clone, Default)]
    pub struct Attachment {
        pub filename: String,
        pub content_type: String,
        pub size: usize,
        pub content_id: Option<String>,
        pub disposition: Option<String>,
        pub data: Option<Vec<u8>>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct EmailAddress {
        pub name: Option<String>,
        pub address: String,
    }

    #[derive(Debug, Clone)]
    pub struct Mail{
        pub header: String,
        pub body: String,
        pub from: Option<EmailAddress>,
        pub to: Vec<EmailAddress>,
        pub cc: Vec<EmailAddress>,
        pub bcc: Vec<EmailAddress>,
        pub subject: Option<String>,
        pub date: Option<String>,
        pub message_id: Option<String>,
        pub attachments: Vec<Attachment>,
        pub urls: Vec<String>,
        pub in_reply_to: Option<String>,
        pub references: Vec<String>,
        pub headers: HashMap<String, String>,
        pub is_multipart: bool,
        pub content_type: Option<String>,
    }

    pub type MailBox = Vec<Mail>;

    #[derive(Debug, Clone)]
    pub struct MailFolder {
        pub name: String,
        pub mails: MailBox,
        pub unread_count: usize,
        pub total_count: usize,
    }

    impl Default for MailFolder {
        fn default() -> Self {
            Self {name: String::new(),mails: Vec::new(),unread_count: 0,total_count: 0}
        }
    }

    #[derive(Debug, Clone)]
    pub struct MailSettings {
        pub imap_server: String,
        pub imap_port: u16,
        pub imap_username: String,
        pub imap_password: String,
        pub imap_use_ssl: bool,
        pub smtp_server: String,
        pub smtp_port: u16,
        pub smtp_username: String,
        pub smtp_password: String,
        pub smtp_use_ssl: bool,
        pub default_signature: Option<String>,
        pub auto_check_interval: Option<u64>,
    }

    impl Default for MailSettings {
        fn default() -> Self {
            Self {imap_server: String::new(),imap_port: 993,imap_username: String::new(),imap_password: String::new(),imap_use_ssl: true,smtp_server: String::new(),smtp_port: 587,smtp_username: String::new(),smtp_password: String::new(),smtp_use_ssl: true,default_signature: None,auto_check_interval: None}
        }
    }

    impl Default for Mail{
        fn default() -> Self {
            Self { header: "".to_string(), body: "".to_string(),from: None,to: Vec::new(),cc: Vec::new(),bcc: Vec::new(),subject: None,date: None,message_id: None,attachments: Vec::new(),urls: Vec::new(),in_reply_to: None,references: Vec::new(),headers: HashMap::new(),is_multipart: false,content_type: None}
        }
    }

    impl Mail{
        pub fn new() -> Self{
            Self::default()
        }

        pub fn get_raw(&self,seperator: Option<&str>) -> String{
            self.header.clone() + if let Some(x) = seperator{x}else{""} + &self.body
        }

        pub fn has_attachments(&self) -> bool {
            !self.attachments.is_empty()
        }

        pub fn total_attachment_size(&self) -> usize {
            self.attachments.iter().map(|a| a.size).sum()
        }

        pub fn n_attachments(&self) -> usize{
            self.attachments.len()
        }
    }




}