// TODO: Better logging!
use std::env;

use crate::calendar::{SaisonkalenderGemuese, SaisonkalenderObst, SaisonkalenderSalat};
use crate::gemuese::{Gemuese, Obst, Salat};
use futures::StreamExt;
use telegram_bot::*;

mod calendar;
mod gemuese;

#[derive(Debug, Eq, PartialEq, Hash)]
enum AllowedCommands {
    Gemuese,
    Obst,
    Salat,
    Other(String),
}

impl Default for AllowedCommands {
    fn default() -> Self {
        AllowedCommands::Other(String::from(""))
    }
}

impl From<String> for AllowedCommands {
    fn from(value: String) -> Self {
        let value = value.to_lowercase();
        let obst = "obst".to_owned();
        let gemuese = "gemuese".to_owned();
        let gemuese_2 = "gemüse".to_owned();
        let salat = "salat".to_owned();

        if value == obst {
            AllowedCommands::Obst
        } else if value == gemuese || value == gemuese_2 {
            AllowedCommands::Gemuese
        } else if value == salat {
            AllowedCommands::Salat
        } else {
            AllowedCommands::Other(value)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    //log4rs::init_file(
    //    "logging_config.yaml",
    //    log4rs::config::Deserializers::default(),
    //).unwrap();

    let skg = SaisonkalenderGemuese::new();
    let sko = SaisonkalenderObst::new();
    let sks = SaisonkalenderSalat::new();

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                let command = AllowedCommands::from(data.to_lowercase().clone());

                let mut reply = String::new();

                match command {
                    AllowedCommands::Gemuese => {
                        let seasonal_gemuese_frisch = skg.get_seasonal_gemuese_frisch();
                        let seasonal_gemuese_lager = skg.get_seasonal_gemuese_lager();
                        match seasonal_gemuese_frisch {
                            None => {
                                reply+= "Ein Fehler ist aufgetreten! Bitte versuche es Erneut, oder melde ihn (Fehlercode 1).";
                            }
                            Some(sgf) => {
                                if !sgf.is_empty() {
                                    reply += "Zur Zeit gibt es folgendes Gemüse saisonal:\n";
                                    for gemuese in sgf {
                                        // TODO: Implement `Display` for `Gemuese`, `Obst` and `Salat`.
                                        reply += &format!("{:?}\n", gemuese);
                                    }
                                }
                            }
                        }
                        match seasonal_gemuese_lager {
                            None => {
                                reply+= "Ein Fehler ist aufgetreten! Bitte versuche es Erneut, oder melde ihn (Fehlercode 2).";
                            }
                            Some(sgl) => {
                                if !sgl.is_empty() {
                                    reply += "Zur Zeit gibt es folgendes Gemüse als Lagerware:\n";
                                    for gemuese in sgl {
                                        reply += &format!("{:?}\n", gemuese);
                                    }
                                }
                            }
                        }
                    }
                    AllowedCommands::Obst => {
                        let seasonal_obst_frisch = sko.get_seasonal_obst_frisch();
                        let seasonal_obst_lager = sko.get_seasonal_obst_lager();
                        match seasonal_obst_frisch {
                            None => {
                                reply+= "Ein Fehler ist aufgetreten! Bitte versuche es Erneut, oder melde ihn (Fehlercode 3).";
                            }
                            Some(sof) => {
                                if !sof.is_empty() {
                                    reply += "Zur Zeit gibt es folgendes Obst saisonal:\n";
                                    for obst in sof {
                                        reply += &format!("{:?}\n", obst);
                                    }
                                }
                            }
                        }
                        match seasonal_obst_lager {
                            None => {
                                reply+= "Ein Fehler ist aufgetreten! Bitte versuche es Erneut, oder melde ihn (Fehlercode 4).";
                            }
                            Some(sol) => {
                                if !sol.is_empty() {
                                    reply += "Zur Zeit gibt es folgendes Obst als Lagerware:\n";
                                    for obst in sol {
                                        reply += &format!("{:?}\n", obst);
                                    }
                                }
                            }
                        }
                    }
                    AllowedCommands::Salat => {
                        let seasonal_salat_frisch = sks.get_seasonal_salat_frisch();
                        let seasonal_salat_lager = sks.get_seasonal_salat_lager();
                        match seasonal_salat_frisch {
                            None => {
                                reply+= "Ein Fehler ist aufgetreten! Bitte versuche es Erneut, oder melde ihn (Fehlercode 5).";
                            }
                            Some(ssf) => {
                                if !ssf.is_empty() {
                                    reply += "Zur Zeit gibt es folgenden Salat saisonal:\n";
                                    for salat in ssf {
                                        reply += &format!("{:?}\n", salat);
                                    }
                                }
                            }
                        }
                        match seasonal_salat_lager {
                            None => {
                                reply+= "Ein Fehler ist aufgetreten! Bitte versuche es Erneut, oder melde ihn (Fehlercode 6).";
                            }
                            Some(ssl) => {
                                if !ssl.is_empty() {
                                    reply += "Zur Zeit gibt es folgenden Salat als Lagerware:\n";
                                    for salat in ssl {
                                        reply += &format!("{:?}\n", salat);
                                    }
                                }
                            }
                        }
                    }
                    AllowedCommands::Other(s) => {
                        match Salat::try_from(s.as_str()) {
                            Ok(s) => {
                                let months = sks.get_months_for(&s);
                                reply += &format!("{:?} hat Saison in den folgenden Monaten:\n", s);
                                months.iter().for_each(|m| reply += &format!("{:?}\n", m));
                            }
                            Err(_) => {
                                match Obst::try_from(s.as_str()) {
                                    Ok(o) => {
                                        let months = sko.get_months_for(&o);
                                        reply += &format!(
                                            "{:?} hat Saison in den folgenden Monaten:\n",
                                            o
                                        );
                                        months.iter().for_each(|m| reply += &format!("{:?}\n", m));
                                    }
                                    Err(_) => {
                                        match Gemuese::try_from(s.as_str()) {
                                            Ok(g) => {
                                                let months = skg.get_months_for(&g);
                                                reply += &format!(
                                                    "{:?} hat Saison in den folgenden Monaten:\n",
                                                    g
                                                );
                                                months
                                                    .iter()
                                                    .for_each(|m| reply += &format!("{:?}\n", m));
                                            }
                                            Err(_) => {
                                                // TODO: Levenshtein distance check

                                                reply += "Ich kann mit diesem Befehl (noch) nichts anfangen, tut mir leid!"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                api.send(message.text_reply(reply)).await?;
            }
        }
    }
    Ok(())
}
