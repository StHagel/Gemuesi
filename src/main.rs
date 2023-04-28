#![feature(specialization)]
// TODO: Better logging!
use std::env;

use crate::calendar::{GermanString, Saisonkalender};
use crate::gemuese::{Gemuese, Obst, Salat};

use futures::StreamExt;
use levenshtein::levenshtein;
use strum::IntoEnumIterator;
use telegram_bot::*;

mod calendar;
mod gemuese;

const LEVENSHTEIN_THRESHOLD: usize = 3;

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

    let skg = Saisonkalender::new();
    let sko = Saisonkalender::new();
    let sks = Saisonkalender::new();

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
                        let seasonal_gemuese_frisch = skg.get_seasonal_frisch();
                        let seasonal_gemuese_lager = skg.get_seasonal_lager();
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
                        let seasonal_obst_frisch = sko.get_seasonal_frisch();
                        let seasonal_obst_lager = sko.get_seasonal_lager();
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
                        let seasonal_salat_frisch = sks.get_seasonal_frisch();
                        let seasonal_salat_lager = sks.get_seasonal_lager();
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
                        if let Ok(s2) = Salat::try_from(s.as_str()) {
                            let months = sks.get_months_for(&s2);
                            reply += &format!("{:?} hat Saison in den folgenden Monaten:\n", s2);
                            months
                                .iter()
                                .for_each(|m| reply += &(m.to_german_string() + "\n"));
                        } else if let Ok(o) = Obst::try_from(s.as_str()) {
                            let months = sko.get_months_for(&o);
                            reply += &format!("{:?} hat Saison in den folgenden Monaten:\n", o);
                            months
                                .iter()
                                .for_each(|m| reply += &(m.to_german_string() + "\n"));
                        } else if let Ok(g) = Gemuese::try_from(s.as_str()) {
                            let months = skg.get_months_for(&g);
                            reply += &format!("{:?} hat Saison in den folgenden Monaten:\n", g);
                            months
                                .iter()
                                .for_each(|m| reply += &(m.to_german_string() + "\n"));
                        } else {
                            let mut hit = false;
                            Gemuese::iter().for_each(|g| {
                                Obst::iter().for_each(|o| {
                                    Salat::iter().for_each(|s2| {
                                        let (g_string, o_string, s_string) = (
                                            format!("{g:?}").to_lowercase(),
                                            format!("{o:?}").to_lowercase(),
                                            format!("{s2:?}").to_lowercase(),
                                        );
                                        if !hit {
                                            let str_to_append =
                                                if levenshtein(g_string.as_str(), s.as_str())
                                                    < LEVENSHTEIN_THRESHOLD
                                                    && !hit
                                                {
                                                    hit = true;
                                                    format!("{g:?}")
                                                } else if levenshtein(o_string.as_str(), s.as_str())
                                                    < LEVENSHTEIN_THRESHOLD
                                                    && !hit
                                                {
                                                    hit = true;
                                                    format!("{o:?}")
                                                } else if levenshtein(s_string.as_str(), s.as_str())
                                                    < LEVENSHTEIN_THRESHOLD
                                                    && !hit
                                                {
                                                    hit = true;
                                                    format!("{s:?}")
                                                } else {
                                                    "".to_owned()
                                                };
                                            if hit {
                                                reply += &format!(
                                                    "\nMeintest du vielleicht:\n{str_to_append}"
                                                );
                                            }
                                        }
                                    });
                                });
                            });
                            if !hit {
                                reply += "Ich kann mit diesem Befehl (noch) nichts anfangen, tut mir leid!";
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
