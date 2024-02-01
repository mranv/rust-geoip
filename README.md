## Getting Started

An access token is required, which can be acquired by signing up for a free account
at [https://ipinfo.io/signup](https://ipinfo.io/signup).

The free plan is limited to 50,000 requests per month, and doesn't include some of the
data fields such as the IP type and company information. To get the complete list of
information on an IP address and make more requests per day see [https://ipinfo.io/pricing](https://ipinfo.io/pricing).

## Examples

There are several ready-to-run examples located in the `/examples` directory. These can be run directly, replacing `<token>` with your access token

```bash
cargo run --example lookup -- <token>
```

```bash
cargo run --example lookup_batch -- <token>
```

```bash
cargo run --example get_map
```

The `lookup` example above looks more or less like

```rust
use ipinfo::{IpInfo, IpInfoConfig};
#[tokio::main]
async fn main() {
    let config = IpInfoConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };

    let mut ipinfo = IpInfo::new(config)
        .expect("should construct");

    let res = ipinfo.lookup("8.8.8.8").await;
    match res {
        Ok(r) => {
            println!("{} lookup result: {:?}", "8.8.8.8", r);
        },
        Err(e) => println!("error occurred: {}", &e.to_string()),
    }
}
```

## Features

- Smart LRU cache for cost and quota savings.
- Structured and type-checked query results.
- Bulk IP address lookup using IPinfo [batch API](https://ipinfo.io/developers/batch).
- Locate IPs on a World Map.

#### Internationalization

When looking up an IP address, the `response` includes `country_name` which is the country name based on American English, `is_eu` which returns `true` if the country is a member of the European Union (EU), `country_flag` which includes the emoji and Unicode of a country's flag, `country_currency`
which includes the code and symbol of a country's currency, `country_flag_url` which returns a public link to the country's flag image as an SVG which can be used anywhere. and `continent` which includes the code and name of the continent.

```rust
let r = ipinfo.lookup("8.8.8.8");
println!("{}: {}", "8.8.8.8", r.country_name) // United States
println!("{}: {:?}", "8.8.8.8", r.is_eu) // Some(false)
println!("{}: {:?}", "8.8.8.8", r.country_flag) // Some(CountryFlag { emoji: "🇺🇸", unicode: "U+1F1FA U+1F1F8" })
println!("{}: {:?}", "8.8.8.8", r.country_flag_url) // Some(https://cdn.ipinfo.io/static/images/countries-flags/US.svg)
println!("{}: {:?}", "8.8.8.8", r.country_currency) // Some(CountryCurrency { code: "USD", symbol: "$" })
println!("{}: {:?}", "8.8.8.8", r.continent) // Some(Continent { code: "NA", name: "North America" })
```

It is possible to return the country name in other languages, change the EU countries and change the flag emoji or unicode by setting the `default_countries`, `default_eu`, `default_country_flags`, `default_currencies` and `default_continents` when creating the `IPinfo` client.

```rust
let countries = {
    let json_data = r#"
        {
            "US": "United States"
        }
    "#;
    serde_json::from_str(json_data).expect("error parsing user-defined JSON!")
};

let config = IpInfoConfig {
    default_countries: countries,
    ..Default::default()
};
```
