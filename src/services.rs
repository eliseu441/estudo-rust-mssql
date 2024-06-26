
use std::fs::File;
use std::io::Read;
use serde_json::Value;
use tiberius::Row;
use tiberius::{Client, Config, Query, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use serde_json::json;
use actix_web::{
    web::{
        scope,
        Json,
        Path,
        Data,
        ServiceConfig
        
    },
    Error,
    get,
    post,
    HttpResponse,
    Responder,
};


pub async fn connection()-> anyhow::Result<Vec<Vec<Row>>> {
    let mut config = Config::new();

    config.host("163.123.183.80");
    config.port(18501);
    config.authentication(AuthMethod::sql_server("eliseu441", "Trembolona550"));
    config.trust_cert(); 
    config.instance_name("HEFESTO");

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    
    let mut client = Client::connect(config, tcp.compat_write()).await?;
    let row = client
    .query("SELECT * FROM [HEFESTO].[dbo].[TBF_GENERAL_STOCK];", &[])
    .await?
    .into_results()
    .await?;


    Ok(row)
}



#[get("/getStock")]
async fn get_stock() -> impl Responder {

    let response = "teste";
    let query = connection().await;
   // println!("{:?}", query);
    


    format!("{:?}", query)
}


pub fn config(conf:  &mut ServiceConfig) {
    let scope = scope("/api")
                .service(get_stock);



    conf.service(scope);
}