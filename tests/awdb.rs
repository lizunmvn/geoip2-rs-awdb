#[cfg(test)]
mod tests {
    use geoip2_awdb::{ Reader, AwdbCity };
    use std::{ net::IpAddr, str::FromStr };

    #[test]
    fn test_metadata() {
        let buffer = std::fs::read("./testdata/IP_basic_single_WGS84.awdb").unwrap();
        let reader = Reader::<AwdbCity>::from_bytes_awdb(&buffer).unwrap();

        let metadata = reader.get_metadata();
        {
            println!("Metadata:");
            println!("\tBinary Format Major Version: {}", metadata.binary_format_major_version);
            println!("\tBinary Format Minor Version: {}", metadata.binary_format_minor_version);
            println!("\tNode Count: {}", metadata.node_count);
            println!("\tRecord Size: {}", metadata.record_size);
            println!("\tIP Version: {}", metadata.ip_version);
            println!("\tDatabase Type: {}", metadata.database_type);
            println!("\tLanguages: {:?}", metadata.languages);
            println!("\tBuild Epoch: {}", metadata.build_epoch);
            println!("\tDescription:");
        }
    }

    #[test]
    fn test_awdb_city() {
        let buffer = std::fs::read("./testdata/IP_basic_single_WGS84.awdb").unwrap();
        let reader = Reader::<AwdbCity>::from_bytes_awdb(&buffer).unwrap();

        // Generate a random IP address
        let rand_ip = "125.78.97.58".to_string();

        // Print the generated random IP address
        println!("Generated random IP address: {}", rand_ip);

        // Print result
        let result = reader
            .lookup(IpAddr::from_str(&rand_ip).unwrap())
            .unwrap();
        {
            println!("AwdbCity:");
            println!("\tContinent: {:?}", result.continent);
            println!("\tOwner: {:?}", result.owner);
            println!("\tCountry: {:?}", result.country);
            println!("\tAdcode: {:?}", result.adcode);
            println!("\tCity: {:?}", result.city);
            println!("\tTimezone: {:?}", result.timezone);
            println!("\tISP: {:?}", result.isp);
            println!("\tAccuracy: {:?}", result.accuracy);
            println!("\tSource: {:?}", result.source);
            println!("\tASNumber: {:?}", result.asnumber);
            println!("\tAreaCode: {:?}", result.areacode);
            println!("\tZipCode: {:?}", result.zipcode);
            println!("\tLngWGS: {:?}", result.lngwgs);
            println!("\tProvince: {:?}", result.province);
            println!("\tLatWGS: {:?}", result.latwgs);
            println!("\tRadius: {:?}", result.radius);
            println!("\tDistrict: {:?}", result.district);
        }
    }
}


