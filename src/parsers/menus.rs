pub mod menus{
    use crate::parsers::models::models::{Mail, MailFolder};
    use std::collections::HashMap;

    // Sample Common Wordlist latter migh replace with a ML spam classifier
    const SPAM_TERMS: &[&str] = &["viagra","casino","lottery","you won","claim your prize","free money","act now","limited time"];
    const AUTOMATED_SENDERS: &[&str] = &["noreply@","no-reply@","donotreply@","notifications@","alerts@","mailer-daemon@"];

    #[derive(Debug, Clone, Default)]
    pub struct MailboxMenu {
        pub folders: HashMap<String, MailFolder>,
        pub current_folder: Option<String>,
    }

    impl MailboxMenu {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn add_folder(&mut self, name: String) {
            self.folders.insert(name.clone(), MailFolder {name: name.clone(),mails: Vec::new(),unread_count: 0,total_count: 0});
        }

        pub fn get_folder(&self, name: &str) -> Option<&MailFolder> {
            self.folders.get(name)
        }

        pub fn get_folder_mut(&mut self, name: &str) -> Option<&mut MailFolder> {
            self.folders.get_mut(name)
        }

        pub fn add_mail_to_folder(&mut self, folder_name: &str, mail: Mail) {
            if let Some(folder) = self.get_folder_mut(folder_name) {
                folder.mails.push(mail);
                folder.total_count += 1;
            }
        }

        pub fn set_current_folder(&mut self, name: String) {
            if self.folders.contains_key(&name) {
                self.current_folder = Some(name);
            }
        }

        pub fn get_current_folder(&self) -> Option<&MailFolder> {
            self.current_folder.as_ref().and_then(|name| self.folders.get(name))
        }

        pub fn list_folders(&self) -> Vec<&str> {
            self.folders.keys().map(|s| s.as_str()).collect()
        }
    }

    pub fn parse_imap_folders(flist: &str) -> Vec<String> {
        flist.lines().filter_map(|line| {
            if line.contains('"') {
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() >= 2 {
                    Some(parts[1].to_string())
                } else {
                    None
                }
            } else {
                None
            }
        }).collect()
    }


    pub fn categorize_mail(mail: &Mail) -> MailCategory {
        let subject = mail.subject.as_deref().unwrap_or("");
        let from = mail.from.as_ref().map(|f| f.address.as_str()).unwrap_or("");
        let subject = subject.to_ascii_lowercase();
        let from = from.to_ascii_lowercase();

        let (mut s_spam,mut s_notif) = (0,0);

        for x in SPAM_TERMS {
            if subject.contains(x){
                s_spam += 3;
            }
        }

        for x in AUTOMATED_SENDERS {
            if from.contains(x) {
                s_notif += 3;
            }
        }

        if mail.in_reply_to.is_some() || subject.starts_with("re:") || subject.starts_with("fw:") || subject.starts_with("fwd:"){
            s_notif -= 1;
        }

        if s_spam >= 3 {
            MailCategory::Spam
        } else if s_notif >= 2 {
            MailCategory::Notifications
        } else {
            MailCategory::Inbox
        }
    }

    pub fn init_mailboxes() -> MailboxMenu {
        let mut menu = MailboxMenu::new();        
        menu.add_folder("Inbox".to_string());
        menu.add_folder("Sent".to_string());
        menu.add_folder("Drafts".to_string());
        menu.add_folder("Spam".to_string());
        menu.add_folder("Trash".to_string());
        menu.add_folder("Archive".to_string());
        menu.add_folder("Notifications".to_string());
        menu.set_current_folder("Inbox".to_string());        
        menu
    }

    pub fn parse_mailbox(raw: &str) -> MailboxMenu {
        let mut menu = init_mailboxes();
        let folders = parse_imap_folders(raw);
        for folder in folders {
            if !menu.folders.contains_key(&folder) {
                menu.add_folder(folder);
            }
        }
        
        menu
    }

    pub fn extract_menu_items(header: &str) -> Vec<String> {
        header.lines().filter(|line| line.contains(':')).map(|line| line.split(':').next().unwrap_or("").trim().to_string()).collect()
    }

    pub fn parse_folder_stats(raw: &str) -> Option<(String, usize, usize)> {
        let parts: Vec<&str> = raw.split_whitespace().collect();
        if parts.len() >= 3 {
            Some((parts[0].to_string(), parts[1].parse().ok().unwrap_or(0), parts[2].parse().ok().unwrap_or(0)))
        } else {
            None
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MailCategory {
        Inbox,
        Spam,
        Notifications,
        Promotions,
    }

}