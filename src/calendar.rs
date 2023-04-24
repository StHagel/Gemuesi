use crate::gemuese::{Gemuese, Obst, Salat};
use std::collections::HashMap;
use time::Month;
use time::Month::*;

use chrono::Datelike;

#[derive(Debug, Eq, PartialEq, Default)]
pub struct SaisonkalenderGemuese {
    frisch: HashMap<Gemuese, Vec<Month>>,
    lagerware: HashMap<Gemuese, Vec<Month>>,
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct SaisonkalenderObst {
    frisch: HashMap<Obst, Vec<Month>>,
    lagerware: HashMap<Obst, Vec<Month>>,
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct SaisonkalenderSalat {
    frisch: HashMap<Salat, Vec<Month>>,
    lagerware: HashMap<Salat, Vec<Month>>,
}

impl SaisonkalenderGemuese {
    pub fn new() -> SaisonkalenderGemuese {
        let mut frisch = HashMap::new();
        let mut lagerware = HashMap::new();

        frisch.insert(Gemuese::Aubergine, vec![July, August, September, October]);
        frisch.insert(
            Gemuese::Blumenkohl,
            vec![May, June, July, August, September, October],
        );
        frisch.insert(
            Gemuese::GrueneBohnen,
            vec![July, August, September, October],
        );
        frisch.insert(Gemuese::DickeBohnen, vec![June, July, August]);
        frisch.insert(
            Gemuese::Brokkoli,
            vec![June, July, August, September, October],
        );
        frisch.insert(
            Gemuese::Butterrueben,
            vec![August, September, October, November, December],
        );
        frisch.insert(
            Gemuese::Champignons,
            vec![
                January, February, March, April, May, June, July, August, September, October,
                November, December,
            ],
        );
        frisch.insert(Gemuese::Erbsen, vec![June, July, August]);
        frisch.insert(
            Gemuese::Fenchel,
            vec![June, July, August, September, October, November],
        );
        frisch.insert(
            Gemuese::Gruenkohl,
            vec![November, December, January, February],
        );
        frisch.insert(Gemuese::Gurke, vec![June, July, August, September, October]);
        frisch.insert(
            Gemuese::Kartoffeln,
            vec![June, July, August, September, October],
        );
        frisch.insert(
            Gemuese::Kohlrabi,
            vec![May, June, July, August, September, October],
        );
        frisch.insert(Gemuese::Kuerbis, vec![August, September, October, November]);
        frisch.insert(
            Gemuese::Lauch,
            vec![
                January, February, March, April, May, June, July, August, September, October,
                November, December,
            ],
        );
        frisch.insert(
            Gemuese::Fruehlingszwiebeln,
            vec![May, June, July, August, September, October],
        );
        frisch.insert(Gemuese::Mais, vec![August, September, October]);
        frisch.insert(
            Gemuese::Mangold,
            vec![May, June, July, August, September, October],
        );
        frisch.insert(
            Gemuese::Karotten,
            vec![June, July, August, September, October],
        );
        frisch.insert(Gemuese::Paprika, vec![July, August, September, October]);
        frisch.insert(Gemuese::Pastinaken, vec![]);
        frisch.insert(
            Gemuese::Radieschen,
            vec![May, June, July, August, September, October],
        );
        frisch.insert(
            Gemuese::Rosenkohl,
            vec![October, November, December, January, February, March],
        );
        frisch.insert(
            Gemuese::RoteBeete,
            vec![July, August, September, October, November],
        );
        frisch.insert(
            Gemuese::Rotkohl,
            vec![June, July, August, September, October, November],
        );
        frisch.insert(
            Gemuese::Schwarzwurzeln,
            vec![October, November, December, January, February],
        );
        frisch.insert(Gemuese::Spargel, vec![April, May, June]);
        frisch.insert(
            Gemuese::Spinat,
            vec![March, April, May, September, October, November],
        );
        frisch.insert(Gemuese::Spitzkohl, vec![May, June]);
        frisch.insert(
            Gemuese::Staudensellerie,
            vec![July, August, September, October],
        );
        frisch.insert(
            Gemuese::Steckrueben,
            vec![August, September, October, November, December],
        );
        frisch.insert(Gemuese::Tomaten, vec![July, August, September, October]);
        frisch.insert(
            Gemuese::Topinambur,
            vec![October, November, December, January, February, March],
        );
        frisch.insert(
            Gemuese::Weisskohl,
            vec![June, July, August, September, October, November],
        );
        frisch.insert(
            Gemuese::Wirsing,
            vec![
                May, June, July, August, September, October, November, December, January, February,
            ],
        );
        frisch.insert(
            Gemuese::Zucchini,
            vec![June, July, August, September, October],
        );
        frisch.insert(Gemuese::Zuckerschoten, vec![June, July, August]);
        frisch.insert(Gemuese::Zwiebeln, vec![July, August, September, October]);

        lagerware.insert(Gemuese::Butterrueben, vec![January, February, March, April]);
        lagerware.insert(
            Gemuese::Kartoffeln,
            vec![November, December, January, February, March, April, May],
        );
        lagerware.insert(Gemuese::Kuerbis, vec![December, January, February]);
        lagerware.insert(
            Gemuese::Karotten,
            vec![November, December, January, February, March, April, May],
        );
        lagerware.insert(Gemuese::Pastinaken, vec![April]);
        lagerware.insert(
            Gemuese::RoteBeete,
            vec![December, January, February, March, April],
        );
        lagerware.insert(
            Gemuese::Rotkohl,
            vec![December, January, February, March, April, May],
        );
        lagerware.insert(Gemuese::Steckrueben, vec![January, February, March]);
        lagerware.insert(
            Gemuese::Weisskohl,
            vec![December, January, February, March, April],
        );
        lagerware.insert(Gemuese::Wirsing, vec![March]);
        lagerware.insert(
            Gemuese::Zwiebeln,
            vec![
                November, December, January, February, March, April, May, June,
            ],
        );

        SaisonkalenderGemuese { frisch, lagerware }
    }

    pub fn get_seasonal_gemuese_frisch(&self) -> Option<Vec<Gemuese>> {
        let current_month = get_current_month()?;

        let mut seasonal_gemuese = vec![];
        for (gemuese, season) in &self.frisch {
            if season.contains(&current_month) {
                seasonal_gemuese.push(gemuese.clone())
            }
        }

        Some(seasonal_gemuese)
    }

    pub fn get_seasonal_gemuese_lager(&self) -> Option<Vec<Gemuese>> {
        let current_month = get_current_month()?;

        let mut seasonal_gemuese = vec![];
        for (gemuese, season) in &self.lagerware {
            if season.contains(&current_month) {
                seasonal_gemuese.push(gemuese.clone())
            }
        }

        Some(seasonal_gemuese)
    }
}

impl SaisonkalenderObst {
    pub fn new() -> SaisonkalenderObst {
        let mut frisch = HashMap::new();
        let mut lagerware = HashMap::new();

        frisch.insert(Obst::Apfel, vec![August, September, October, November]);
        frisch.insert(Obst::Aprikose, vec![July, August]);
        frisch.insert(Obst::Birne, vec![August, September, October]);
        frisch.insert(Obst::Blaubeeren, vec![June, July, August, September]);
        frisch.insert(Obst::Brombeeren, vec![July, August, September]);
        frisch.insert(Obst::Erdbeeren, vec![May, June, July]);
        frisch.insert(Obst::Himbeeren, vec![June, July, August]);
        frisch.insert(Obst::Holunderbeeren, vec![September, October]);
        frisch.insert(Obst::Johannisbeeren, vec![June, July, August]);
        frisch.insert(Obst::Kirschen, vec![June, July, August]);
        frisch.insert(Obst::Mirabellen, vec![July, August, September]);
        frisch.insert(Obst::Pflaumen, vec![July, August, September]);
        frisch.insert(Obst::Quitten, vec![September, October, November]);
        frisch.insert(Obst::Rhabarber, vec![April, May, June]);
        frisch.insert(Obst::Stachelbeeren, vec![June, July, August]);
        frisch.insert(Obst::Wassermelone, vec![August, September]);
        frisch.insert(Obst::Weintrauben, vec![September, October]);
        frisch.insert(Obst::Zwetschgen, vec![July, August, September, October]);

        lagerware.insert(
            Obst::Apfel,
            vec![December, January, February, March, April, May],
        );
        lagerware.insert(Obst::Birne, vec![November, December]);

        SaisonkalenderObst { frisch, lagerware }
    }

    pub fn get_seasonal_obst_frisch(&self) -> Option<Vec<Obst>> {
        let current_month = get_current_month()?;

        let mut seasonal_obst = vec![];
        for (obst, season) in &self.frisch {
            if season.contains(&current_month) {
                seasonal_obst.push(obst.clone())
            }
        }

        Some(seasonal_obst)
    }

    pub fn get_seasonal_obst_lager(&self) -> Option<Vec<Obst>> {
        let current_month = get_current_month()?;

        let mut seasonal_obst = vec![];
        for (obst, season) in &self.lagerware {
            if season.contains(&current_month) {
                seasonal_obst.push(obst.clone())
            }
        }

        Some(seasonal_obst)
    }
}

impl SaisonkalenderSalat {
    pub fn new() -> SaisonkalenderSalat {
        let mut frisch = HashMap::new();
        let mut lagerware = HashMap::new();

        frisch.insert(Salat::Batavia, vec![May, June, July, August, September]);
        frisch.insert(
            Salat::Chicoree,
            vec![October, November, December, January, February, March, April],
        );
        frisch.insert(
            Salat::Eichblattsalat,
            vec![May, June, July, August, September, October],
        );
        frisch.insert(
            Salat::Eisbergsalat,
            vec![June, July, August, September, October],
        );
        frisch.insert(
            Salat::Endiviensalat,
            vec![
                May, June, July, August, September, October, November, December,
            ],
        );
        frisch.insert(
            Salat::Feldsalat,
            vec![October, November, December, January, February, March, April],
        );
        frisch.insert(
            Salat::Kopfsalat,
            vec![May, June, July, August, September, October],
        );
        frisch.insert(
            Salat::LolloRosso,
            vec![May, June, July, August, September, October],
        );
        frisch.insert(
            Salat::Portulak,
            vec![
                July, August, September, October, November, December, January, February, March,
                April,
            ],
        );
        frisch.insert(Salat::Radicchio, vec![August, September, October, November]);
        frisch.insert(
            Salat::Rucola,
            vec![April, May, June, July, August, September],
        );

        lagerware.insert(Salat::Radicchio, vec![December, January, February]);

        SaisonkalenderSalat { frisch, lagerware }
    }

    pub fn get_seasonal_salat_frisch(&self) -> Option<Vec<Salat>> {
        let current_month = get_current_month()?;

        let mut seasonal_salat = vec![];
        for (salat, season) in &self.frisch {
            if season.contains(&current_month) {
                seasonal_salat.push(salat.clone())
            }
        }

        Some(seasonal_salat)
    }

    pub fn get_seasonal_salat_lager(&self) -> Option<Vec<Salat>> {
        let current_month = get_current_month()?;

        let mut seasonal_salat = vec![];
        for (salat, season) in &self.lagerware {
            if season.contains(&current_month) {
                seasonal_salat.push(salat.clone())
            }
        }

        Some(seasonal_salat)
    }
}

fn get_current_month() -> Option<Month> {
    match chrono::Local::now().month() {
        1 => Some(January),
        2 => Some(February),
        3 => Some(March),
        4 => Some(April),
        5 => Some(May),
        6 => Some(June),
        7 => Some(July),
        8 => Some(August),
        9 => Some(September),
        10 => Some(October),
        11 => Some(November),
        12 => Some(December),
        _ => None,
    }
}
