/// Script de Contrôle pour Prise Tapo
///
/// Ce script est conçu pour contrôler une prise intelligente Tapo (P110) via son API.
/// Il permet à l'utilisateur d'allumer ou d'éteindre l'appareil en fonction de l'action spécifiée.
/// De plus, le script peut afficher des informations sur l'appareil ne repondant un json ou en l'envoyant vers un broker MQTT'.
///
/// Utilisation :
///     cargo run <adresse_ip> <login> <pass> <action> <jeton>
///
/// Paramètres :
///     - adresse_ip : L'adresse IP de la prise intelligente Tapo.
///     - action : Action à effectuer ('on' pour allumer, 'off' pour éteindre, 'status' pour obtenir les infos).
///     - protocol : MQTT la prise renverra alors un message mqtt
///
/// Exemple :
///     cargo run 192.168.0.70 device_info mqtt
///
/// Remarques :
///     - Le script utilise l'API Tapo via la crate `tapo`.
///
///
/// Credit : // https://github.com/mihai-dinculescu/tapo

/// Script de Contrôle pour Prise Tapo
/// ...

use std::{env, process, time::Duration};
extern crate paho_mqtt as mqtt;
use tokio::time::Duration as TokioDuration;
use config::{Config, File};
use serde::Deserialize;
use tapo::ApiClient;
use serde_json::to_string;



// Pour recuperer les valeurs dans le fichier config
#[derive(Debug, Deserialize)]
struct MqttConfig {
    broker_address: String,
    topic_name: String,
}


#[derive(Debug, Deserialize)]
struct TapoConfig {
    tapo_username: String,
    tapo_password: String,
}


fn load_config() -> Result<(MqttConfig, TapoConfig), Box<dyn std::error::Error>> {
    // Créez une nouvelle instance de configuration
    let mut settings = Config::default();

    // Obtenez le répertoire actuel de l'exécutable
    let exe_path = env::current_exe()?;
    let current_dir = exe_path.parent().ok_or("Impossible de déterminer le répertoire de l'exécutable")?;

    // Vérifiez l'existence du fichier de configuration personnel
    let config_file_name = if current_dir.join("ft_tp110_config_perso.toml").exists() {
        "ft_tp110_config_perso.toml"
    } else {
        "ft_tp110_config.toml"
    };

    // Construisez le chemin absolu du fichier de configuration
    let config_path = current_dir.join(config_file_name);

    // Chargez la configuration à partir du fichier
    if let Err(e) = settings.merge(File::from(config_path)) {
        // Vérifiez s'il s'agit d'une erreur indiquant l'absence du fichier
        if e.to_string().contains("was not found") {
            eprintln!("Le fichier de configuration {} est manquant.", config_file_name);
        } else {
            // Gérez d'autres erreurs si nécessaire
            return Err(Box::new(e));
        }
    }

    // Obtenez les valeurs de la configuration
    let mqtt_config: MqttConfig = settings.get("mqtt")?;
    let tapo_config: TapoConfig = settings.get("tapo")?;

    Ok((mqtt_config, tapo_config))
}


// Fonction pour envoyer le message MQTT
fn send_mqtt_message(broker: String, client_id: String, mut topic: String, message_content: String, quelproto: String ) {
    
    if quelproto != "none" {

        // Construisons ce qui va être le nom de l'equipement decouvert en automatique par le plugin Jeedom MQTT
        // Dansft_tp110_config.toml j'ai mis topic_name = "/maison/TpLink_"
        // Ici je reprends le derniers champs de l'adresse ip de la prise
        // Mon equipement est a l'adresse 192.168.0.70
        // La chaine construite sera alors "/maison/TpLink_70"
        let last_octet = client_id.split('.').last().unwrap_or("0");
        topic.push_str(last_octet);
        let qos = 1;
        let host = broker.to_string();

        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(host)
            .client_id(client_id.to_string())
            .finalize();

        let cli = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
            println!("Error creating the client: {:?}", err);
            process::exit(1);
        });

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(20))
            .clean_session(true)
            .finalize();

        if let Err(e) = cli.connect(conn_opts) {
            println!("Unable to connect: {:?}", e);
            process::exit(1);
        }

        let content = message_content;
        let msg = mqtt::Message::new(topic.to_string(), content.to_string(), qos);
        if let Err(e) = cli.publish(msg) {
            println!("Error sending message: {:?}", e);
            process::exit(1);
        }

        let tok = cli.disconnect(None);
        println!("Disconnect from the broker");
        tok.unwrap();
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Chargez la configuration à partir du fichier
    let (mqtt_config, tapo_config) = match load_config() {
        Ok(configs) => configs,
        Err(err) => {
            eprintln!("Erreur lors du chargement de la configuration : {}", err);
            process::exit(1);
        }
    };

    
    // Attente pour laisser le temps à MQTT de traiter le message (peut être ajusté selon vos besoins)
    tokio::time::sleep(TokioDuration::from_secs(2)).await;

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 4 {   //Il faut 2 arguments obligatoire le 3eme facutatif
        eprintln!("Usage: cargo run <adresse_ip> <action> <protocol>");
        process::exit(1);
    }

    let ip_address = &args[1];
    let action_jeedom = &args[2];
    let quelprotocol = args.get(3).map_or("none", |p| p);      // Si il y a pas de parametre quelprotocol vaudra "none" sinon la valeur du parametre passé au script

    
    // Les variables pour mqtt    
    //let _broker_address = mqtt_config.broker_address;
    let default_broker = format!("tcp://{}:1883", mqtt_config.broker_address);
    // println!("Default Broker: {}", default_broker);
    // default_client est le nom qui est presenté au brocker pour le suivi du coup je vais y mettre la chaine de caractere de l'adresse ip comme ça c'est bien unique pourle reseau
    //let default_client = "rust_publish".to_string();
    let default_client = ip_address;
    //let _topic_name = mqtt_config.topic_name;
    let default_topic = mqtt_config.topic_name;
    

    // Les variables pour l'api tapo
    let tapo_username = tapo_config.tapo_username;
    let tapo_password = tapo_config.tapo_password;

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
            println!("{}", json_string);
            send_mqtt_message(default_broker, default_client.to_string(), default_topic, json_string, quelprotocol.to_string());
        }
        "device_usage" => {
            let device_usage = device.get_device_usage().await?;
            let json_string = to_string(&device_usage)?;
            println!("{}", json_string);
            // Envoie le JSON au broker MQTT
            //connect_and_publish_message(default_broker, &default_client, default_topic, &json_string).await?;
        }
        "get_current_power" => {
            // Quand c'etait renvoi sous forme json
            let current_power = device.get_current_power().await?;
            let json_string = to_string(&current_power)?;
            //println!("{}", json_string)
            send_mqtt_message(default_broker, default_client.to_string(), default_topic, json_string, quelprotocol.to_string());
            //let current_power_result = device.get_current_power().await?;
            //let current_power_value = current_power_result.current_power;
            println!("{}", current_power.current_power);
        }
        "get_energy_usage" => {
            let energy_usage = device.get_energy_usage().await?;
            let json_string = to_string(&energy_usage)?;           
            println!("{}", json_string);
            send_mqtt_message(default_broker, default_client.to_string(), default_topic, json_string, quelprotocol.to_string());
        }
        _ => {
            eprintln!("Invalid action. Use 'on' or 'off' or 'device_info' or 'device_usage' or 'get_current_power' or 'get_energy_usage.");
            process::exit(1);
        }
    }

    Ok(())
}
