use tiberius::{Client, Config, Query, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut config = Config::new();

    config.host("163.123.183.80");
    config.port(18501);
    config.authentication(AuthMethod::sql_server("eliseu441", "Trembolona550"));
    config.trust_cert(); // on production, it is not a good idea to do this
    config.instance_name("HEFESTO");

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    // To be able to use Tokio's tcp, we're using the `compat_write` from
    // the `TokioAsyncWriteCompatExt` to get a stream compatible with the
    // traits from the `futures` crate.
    let mut client = Client::connect(config, tcp.compat_write()).await?;
    
    let row = client
    .query("SELECT * FROM [HEFESTO].[dbo].[TBF_GENERAL_STOCK];", &[])
    .await?
    .into_results()
    .await?;
    //assert_eq!("ITEM", row.get("ITEM"));
    
    println!("{:?}", row);
    /*OR
    let _res = client.query("SELECT ID FROM TBF_GENERAL_STOCK WHERE ID = @TEST", &[&String]).await?;
     */
    Ok(())
}