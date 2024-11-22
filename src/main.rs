use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;
use dotenv::dotenv;
use std::env;

// Struct to represent Bitcoin pricing data
#[derive(Debug, Deserialize)]
pub struct Bitcoin {}

// Struct to represent Ethereum pricing data
#[derive(Debug, Deserialize)]
pub struct Ethereum {}

// Struct to represent S&P 500 pricing data
#[derive(Debug, Deserialize)]
pub struct SP500 {}

// Pricing trait to define common behavior
trait Pricing {
    fn fetch_price(&self) -> Result<f64, String>; // Fetches the latest price
    fn save_to_file(&self, price: f64) -> Result<(), String>; // Saves price to a file
}

// Implement Pricing trait for Bitcoin
impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<f64, String> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        let response = ureq::get(url).call().map_err(|e| e.to_string())?;
    
        let json: serde_json::Value = serde_json::from_reader(response.into_reader())
            .map_err(|e| e.to_string())?;
    
        json["bitcoin"]["usd"]
            .as_f64()
            .ok_or_else(|| "Failed to parse Bitcoin price".to_string())
    }

    fn save_to_file(&self, price: f64) -> Result<(), String> {
        let path = "bitcoin_prices.txt";
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .map_err(|e| e.to_string())?;
        writeln!(
            file,
            "Bitcoin Price: {:.2}, Timestamp: {}",
            price,
            chrono::Utc::now()
        )
        .map_err(|e| e.to_string())
    }
}

// Implement Pricing trait for Ethereum
impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<f64, String> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
    let response = ureq::get(url).call().map_err(|e| e.to_string())?;

    let json: serde_json::Value = serde_json::from_reader(response.into_reader())
        .map_err(|e| e.to_string())?;

    json["ethereum"]["usd"]
        .as_f64()
        .ok_or_else(|| "Failed to parse Ethereum price".to_string())
}

    fn save_to_file(&self, price: f64) -> Result<(), String> {
        let path = "ethereum_prices.txt";
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .map_err(|e| e.to_string())?;
        writeln!(
            file,
            "Ethereum Price: {:.2}, Timestamp: {}",
            price,
            chrono::Utc::now()
        )
        .map_err(|e| e.to_string())
    }
}

// Implement Pricing trait for SP500
impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<f64, String> {
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d";
        let response = ureq::get(url).call().map_err(|e| e.to_string())?;
    
        let json: serde_json::Value = serde_json::from_reader(response.into_reader())
            .map_err(|e| e.to_string())?;
    
        let close_prices = json["chart"]["result"][0]["indicators"]["quote"][0]["close"]
            .as_array()
            .ok_or_else(|| "Failed to parse closing prices".to_string())?;
    
        close_prices
            .last()
            .and_then(|val| val.as_f64())
            .ok_or_else(|| "Failed to get the latest S&P 500 price".to_string())
    }

    fn save_to_file(&self, price: f64) -> Result<(), String> {
        let path = "sp500_prices.txt";
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .map_err(|e| e.to_string())?;
        writeln!(
            file,
            "S&P 500 Price: {:.2}, Timestamp: {}",
            price,
            chrono::Utc::now()
        )
        .map_err(|e| e.to_string())
    }
}

fn main() {

    dotenv().ok(); // Load environment variables
    let api_key = env::var("API_KEY").expect("API_KEY not set in .env file");
    println!("Your API Key: {}", api_key);

    // Create instances of each asset
    let assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin {}),
        Box::new(Ethereum {}),
        Box::new(SP500 {}),
    ];

    // Periodic data fetching loop
    loop {
        for asset in &assets {
            match asset.fetch_price() {
                Ok(price) => {
                    if let Err(e) = asset.save_to_file(price) {
                        eprintln!("Error saving price: {}", e);
                    }
                }
                Err(e) => eprintln!("Error fetching price: {}", e),
            }
        }
        thread::sleep(Duration::from_secs(10));
    }
}
