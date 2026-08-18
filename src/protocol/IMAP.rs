pub mod imap{
    use std::error::Error;

use async_imap::Session;
use futures_util::TryStreamExt;
use tokio_native_tls::TlsConnector;

use crate::config::Config::EmailAccount;


    pub async fn connect(acc: &EmailAccount) -> Result<Session<tokio_native_tls::TlsStream<tokio::net::TcpStream>>,Box<dyn Error>>{
        let tcp = tokio::net::TcpStream::connect((acc.imap_server.as_str(),acc.imap_port)).await?;
        let connector = tokio_native_tls::native_tls::TlsConnector::new()?;
        let connector = TlsConnector::from(connector);
        let tls_stream = connector.connect(&acc.imap_server, tcp).await?;
        let mut client = async_imap::Client::new(tls_stream);
        let _ = client.read_response().await?;
        let session = client.login(&acc.username,acc.get_pass()).await.map_err(|e| e.0)?;
        Ok(session)
    }


    pub async fn get_mails(acc: &EmailAccount) -> Result<Vec<Vec<u8>>,Box<dyn Error>>{
        let mut sess = connect(acc).await?;
        sess.select("INBOX").await?;
        let messages = sess.fetch("1.*", "RFC822").await?;
        let messages: Vec<_> = messages.try_collect().await?;
        let mut ret = vec![];
        messages.iter().for_each(|x| {
            if let Some(y) = x.body(){
                ret.push(y.to_vec());
            }
        });
        Ok(ret)
    }

}












