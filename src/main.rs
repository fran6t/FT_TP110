/// Script de Contrôle pour Prise Tapo
///
/// Ce script est conçu pour contrôler une prise intelligente Tapo (P110) via son API.
/// Il permet à l'utilisateur d'allumer ou d'éteindre l'appareil en fonction de l'action spécifiée.
/// De plus, le script peut afficher des informations sur l'appareil en mode 'status'.
///
/// Utilisation :
///     cargo run <adresse_ip> <login> <pass> <action> <jeton>
///
/// Paramètres :
///     - adresse_ip : L'adresse IP de la prise intelligente Tapo.
///     - login : Nom d'utilisateur du compte Tapo.
///     - pass : Mot de passe du compte Tapo.
///     - action : Action à effectuer ('on' pour allumer, 'off' pour éteindre, 'status' pour obtenir les infos).
///
/// Exemple :
///     cargo run 192.168.1.100 mon_nom_utilisateur mon_mot_de_passe on mon_jeton_de_securite
///
/// Remarques :
///     - Le script utilise l'API Tapo via la crate `tapo`.
///     - La journalisation est mise en œuvre pour afficher des informations sur l'exécution du script.
///     - En mode 'status', diverses informations sur l'appareil sont imprimées dans la console.
///
/// Codes de Sortie :
///     - 0 : Le script s'est exécuté avec succès.
///     - 1 : Nombre incorrect d'arguments ou action spécifiée invalide.
///
/// Credit : // https://github.com/mihai-dinculescu/tapo

/// Script de Contrôle pour Prise Tapo
/// ...

use std::{env, process};
use tapo::ApiClient;
use serde_json::to_string;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Suppression de la configuration de la journalisation
    // ...

    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: cargo run <adresse_ip> <login> <pass> <action>");
        process::exit(1);
    }

    let ip_address = &args[1];
    let tapo_username = &args[2];
    let tapo_password = &args[3];
    let action_jeedom = &args[4];

    // Suppression des lignes de journalisation
    // ...

    let device = ApiClient::new(tapo_username, tapo_password)?
        .p110(ip_address)
        .await?;

    match action_jeedom.as_str() {
        "on" => {
            // Ancien code qui renvoyait un json que je n'arrive pas a traiter depuis le plugin script en direct
            // device.on().await?;
            // println!(r#"{{"error_code": 0, "device_on": true}}"#);
            device.on().await?;
            println!("1");
        }
        "off" => {
            //println!(r#"{{"error_code": 0, "device_on": false}}"#);
            device.off().await?;
            println!("0");
        }
        "device_info" => {
            let device_info = device.get_device_info().await?;
            let json_string = to_string(&device_info)?;
            println!("{}", json_string)
        }
        "device_usage" => {
            let device_usage = device.get_device_usage().await?;
            let json_string = to_string(&device_usage)?;
            println!("{}", json_string)
        }
        "get_current_power" => {
            // Quand c'etait renvoi sous forme json
            //let current_power = device.get_current_power().await?;
            //let json_string = to_string(&current_power)?;
            //println!("{}", json_string)
            let current_power_result = device.get_current_power().await?;
            let current_power = current_power_result.current_power;
            println!("{}", current_power);
        }
        "get_energy_usage" => {
            let energy_usage = device.get_energy_usage().await?;
            let json_string = to_string(&energy_usage)?;
            println!("{}", json_string)
        }
        _ => {
            eprintln!("Invalid action. Use 'on' or 'off'.");
            process::exit(1);
        }
    }

    Ok(())
}
