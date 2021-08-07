fn main() {
    println!("Hello, world!");
}

fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
    let mut hm = std::collections::HashMap::<String, i32>::new();
    let mut res = std::vec::Vec::<String>::new();
    for s in cpdomains.iter() {
        let s: Vec<&str> = s.split(" ").collect();
        let num = s[0].parse::<i32>().unwrap();
        for (i, subdo) in s[1].rsplit(".").enumerate() {
            let mut to_rest: Vec<&str> = s[1].rsplit(".").collect::<Vec<&str>>().to_vec();
            to_rest.reverse();
            let to_rest = to_rest.join(".").to_string();
            match hm.clone().get(subdo) {
                Some(count) => hm.insert(to_rest, count+num),
                None => hm.insert(to_rest, num)
            };
        }
    }
    for (key, val) in hm.iter() {
        res.push(format!("{:?} {}", val, key).to_string());
    }
    res
}

#[test]
fn test_subdomain_vists() {
    let v = vec!["900 google.mail.com".to_string(), "50 yahoo.com".to_string(), "1 intel.mail.com".to_string(), "5 wiki.org".to_string()];
    let res = vec!["901 mail.com","50 yahoo.com","900 google.mail.com","5 wiki.org","5 org","1 intel.mail.com","951 com"];
    // subdomain_visits(v);
    assert_eq!(subdomain_visits(v), res);
}
