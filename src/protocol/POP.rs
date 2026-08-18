pub mod pop{

use std::error::Error;

use pop3_mail_client::{Pop3Client, Pop3ClientBuilder, Pop3ClientBuilderConnect, Pop3Connection};

use crate::config::Config::EmailAccount;


    pub async fn get_mails(acc: &EmailAccount) -> Result<Vec<Vec<u8>>,Box<dyn Error>>{
        let clnt = Pop3Client::builder();
        let clnt = clnt.username(&acc.address);
        let clnt = clnt.password(&acc.get_pass());
        let mut clnt = clnt.connect(Pop3Connection::new(&acc.pop3_server,acc.pop3_port))?;
        let messages = clnt.list()?;

        let mut ret = vec![];
        for m in messages.messages{
            let mut temp = vec![];
            clnt.retrieve(m.message_id,&mut temp)?;
            ret.push(temp);
        }
        Ok(ret)
    }

}