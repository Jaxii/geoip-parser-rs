use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let urls = vec![
        "https://ftp.arin.net/pub/stats/arin/delegated-arin-extended-latest",
        "https://ftp.apnic.net/stats/apnic/delegated-apnic-extended-latest",
        "https://ftp.ripe.net/ripe/stats/delegated-ripencc-extended-latest",
        "https://ftp.apnic.net/stats/afrinic/delegated-afrinic-extended-latest",
        "https://ftp.apnic.net/stats/lacnic/delegated-lacnic-extended-latest",
    ];

    for url in urls {

        let test = get_ips(url).await?;

        for thing in test {
            println!("{:?}", thing)
        }
    }

    Ok(())
}

async fn get_ips(url: &str) -> Result<Vec<IPAddressAllocation>, Error> {

    let response = reqwest::get(url).await?.text().await?;

    let data = parse_data(&response);
    
    let allocations: Vec<IPAddressAllocation> = data.iter()
        .filter_map(|line| match IPAddressAllocation::from_line(line) {
            Ok(allocation) => Some(allocation),
            Err(_) => None,
        })
        .collect();

    Ok(allocations)
}

#[derive(Debug)]
struct IPAddressAllocation {
    registry: String,
    country_code: String,
    ip_version: String,
    ip_address: String,
    block_size: u32,
    date: String,
    status: String,
    //unique_id: u32, //some registries keep hexadecimal unique_id instead of u32
}


impl IPAddressAllocation {
    fn from_line(line: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 8 {
            return Err("Invalid line format");
        }

        Ok(Self {
            registry: parts[0].to_string(),
            country_code: parts[1].to_string(),
            ip_version: parts[2].to_string(),
            ip_address: parts[3].to_string(),
            block_size: parts[4].parse().map_err(|_| "Invalid block size")?,
            date: parts[5].to_string(),
            status: parts[6].to_string(),
            //unique_id: parts[7].parse().map_err(|_| "Invalid unique ID")?,
        })
    }
}

fn parse_data(data: &str) -> Vec<&str> {
    data.lines().collect()
}

#[test]
fn test_parser() {

    let data = [
        "lacnic|BR|ipv4|24.152.8.0|1024|20200309|allocated|301675",
        "lacnic|BR|ipv4|24.152.72.0|1024|20200311|allocated|274578",
        "afrinic|ZA|asn|1230|1|19910301|allocated|F36B9F4B",
        "afrinic|ZA|asn|1231|1|19910301|allocated|F36B9F4B",
        "afrinic|ZA|asn|1232|1|19910301|allocated|F36B9F4B",
        "afrinic|ZA|asn|2018|1|20010307|allocated|F36B9F4B",
        "afrinic|EG|asn|2561|1|20070920|allocated|F3648BE1",
        "afrinic|ZA|asn|2905|1|19930910|allocated|F367678F"
    ];

    for line in data.iter() {
        match IPAddressAllocation::from_line(line) {
            Ok(allocation) => println!("{:#?}", allocation),
            Err(e) => println!("Error parsing line: {}", e),
        }
    }
}
