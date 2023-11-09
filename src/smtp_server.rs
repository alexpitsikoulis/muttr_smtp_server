use samotop::{
    mail, smtp,
    server::TcpServer, 
};

pub struct SmtpServer();

impl SmtpServer {
    pub async fn start(port: u16) -> Result<(), Box<dyn std::error::Error + std::marker::Send + Sync>> {
        let mail = mail::Builder
            + smtp::Esmtp.with(smtp::SmtpParser);
    
        TcpServer::on(format!("localhost:{}", port)).serve(mail.build()).await
    }
}