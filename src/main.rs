use clap::{Parser, Subcommand};
use prettytable::{Cell, Row, Table};
use reqwest::Method;
use serde::Deserialize;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long)]
    option: Option<String>,
}
#[derive(Deserialize, Debug)]
struct Ticker {
    symbol: String,
    price: String,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// Get lists price crypto
    List {
        symbol: Option<String>,
    },
    /// Create a order
    Create,
    /// Delete a order
    Delete {
        id: i64,
    },
    Read {
        id: i64,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();
    println!("{:#?}", cli);

    match cli.command {
        Commands::List { symbol } => {
            request("/v1".to_string(), reqwest::Method::GET, symbol).await?;
        }
        Commands::Create => {
            todo!()
        }
        _ => {}
    }

    Ok(())
}
fn format_symbols(symbols: &str) -> String {
    // Split the symbols by commas, quote each symbol, and join them with commas
    let formatted = symbols
        .split(',')
        .map(|s| format!("\"{}\"", s)) // Quote each symbol
        .collect::<Vec<String>>()
        .join(","); // Join symbols with commas

    // Wrap the entire result in square brackets
    format!("[{}]", formatted)
}
async fn request(
    url: String,
    method: Method,
    body: Option<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let symbols = body.unwrap_or_else(|| "BTCUSDT,ETHUSDT,BNBUSDT".to_string());
    let url = format!(
        "https://api.binance.com/api/v3/ticker/price?symbols={}",
        format_symbols(&symbols)
    );
    let a = 1;
    let client = reqwest::Client::new();
    let mut request_builder = client.request(method.clone(), &url);

    let res = request_builder.send().await?;
    //eprintln!("Response: {:?} {}", res.version(), res.status());
    //eprintln!("Headers: {:#?}\n", res.headers());

    let response = res.json::<Vec<Ticker>>().await?;
    //let response_text = res.text().await?;
    //let ticker: Vec<Ticker> = serde_json::from_str(&response_text)?;
    //let ticker: Ticker = serde_json::from_str(&response_text)?;
    // Print the response in a table format using `prettytable`
    let mut table = Table::new();

    // Add headers to the table
    table.add_row(Row::new(vec![Cell::new("Symbol"), Cell::new("Price")]));

    // Add data rows to the table
    for ticker in response {
        table.add_row(Row::new(vec![
            Cell::new(&ticker.symbol),
            Cell::new(&ticker.price),
        ]));
    }

    // Print the table
    table.printstd();
    Ok(())
}
