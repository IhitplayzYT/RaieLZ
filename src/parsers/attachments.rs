pub mod attachments{
    use crate::parsers::models::models::Attachment;
    use std::collections::HashMap;

    pub fn parse_attachments(content_type: &str, body: &[u8], headers: &HashMap<String, String>) -> Vec<Attachment> {
        let mut attachments = Vec::new();
        if content_type.contains("multipart/") {
            attachments = parse_multipart(body, content_type, headers);
        } else if is_attachment(content_type, headers) {
            if let Some(attachment) = parse_single_attachment(content_type, body, headers) {
                attachments.push(attachment);
            }
        }
        attachments
    }

    fn parse_multipart(body: &[u8], content_type: &str, headers: &HashMap<String, String>) -> Vec<Attachment> {
        let mut ret = Vec::new();
        if let Some(boundary) = extract_boundary(content_type) {
            for i in split_multipart(body, &boundary){
                if let Some((p_headers, p_body)) = parse_part_headers(&i) {
                    if let Some(ctype) = p_headers.get("content-type") {
                        if is_attachment(ctype, &p_headers) {
                            if let Some(attachment) = parse_single_attachment(ctype, &p_body, &p_headers) {
                                ret.push(attachment);
                            }
                        }
                    }
                }
            }
        }   
        ret
    }

    fn extract_boundary(raw: &str) -> Option<String> {
        raw.split(';').find_map(|x| {
                let x = x.trim();
                if x.to_lowercase().starts_with("boundary=") {
                    Some(x[9..].trim_matches('"').to_string())
                } else {
                    None
                }
            })
    }

    fn split_multipart(body: &[u8], boundary: &str) -> Vec<Vec<u8>> {
        let (ed,boundary) = (format!("--{}--", boundary).into_bytes(),format!("--{}", boundary).into_bytes());
        let mut ret = Vec::new();
        let (mut start,mut i) = (0,0);
        let (l,bl,edbl) = (body.len(),boundary.len(),ed.len());
        while i < l {
            if i + bl <= l && &body[i..i + bl] == boundary.as_slice() {
                if start > 0 {
                    ret.push(body[start..i].to_vec());
                }
                i += bl;
                start = i;
            } else if i + edbl <= l && &body[i..i + edbl] == ed.as_slice() {
                if start > 0 {
                    ret.push(body[start..i].to_vec());
                }
                break;
            } else {
                i += 1;
            }
        }
        ret
    }

    fn parse_part_headers(part: &[u8]) -> Option<(HashMap<String, String>, Vec<u8>)> {
        let mut headers = HashMap::new();
        let (mut ed,mut i) = (0,0);
        while i < part.len() {
            if i + 1 < part.len() && part[i] == b'\r' && part[i + 1] == b'\n' {
                if i + 3 < part.len() && part[i + 2] == b'\r' && part[i + 3] == b'\n' {
                    ed = i + 4;
                    break;
                }
                let header_line = String::from_utf8_lossy(&part[ed..i]).to_string();
                if let Some((key, value)) = header_line.split_once(':') {
                    headers.insert(key.trim().to_lowercase(), value.trim().to_string());
                }
                ed = i + 2;
                i += 2;
            } else {
                i += 1;
            }
        }
        

        if ed > 0 {
            Some((headers, part[ed..].to_vec()))
        } else {
            None
        }
    }

    fn is_attachment(content_type: &str, headers: &HashMap<String, String>) -> bool {
        let ctype = headers.get("content-disposition").map(|s| s.trim().to_lowercase()).unwrap_or_default();        
        ctype.contains("attachment") || ctype.contains("inline") || !content_type.starts_with("text/plain") && !content_type.starts_with("text/html")
    }

    fn parse_single_attachment(content_type: &str, body: &[u8], headers: &HashMap<String, String>) -> Option<Attachment> {
        Some(Attachment {filename: extract_filename(headers).unwrap_or_else(|| "unnamed".to_string()),content_type: content_type.to_string(),size: body.len(),content_id:headers.get("content-id").map(|s| s.trim_matches('<').trim_matches('>').to_string()),disposition:headers.get("content-disposition").cloned(),data: Some(body.to_vec())})
    }

    fn extract_filename(headers: &HashMap<String, String>) -> Option<String> {
        if let Some(ctype) = headers.get("content-disposition") {
            if let Some(fname) = extract_param(ctype, "filename") {
                return Some(fname);
            }
        }
        if let Some(ctype) = headers.get("content-type") {
            if let Some(filename) = extract_param(ctype, "name") {
                return Some(filename);
            }
        }
        None
    }

    fn extract_param(header_value: &str, param_name: &str) -> Option<String> {
        header_value.split(';').find_map(|param| {
            let param = param.trim();
            if param.to_lowercase().starts_with(&format!("{}=", param_name)) {
                Some(param[param_name.len() + 1..].trim_matches('"').to_string())
            } else {
                None
            }
        })
    }

    pub fn is_mulitmedia(content_type: &str) -> bool {
        content_type.starts_with("image/") || content_type.starts_with("video/") || content_type.starts_with("audio/")
    }

    pub fn get_attachment_extension(fname: &str) -> Option<&str> {
        fname.rsplit('.').next()
    }
}