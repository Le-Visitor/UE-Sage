use modbus::tcp;
use modbus::{Client, Coil};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use std::vec::Vec;

fn main() {
    let mut adress: String = "127.0.0.1".to_string(); // Affectation par défaut, typage forcé pour rendre mutable
    let mut port = 55022; // Affectation par défaut
    for (i, e) in std::env::args().enumerate() {
        if i == 1 {
            adress = e.to_string(); // Réaffectation si arguments donnés
        } else if i == 2 {
            port = e.parse::<u16>().unwrap(); // Réaffectation si arguments donnés
        }
    }
    println!("{} {}", adress, port); // Affichage des valeurs affectées

    let mut cfg = tcp::Config::default();
    cfg.tcp_port = port;
    cfg.tcp_connect_timeout = Some(Duration::from_secs(10));
    let mut client = tcp::Transport::new_with_cfg(&adress, cfg).unwrap();
    assert!(client.write_single_coil(0, Coil::On).is_ok());

    let mut nb_coils = 256; // Préparation de valeurs différentes selon si simulation ou automate
    let mut nb_holding_registers = 125;
    if port == 55022 {
        nb_coils = 20;
        nb_holding_registers = 5;
    }

    for mut _repetion in 0..20 {
        //Surveillance de l'équipement
        let coil_client = client.read_coils(0, nb_coils).unwrap();
        let coil_arrray = coil_formatting(coil_client);
        let holding_client = client
            .read_holding_registers(0, nb_holding_registers)
            .unwrap();
        println!("{:?}{:?}", coil_arrray, holding_client);
        sleep(Duration::new(0, 100000000));
        _repetion += 1;
    }
}

fn coil_formatting(coil_array: Vec<Coil>) -> String {
    let mut result: String = "".to_string();
    for e in coil_array {
        if e == Coil::from_str("On").unwrap() {
            result += &'•'.to_string();
        } else if e == Coil::from_str("Off").unwrap() {
            result += &'_'.to_string();
        }
    }
    return result;
}
