use modbus::{tcp, Client, Coil};
use std::str::FromStr;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::vec::Vec;

fn main() {
    let db = rusqlite::Connection::open("plc.db").unwrap();                      // Création de la base de données
    db.busy_handler(Some(|_retry_count| {
    std::thread::sleep(std::time::Duration::from_millis(1));
    true
    })).unwrap();
    db.execute(
        "CREATE TABLE IF NOT EXISTS acquisition (
            id      INTEGER PRIMARY KEY,
            utc_ms  INTEGER,
            adresse TEXT,
            etat    INTEGER )",
        (),
    ).unwrap();

    let mut adress: String = "127.0.0.1".to_string();                           // Affectation par défaut, typage forcé pour rendre mutable
    let mut port = 55022;                                                       // Affectation par défaut
    for (i, e) in std::env::args().enumerate() {
        if i == 1 {
            adress = e.to_string();                                             // Réaffectation si arguments donnés
        } else if i == 2 {
            port = e.parse::<u16>().unwrap();                                   // Réaffectation si arguments donnés
        }
    }
    println!("{} {}", adress, port);                                            // Affichage des valeurs affectées

    let mut cfg = tcp::Config::default();
    cfg.tcp_port = port;
    cfg.tcp_connect_timeout = Some(Duration::from_secs(10));
    let mut client = tcp::Transport::new_with_cfg(&adress, cfg).unwrap();
    assert!(client.write_single_coil(0, Coil::On).is_ok());

    let mut nb_coils = 256;                                                     // Préparation de valeurs différentes selon si simulation ou automate
    let mut nb_holding_registers = 125;
    if port == 55022 {
        nb_coils = 20;
        nb_holding_registers = 5;
    }

    let mut coil_client = client.read_coils(0, nb_coils).unwrap();
    let mut holding_client = client
            .read_holding_registers(0, nb_holding_registers)
            .unwrap();

    let time_server = 100;                                                      //time for changing state of server en ms
    let time_repetion = 20;                                                     //temps voulue de la boucle en s
    for mut _repetion in 0..((time_repetion * 1000) / time_server) {
                                                                                //Surveillance de l'équipement
        let coil_client_temp = client.read_coils(0, nb_coils).unwrap();
        let holding_client_temp = client
            .read_holding_registers(0, nb_holding_registers)
            .unwrap();
        if coil_client != coil_client_temp
            || holding_client != holding_client_temp
        {
            let coil_array_old = coil_formatting(coil_client.clone());
            let coil_array_new = coil_formatting(coil_client_temp.clone());
            store_coil_event(db, SystemTime::now(), coil_array_old, coil_array_new);
            store_holding_event(db, SystemTime::now(), holding_client, holding_client_temp);
            coil_client = coil_client_temp;
            holding_client = holding_client_temp;
        }
        sleep(Duration::new(0, time_server * 1000000));
    }
}

fn coil_formatting(coil_array : Vec<Coil>) -> String {
    let mut result : String = "".to_string();
    for e in coil_array {
        if e == Coil::from_str("On").unwrap() {
            result += &'•'.to_string();
        } else if e == Coil::from_str("Off").unwrap() {
            result += &'_'.to_string();
        }
    }
    return result;
}

fn store_coil_events(
    db: &rusqlite::Connection,
    utc_ms: u64,
    prev_values: &mut [modbus::Coil],
    new_values: &[modbus::Coil],
) {
    let state= modbus::Coil::On as u8;
    for (i, (ref_to_a, ref_to_b)) in prev_values.iter_mut().zip(new_values.iter()).enumerate() {
        if ref_to_a != ref_to_b {
            let plc_addr = format!("%M{}",i);
            db.execute(
                "INSERT INTO acquisition (utc_ms, adresse, etat)
                VALUES (?1, ?2, ?3)",
                (utc_ms as i64, plc_addr, state),)
                .unwrap();
        }
    }
}

fn store_holding_events(
    db: &rusqlite::Connection,
    utc_ms: u64,
    prev_values: &mut[u8],
    new_values: &[u8],
) {
    let state = new_values;
    for (i, (ref_to_c, ref_to_d)) in prev_values.iter_mut().zip(new_values.iter()).enumerate() {
        if ref_to_c != ref_to_d {
            let plc_addr = format!("%MW{}",i);
            db.execute(
                "INSERT INTO acquisition (utc_ms, adresse, etat)
                VALUES (?1, ?2, ?3)",
                (utc_ms as i64, plc_addr, state),)
                .unwrap();
        }
    }
}
