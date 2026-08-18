pub mod Helper{
    use std::process::exit;



    const DBG_STR: &str = "";
    
    const DB_URL: &str = "";
    const DB_USERNAME: &str = "root";
    const OK:i32 = 0;
    const ERR:i32 = -1;


    #[derive(Debug,Clone)]
    pub struct CLI{
        pub dbg: bool,
        pub mail: Option<String>,
        pub pass: Option<String>, 
        pub db_url: Option<String>,
        pub db_user: Option<String>,
        pub db_pass: Option<String>,   
        pub db_port: Option<u16>,
        pub db: Option<String>,
        pub username: Option<String>,
        pub smtp_srvr: Option<String>,
        pub imap_srvr: Option<String>,
        pub pop3_srvr: Option<String>,
        pub pop3_port: Option<u16>,
        pub imap_port: Option<u16>,
        pub smtp_port: Option<u16>,
        pub config: Option<String>
    }


    pub fn Help(){
        println!("{DBG_STR}");
        exit(OK);
    }


    impl CLI{
        pub fn new() -> Self{
            Self {dbg: false,mail:None,db_pass:None,pass:None,db_url:None,db_user:None,db:None,db_port: None,username:None,smtp_srvr:None,imap_srvr:None,smtp_port:None,imap_port:None,config:None,pop3_port:None,pop3_srvr:None}
        }

        pub fn Parse_Args(&mut self){
            let args: Vec<String> = std::env::args().skip(1).collect();
            self.mail = if let Ok(x) = std::env::var("MAIL"){Some(x)}else{None}; 
            self.pass = if let Ok(x) = std::env::var("PASS"){Some(x)}else{None}; 
            self.db = if let Ok(x) = std::env::var("DB"){Some(x)}else{None}; 
            self.db_user = if let Ok(x) = std::env::var("DB_USER"){Some(x)}else{None}; 
            self.db_pass = if let Ok(x) = std::env::var("DB_PASS"){Some(x)}else{None}; 
            self.db_url = if let Ok(x) = std::env::var("DB_URL"){Some(x)}else{None}; 
            self.db_port = if let Ok(x) = std::env::var("DB_PORT"){Some(x.parse::<u16>().expect("Port is an Unsigned 16 bit integer"))}else{None}; 
            self.username = if let Ok(x) = std::env::var("USERNAME"){Some(x)}else{None}; 
            self.smtp_srvr = if let Ok(x) = std::env::var("SMTP_SRVR"){Some(x)}else{None}; 
            self.imap_srvr = if let Ok(x) = std::env::var("IMAP_SRVR"){Some(x)}else{None}; 
            self.pop3_srvr = if let Ok(x) = std::env::var("POP3_SRVR"){Some(x)}else{None}; 
 
            self.smtp_port = if let Ok(x) = std::env::var("SMTP_PORT"){Some(x.parse().expect("Port is an Unsigned 16 bit integer"))}else{None}; 
            self.imap_port = if let Ok(x) = std::env::var("IMAP_PORT"){Some(x.parse().expect("Port is an Unsigned 16 bit integer"))}else{None}; 
            self.pop3_port = if let Ok(x) = std::env::var("POP3_PORT"){Some(x.parse().expect("Port is an Unsigned 16 bit integer"))}else{None}; 


            if self.mail.is_none(){
                self.mail = if let Ok(x) = std::env::var("EMAIL"){Some(x)}else{None}; 
            }

            if self.smtp_srvr.is_none(){
                self.smtp_srvr = if let Ok(x) = std::env::var("SMTP_SERVER"){Some(x)}else{None}; 
            }

            if self.smtp_srvr.is_none(){
                self.smtp_srvr = if let Ok(x) = std::env::var("SMTP"){Some(x)}else{None}; 
            }



            if self.imap_srvr.is_none(){
                self.imap_srvr = if let Ok(x) = std::env::var("IMAP_SERVER"){Some(x)}else{None}; 
            }

            if self.imap_srvr.is_none(){
                self.imap_srvr = if let Ok(x) = std::env::var("IMAP"){Some(x)}else{None}; 
            }           


            if self.pop3_srvr.is_none(){
                self.pop3_srvr = if let Ok(x) = std::env::var("POP3_SERVER"){Some(x)}else{None}; 
            }

            if self.pop3_srvr.is_none(){
                self.pop3_srvr = if let Ok(x) = std::env::var("POP3"){Some(x)}else{None}; 
            }                      

            if self.db_port.is_none(){
                self.db_port = if let Ok(x) = std::env::var("PORT"){Some(x.parse::<u16>().expect("Port is an Unsigned 16 bit integer"))}else{None}; 
            }

            if self.pass.is_none(){
                self.pass = if let Ok(x) = std::env::var("PASSWORD"){Some(x)}else{None}; 
            }
            if self.pass.is_none(){
                self.pass = if let Ok(x) = std::env::var("PASWD"){Some(x)}else{None}; 
            }
            if self.pass.is_none(){
                self.pass = if let Ok(x) = std::env::var("PASSWD"){Some(x)}else{None}; 
            }

            if self.db.is_none(){
                self.db = if let Ok(x) = std::env::var("DATABASE"){Some(x)}else{None}; 
            }
            if self.db_user.is_none(){
                self.db_user = if let Ok(x) = std::env::var("DATABASE_USER"){Some(x)}else{None}; 
            }
            
            if self.db_pass.is_none(){
                self.db_pass = if let Ok(x) = std::env::var("DATABASE_PASS"){Some(x)}else{None}; 
            }
            if self.db_pass.is_none(){
                self.db_pass = if let Ok(x) = std::env::var("DATABASE_PASSWORD"){Some(x)}else{None}; 
            }

            if self.db_url.is_none(){
                self.db_url = if let Ok(x) = std::env::var("DATBASE_URL"){Some(x)}else{None};         
            }
            
            if self.db_url.is_none(){
                self.db_url = if let Ok(x) = std::env::var("URL"){Some(x)}else{None}; 
            }

            for i in &args{
                if i == "-d" || i == "--debug" || i == " --DEBUG" || i == "-D"{
                    self.dbg = true;
                } else if i == "-h" || i == "--help" || i == " --HELP" || i == "-H"{
                    Help();
                } else if i.starts_with("--mail=") || i.starts_with("-m="){
                    self.mail = Some(i[i.find("=").unwrap()+1..].to_string());
                } else if i.starts_with("--pass=") || i.starts_with("-p="){
                    self.pass = Some(i[i.find("=").unwrap()+1..].to_string());
                } else if i.starts_with("--db_pass=") || i.starts_with("-db_p="){
                    self.db_pass = Some(i[i.find("=").unwrap()+1..].to_string());
                } else if i.starts_with("--db_url="){
                    self.db_url = Some(i[i.find("=").unwrap()+1..].to_string());
                } else if i.starts_with("--db_user=") || i.starts_with("-db_u="){
                    self.db_user = Some(i[i.find("=").unwrap()+1..].to_string());
                } else if i.starts_with("--db=") || i.starts_with("-db="){
                    self.db = Some(i[i.find("=").unwrap()+1..].to_string());
                } else if i.starts_with("--db_port=") || i.starts_with("-db_p="){
                    self.db_port = Some(i[i.find("=").unwrap()+1..].parse::<u16>().expect("Port is an Unsigned 16 bit integer"));
                }  else if i.starts_with("--username=") || i.starts_with("-u="){
                    self.username = Some(i[i.find("=").unwrap()+1..].to_string());
                } else if i.starts_with("--smtp_srvr=") || i.starts_with("-smpt="){
                    self.smtp_srvr = Some(i[i.find("=").unwrap()+1..].to_string());
                } else if i.starts_with("--imap_srvr=") || i.starts_with("-imap="){
                    self.imap_srvr = Some(i[i.find("=").unwrap()+1..].to_string());
                } else if i.starts_with("--config=") || i.starts_with("-conf="){
                    self.config = Some(i[i.find("=").unwrap()+1..].to_string());
                } else{
                    Help();
                }
            } 

        if self.username.is_none(){
            self.username = self.mail.clone();
        }

        if self.imap_port.is_none(){
            self.imap_port = Some(143);
        }

        if self.smtp_port.is_none(){
            self.smtp_port = Some(25);
        }

        }



    }


    





}