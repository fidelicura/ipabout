pub fn draw_global_info_box(
    ip: String,
    continent: String,
    country: String,
    city: String,
    postal: String,
    provider: String,
    asn: String,
) {
    let ip_str = format!("╞═> IP: [\"{}\"]", ip);
    let loc_str = format!("╞═> Location: [\"{}\", \"{}\", \"{}, \"{}\"]", continent, country, city, postal);
    let asn_str = format!("╞═> Provider: [\"{}\", \"{}\"]", provider, asn);

    let ip_len = ip_str.len() as u16;
    let loc_len = loc_str.len() as u16;
    let asn_len = asn_str.len() as u16;

    let longest_str_len: u16 = *[ip_len, loc_len, asn_len].iter().max().unwrap() as u16;
    let width: u16 = 3 + longest_str_len;
    let height: u16 = 9;


    print!("\n╭");
    for i in 0..width-2 {
        match i {
            2 => print!("┤"),
            3 => print!("R"),
            4 => print!("E"),
            5 => print!("S"),
            6 => print!("P"),
            7 => print!("O"),
            8 => print!("N"),
            9 => print!("S"),
            10 => print!("E"),
            11 => print!("├"),
            _ => print!("─"),
        }
    }
    print!("╮\n");

    for i in 0..height-2 {
        match i {
            1 => {
                print!("{ip_str}");
                for _ in 0..width - ip_len + 3 { print!(" "); }
            },
            3 => {
                print!("{loc_str}");
                for _ in 0..width - loc_len + 3 { print!(" "); }
            },
            5 => {
                print!("{asn_str}");
                for _ in 0..width - asn_len + 3 { print!(" "); }
            },
            _ => {
                print!("│");
                for _ in 0..width - 2 { print!(" "); }
            },
        }
        print!("│\n");
    }

    print!("╰");
    for i in 0..width-2 {
        match i {
            2 => print!("┤"),
            3 => print!("R"),
            4 => print!("E"),
            5 => print!("S"),
            6 => print!("P"),
            7 => print!("O"),
            8 => print!("N"),
            9 => print!("S"),
            10 => print!("E"),
            11 => print!("├"),
            _ => print!("─"),
        }
    }
    print!("╯\n\n");
}


pub fn draw_local_info_box(
    name: String,
    mac: String,
    ip: String,
) {
    let name_str = format!("╞═> Name: [\"{}\"]", name);
    let mac_str = format!("╞═> MAC: [\"{}\"]", mac);
    let ip_str = format!("╞═> IP: [\"{}\"]", ip);

    let name_len: u16 = name_str.len() as u16;
    let mac_len: u16 = mac_str.len() as u16;
    let ip_len: u16 = ip_str.len() as u16;

    let longest_str_len: u16 = *[name_len, mac_len, ip_len].iter().max().unwrap() as u16;
    let width: u16 = 3 + longest_str_len;
    let height: u16 = 9;


    print!("\n╭");
    for i in 0..width-2 {
        match i {
            2 => print!("┤"),
            3 => print!("R"),
            4 => print!("E"),
            5 => print!("S"),
            6 => print!("P"),
            7 => print!("O"),
            8 => print!("N"),
            9 => print!("S"),
            10 => print!("E"),
            11 => print!("├"),
            _ => print!("─"),
        }
    }
    print!("╮\n");

    for i in 0..height-2 {
        match i {
            1 => {
                print!("{name_str}");
                for _ in 0..width - name_len + 3 { print!(" "); }
            },
            3 => {
                print!("{mac_str}");
                for _ in 0..width - mac_len + 3 { print!(" "); }
            },
            5 => {
                print!("{ip_str}");
                for _ in 0..width - ip_len + 3 { print!(" "); }
            },
            _ => {
                print!("│");
                for _ in 0..width - 2 { print!(" "); }
            },
        }
        print!("│\n");
    }

    print!("╰");
    for i in 0..width-2 {
        match i {
            2 => print!("┤"),
            3 => print!("R"),
            4 => print!("E"),
            5 => print!("S"),
            6 => print!("P"),
            7 => print!("O"),
            8 => print!("N"),
            9 => print!("S"),
            10 => print!("E"),
            11 => print!("├"),
            _ => print!("─"),
        }
    }
    print!("╯\n\n");
}


pub fn draw_error_box() {
    let msg_str = format!("╞═> Message: [\"UNABLE TO FIND INFO ABOUT THIS IP\"]");
    let msg_len: u16 = msg_str.len() as u16;

    let height: u16 = 5;
    let width: u16 = 3 + msg_len;


    print!("\n╭");
    for i in 0..width-2 {
        match i {
            2 => print!("┤"),
            3 => print!("E"),
            4 => print!("R"),
            5 => print!("R"),
            6 => print!("O"),
            7 => print!("R"),
            8 => print!("├"),
            _ => print!("─"),
        }
    }
    print!("╮\n");

    for i in 0..height-2 {
        match i {
            2 => {
                print!("╞═> Message: [UNABLE TO FIND INFO]");
                for _ in 0..width - msg_len + 2 { print!(" "); }
            },
            _ => {
                print!("│");
                for _ in 0..width - 2 { print!(" "); }
            },
        }
        print!("│\n");
    }

    print!("╰");
    for i in 0..width-2 {
        match i {
            2 => print!("┤"),
            3 => print!("E"),
            4 => print!("R"),
            5 => print!("R"),
            6 => print!("O"),
            7 => print!("R"),
            8 => print!("├"),
            _ => print!("─"),
        }
    }
    print!("╯\n\n");
}
