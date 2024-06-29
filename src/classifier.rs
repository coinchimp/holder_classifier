use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct Holder {
    address: String,
    amount: String,
}

#[derive(Deserialize)]
struct Token {
    dec: String,
    holder: Vec<Holder>,
}

#[derive(Deserialize)]
struct ApiResponse {
    result: Vec<Token>,
}

#[derive(serde::Serialize)]
pub struct Classification {  // Made public
    crabs: usize,
    octopus: usize,
    fish: usize,
    dolphin: usize,
    shark: usize,
    whale: usize,
    humpback: usize,
    aquaman: usize,
    total_holders: usize,
}

pub async fn classify_holders(api_url: &str) -> Result<Classification, Box<dyn std::error::Error>> {
    let response = reqwest::get(api_url).await?.json::<ApiResponse>().await?;

    if response.result.is_empty() {
        return Err("Missing 'result' field in the response".into());
    }

    let token = &response.result[0];

    let dec = token.dec.parse::<f64>()?;

    let mut crabs = 0;
    let mut octopus = 0;
    let mut fish = 0;
    let mut dolphin = 0;
    let mut shark = 0;
    let mut whale = 0;
    let mut humpback = 0;
    let mut aquaman = 0;

    for holder in &token.holder {
        let amount = holder.amount.parse::<f64>()? / dec;
        if amount < 10_000.0 {
            crabs += 1;
        } else if amount < 100_000.0 {
            octopus += 1;
        } else if amount < 100_000_000.0 {
            fish += 1;
        } else if amount < 100_000_000_000.0 {
            dolphin += 1;
        } else if amount < 100_000_000_000_000.0 {
            shark += 1;
        } else if amount < 100_000_000_000_000_000.0 {
            whale += 1;
        } else if amount < 100_000_000_000_000_000_000.0 {
            humpback += 1;
        } else {
            aquaman += 1;
        }
    }

    let classification = Classification {
        crabs,
        octopus,
        fish,
        dolphin,
        shark,
        whale,
        humpback,
        aquaman,
        total_holders: token.holder.len(),
    };

    Ok(classification)
}
