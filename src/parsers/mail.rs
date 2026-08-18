pub mod mail{
    use std::vec;



use crate::parsers::models::models::{Mail, Placeholder};

    const LINE_TERMINATOR:&str = "\r\n";



    pub fn parse_mail(raw_mails: &Vec<Vec<u8>>) -> Placeholder{

        

        raw_mails.iter().map(| mail | parse_line(mail)).collect::<Vec<&str>>().join("\n");



        Placeholder
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
                        *body.last_mut().unwrap() += parse_bytes(&buff);
                    }else{
                        body.push(parse_bytes(&buff).to_string());
                    }
                }else{
                    if continuation{
                        *header.last_mut().unwrap() += parse_bytes(&buff);
                    }else{
                        header.push(parse_bytes(&buff).to_string());
                    }
                }
                buff = vec![];
                i += 2;
            }else{
                buff.push(raw_mail[i]);
                i += 1;
            }

        }

       Mail { header: header.join("\n"), body: body.join("\n")} 
    }


    pub fn parse_bytes(raw_line: &Vec<u8>) -> &str{
        


        ""        
    }




}