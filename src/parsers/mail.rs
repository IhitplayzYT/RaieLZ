pub mod mail{
    use std::vec;
    use crate::parsers::models::models::{Mail, MailBox, EmailAddress};
    use crate::parsers::attachments::attachments;
    use std::collections::HashMap;

    const LINE_TERMINATOR:&str = "\r\n";

    pub fn parse_mail(raw_mails: &Vec<Vec<u8>>) -> MailBox{
        raw_mails.iter().map(| mail | parse_line(mail)).collect::<Vec<Mail>>()
    }

    pub fn parse_line(raw_mail: &Vec<u8>) -> Mail{
        let mut i = 0 ;
        let ln = raw_mail.len();
        let mut buff = vec![];
        let mut field = false;
        let (mut header,mut body) = (vec![],vec![]);
        let mut continuation = false;
        while i < ln{
            // Parse a line
            if raw_mail[i] == b'\r' && (i+1 < ln) && raw_mail[i+1] == b'\n'{
                // Segreggation between header and body
                if buff.is_empty(){ 
                    field = true;
                }
                if i+2 < ln && raw_mail[i+2] == b' '{
                    continuation = true;
                }else{
                    continuation = false
                }

                if field{
                    if continuation{
                        *body.last_mut().unwrap() += &bytestr(&buff)[..];
                    }else{
                        body.push(bytestr(&buff).to_string());
                    }
                }else{
                    if continuation{
                        *header.last_mut().unwrap() += &bytestr(&buff)[..];
                    }else{
                        header.push(bytestr(&buff).to_string());
                    }
                }
                buff = vec![];
                i += 2;
            }else{
                buff.push(raw_mail[i]);
                i += 1;
            }

        }

        let header_str = header.join("\n");
        let body_str = body.join("\n");
        
        let mut mail = Mail {header: header_str.clone(),body: body_str.clone(),..Default::default()};

        let headers = parse_headers(&header_str);
        mail.headers = headers.clone();
        mail.from = parse_address_field(headers.get("from"));
        mail.to = parse_address_list(headers.get("to"));
        mail.cc = parse_address_list(headers.get("cc"));
        mail.bcc = parse_address_list(headers.get("bcc"));
        mail.subject = headers.get("subject").cloned();
        mail.date = headers.get("date").cloned();
        mail.message_id = headers.get("message-id").cloned();
        mail.in_reply_to = headers.get("in-reply-to").cloned();
        mail.references = parse_references(headers.get("references"));
        mail.content_type = headers.get("content-type").cloned();
        mail.is_multipart = mail.content_type.as_ref().map(|ct| ct.contains("multipart")).unwrap_or(false);
        mail.urls = extract_urls(&body_str);
        mail.attachments = attachments::parse_attachments(mail.content_type.as_deref().unwrap_or("text/plain"), body_str.as_bytes(), &headers);
        mail
    }

    fn parse_headers(header_str: &str) -> HashMap<String, String> {
        let mut headers: HashMap<String, String> = HashMap::new();
        for line in header_str.lines() {
            if let Some((key, value)) = line.split_once(':') {
                // Header keys are lowercase ONLY
                let k = key.trim().to_lowercase();
                let v = value.trim().to_string();
                if let Some(z) = headers.get_mut(&k) {
                    z.push_str(&v);
                } else {
                    headers.insert(k, v);
                }
            }
        }
        headers
    }

    fn parse_address_field(field: Option<&String>) -> Option<EmailAddress> {
        field.and_then(|f| parse_address_list(Some(f)).into_iter().next())
    }

    fn parse_address_list(field: Option<&String>) -> Vec<EmailAddress> {
        match field {
            Some(field) => {field.split(',').map(|addr| parse_address(addr.trim())).filter_map(|a| a).collect()}
            None => Vec::new()
        }
    }

    fn parse_address(addr: &str) -> Option<EmailAddress> {
        let addr = addr.trim();        
        if addr.is_empty() {
            return None;
        }
        // "Name <email@example.com>"
        if addr.contains('<') && addr.contains('>') {
            let (strt,ed) = (addr.find('<')? + 1,addr.find('>')?);
            let email = &addr[strt..ed];
            let name = addr[..strt-1].trim().trim_matches('"');
            Some(EmailAddress {name: if name.is_empty() { None } else { Some(name.to_string()) },address: email.to_string()})
        } else {
        // email@example.com

            Some(EmailAddress {name: None,address: addr.to_string()})
        }
    }

    fn parse_references(field: Option<&String>) -> Vec<String> {
        match field {
            Some(field) => {field.split_whitespace().map(|s| s.trim_matches('<').trim_matches('>').to_string()).filter(|s| !s.is_empty()).collect()}
            None => Vec::new()
        }
    }

    fn extract_urls(text: &str) -> Vec<String> {
        // Some complex regex copied from AI
        let rgx = regex::Regex::new(r"https?://\S+|www\.\S+").unwrap();        
        rgx.find_iter(text).map(|m| m.as_str().to_string()).collect()
    }

    pub fn bytestr(raw_line: &Vec<u8>) -> String{
        raw_line.iter().map(|x| *x as char).collect::<String>()
    }

    pub fn parse_mailbox_name(line: &str) -> Option<String> {
        if line.contains('"') {
            let parts: Vec<&str> = line.split('"').collect();
            if parts.len() >= 2 {
                return Some(parts[1].to_string());
            }
        }
        None
    }

    pub fn parse_mail_flags(line: &str) -> Vec<String> {
        line.split_whitespace().map(|s| s.trim_matches('\\').to_string()).filter(|s| !s.is_empty()).collect()
    }
}