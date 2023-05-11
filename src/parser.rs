use crate::data::{SearchGlobalCommand, SearchGlobalResult};
use crate::drawer::{draw_global_info_box, draw_local_info_box, draw_error_box};
use ifcfg::{IfCfg, AddressFamily};


const REQUEST_ADDR: &str = "https://www.ipapi.co/";


pub async fn get_global_ip_response(
    requested_ip: SearchGlobalCommand
) {
    let request_address = (REQUEST_ADDR.to_string() + &requested_ip.global_ip_addr).to_string() + "/json";
    let client = reqwest::get(request_address)
        .await
        .expect("[ERROR-START]\n\nUnable to get info about this IP!\n\n[ERROR-END]")
        .json::<SearchGlobalResult>()
        .await;

    match client {
        Ok(res) => {
            draw_global_info_box(
                res.ip,
                res.continent_code,
                res.country_name,
                res.city,
                res.postal,
                res.org,
                res.asn,
            );
        },
        Err(_) => { draw_error_box(); }
    }
}


pub fn get_local_ip_response() {
    let ifcfg_init = IfCfg::get();
    match ifcfg_init {
        Ok(val) => {
            for i in 0..val.len() {
                let info = &val[i];
                info.addresses.iter()
                    .for_each(move |addr| {
                        match addr.address_family {
                            AddressFamily::IPv6 => {
                                if let Some(ip) = addr.address {   
                                    draw_local_info_box(
                                        info.name.to_string(),
                                        info.mac.to_string(),
                                        ip.to_string()
                                    );
                                }
                            },
                            AddressFamily::IPv4 => {
                                if let Some(ip) = addr.address {
                                    draw_local_info_box(
                                        info.name.to_string(),
                                        info.mac.to_string(),
                                        ip.to_string()
                                    );
                                }
                            },
                            AddressFamily::Packet => {},
                            _ => panic!("YEP"),
                        }
                    });
            }
        },
        Err(_) => { draw_error_box(); },
    }
}
