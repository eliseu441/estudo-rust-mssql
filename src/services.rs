
use tiberius::ColumnData;
use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use serde_json::json;
use actix_web::{
    web::{
        scope,
        ServiceConfig
    },
    get,
    Responder,
    HttpResponse,
};


pub async fn connection()-> anyhow::Result<Vec<String>> {
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
    .into_first_result()
    .await?;
    
    let mut data:String = String::new();
    //get column names
    let col_names: Vec<&str> = row.first().unwrap().columns().iter().map(|c| c.name()).collect();
    let mut reponse = Vec::new();
    println!("{:?}", col_names);
    for row in row {
        let mut rowtext: Vec<String> = Vec::new();
        for item in row.into_iter() {
            let output = match item {
                ColumnData::Binary(_val) => "binary data".into(),
                ColumnData::Bit(val) => val.unwrap_or_default().to_string(),
                ColumnData::Date(_val) => "don't know how to implement".into(),
                ColumnData::DateTime(_val) => "don't know how to implement".into(),
                ColumnData::DateTime2(_val) => "don't know how to implement".into(),
                ColumnData::DateTimeOffset(_val) => "don't know how to implement".into(),
                ColumnData::F32(val) => val.unwrap_or_default().to_string(),
                ColumnData::F64(val) => val.unwrap_or_default().to_string(),
                ColumnData::Guid(val) => val.unwrap_or_default().to_string(),
                ColumnData::I16(val) => val.unwrap_or_default().to_string(),
                ColumnData::I32(val) => val.unwrap_or_default().to_string(),
                ColumnData::I64(val) => val.unwrap_or_default().to_string(),
                ColumnData::Numeric(val) => val.unwrap().to_string(),
                ColumnData::SmallDateTime(_val) => "don't know how to implement".into(),
                ColumnData::String(val) => val.unwrap_or_default().as_ref().into(),
                ColumnData::Time(_val) => "don't know how to implement".into(),
                ColumnData::U8(val) => val.unwrap_or_default().to_string(),
                ColumnData::Xml(val) => val.unwrap().as_ref().to_string(),
                _ => "nada".into()
            };
            rowtext.push(output);
        }
         data = format!("{}", rowtext.join(","));
         reponse.push(data.clone());
    }
    println!("{:?}", reponse);

    Ok(reponse)

}



#[get("/getStock")]
async fn get_stock() -> impl Responder {

    match  connection().await{
        Ok(reponse) => {
           return HttpResponse::Ok().json(reponse);
        }
        Err(error) => {

            return HttpResponse::InternalServerError().json(
                json!({
                    "status": "error",
                    "message": format!("{:?}", error)
                })
            )
        }
    };
   // println!("{:?}", query);
}


pub fn config(conf:  &mut ServiceConfig) {
    let scope = scope("/api")
                .service(get_stock);



    conf.service(scope);
}