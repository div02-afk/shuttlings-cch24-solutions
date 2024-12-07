use rocket::get;

pub fn expand_ipv6(from_ip: &mut Vec<&str>) {
    if let Some(pos) = from_ip.iter().position(|&x| x == "") {
        let missing_zeros = 8 - from_ip.len() + 1;
        from_ip.splice(pos..=pos, vec!["0"; missing_zeros]);
    }
}
pub fn shorten_ipv6(full_ipv6: &str) -> String {
    let segments: Vec<&str> = full_ipv6.split(':').collect();

    let cleaned_segments: Vec<String> = segments
        .iter()
        .map(|seg| seg.trim_start_matches('0').to_string())
        .map(|seg| if seg.is_empty() { "0".to_string() } else { seg }) // Convert empty segments to "0"
        .collect();

    let compressed = cleaned_segments.join(":");
    let mut longest_zero_seq = (0, 0);

    let mut current_start = None;
    let mut current_length = 0;

    for (i, segment) in cleaned_segments.iter().enumerate() {
        if segment == "0" {
            if current_start.is_none() {
                current_start = Some(i);
            }
            current_length += 1;
        } else if let Some(start) = current_start {
            if current_length > longest_zero_seq.1 {
                longest_zero_seq = (start, current_length);
            }
            current_start = None;
            current_length = 0;
        }
    }

    // Check the last sequence if it was the longest
    if let Some(start) = current_start {
        if current_length > longest_zero_seq.1 {
            longest_zero_seq = (start, current_length);
        }
    }

    // Replace the longest zero sequence with "::"
    if longest_zero_seq.1 > 1 {
        let (start, length) = longest_zero_seq;
        let mut result = Vec::new();

        for i in 0..start {
            result.push(cleaned_segments[i].clone());
        }

        result.push("".to_string()); // Insert "::" here

        for i in start + length..8 {
            result.push(cleaned_segments[i].clone());
        }

        return result.join(":").replace(":::", "::");
    }

    compressed
}

#[get("/2/dest?<from>&<key>")]
pub fn two_dest_one(from: &str, key: &str) -> String {
    let from_ip = from.split(".").collect::<Vec<&str>>();
    let key_ip = key.split(".").collect::<Vec<&str>>();
    let mut result = "".to_string();
    for i in 0..4 {
        let from_num = from_ip[i].parse::<u16>().unwrap();
        let key_num = key_ip[i].parse::<u16>().unwrap();
        let mut add: u16 = from_num + key_num;
        if add > 255 {
            add = add - 256;
        }
        result.push_str(&add.to_string());
        result.push_str(".");
    }
    result.pop();
    result
}

#[get("/2/key?<from>&<to>")]
pub fn two_dest_two(from: &str, to: &str) -> String {
    let from_ip = from.split(".").collect::<Vec<&str>>();
    let to_ip = to.split(".").collect::<Vec<&str>>();
    let mut result = "".to_string();
    for i in 0..4 {
        let from_num = from_ip[i].parse::<u16>().unwrap();
        let to_num = to_ip[i].parse::<u16>().unwrap();
        let ans;
        if from_num <= to_num {
            ans = to_num - from_num;
        } else {
            ans = 256 - from_num + to_num;
        }
        result.push_str(&ans.to_string());
        result.push_str(".");
    }
    println!("{}", result);
    result.pop();
    result
}

#[get("/2/v6/key?<from>&<to>")]
pub fn two_dest_three_two(from: &str, to: &str) -> String {
    let mut from_ip = from.split(":").collect::<Vec<&str>>();
    let mut key_ip = to.split(":").collect::<Vec<&str>>();

    expand_ipv6(&mut from_ip);
    expand_ipv6(&mut key_ip);
    println!("{:?},{:?}", from_ip, key_ip);
    let mut result = "".to_string();
    for i in 0..8 {
        let from_num = u32::from_str_radix(from_ip[i], 16).unwrap();
        let to_num = u32::from_str_radix(key_ip[i], 16).unwrap();
        let ans = from_num ^ to_num;
        // if from_num <= to_num {
        //     ans = to_num - from_num;
        // } else {
        //     ans = 65536 - from_num + to_num;
        // }
        let ans_hex = format!("{:04x}", ans);
        result.push_str(&ans_hex.to_string());
        result.push_str(":");
    }
    result.pop();
    shorten_ipv6(&result)
}

#[get("/2/v6/dest?<from>&<key>")]
pub fn two_dest_three_one(from: &str, key: &str) -> String {
    let mut from_ip = from.split(":").collect::<Vec<&str>>();
    let mut to_ip = key.split(":").collect::<Vec<&str>>();

    expand_ipv6(&mut from_ip);
    expand_ipv6(&mut to_ip);
    println!("{:?} {:?}", from_ip, to_ip);
    let mut result = "".to_string();
    for i in 0..8 {
        let from_num = u64::from_str_radix(from_ip[i], 16).unwrap();
        let to_num = u64::from_str_radix(to_ip[i], 16).unwrap();
        println!("{} {}", from_num, to_num);
        let ans = from_num ^ to_num;
        // if ans > 65535 {
        //     ans = ans - 65536;
        // }
        let ans_hex = format!("{:04x}", ans);
        result.push_str(&ans_hex.to_string());
        result.push_str(":");
    }
    result.pop();
    shorten_ipv6(&result)
}
