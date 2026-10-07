use modbus::{Client, Coil};
use modbus::tcp;
use std::time::Duration;

fn main() {
    let mut adress : String = "127.0.0.1".to_string();                      // Affectation par défaut, typage forcé pour rendre mutable
    let mut port = 55022;                                                   // Affectation par défaut
    for (i, e) in std::env::args().enumerate() {
        if i == 1 {
            adress = e.to_string() ;                                        // Réaffectation si arguments donnés
        } else if i == 2 {
            port = e.parse::<u16>().unwrap();                               // Réaffectation si arguments donnés
        }
    }
    println!("{} {}", adress, port);                                        // Affichage des valeurs affectées

    let mut cfg = tcp::Config::default();
    cfg.tcp_port = port;
    cfg.tcp_connect_timeout = Some(Duration::from_secs(10));
    let mut client = tcp::Transport::new_with_cfg(&adress, cfg).unwrap();
    assert!(client.write_single_coil(0, Coil::On).is_ok());
}