use strum_macros::EnumIter;

#[derive(Clone, Debug, Eq, PartialEq, Hash, EnumIter)]
pub enum Gemuese {
    Aubergine,
    Blumenkohl,
    GrueneBohnen,
    DickeBohnen,
    Brokkoli,
    Butterrueben,
    Champignons,
    Erbsen,
    Fenchel,
    Gruenkohl,
    Gurke,
    Kartoffeln,
    Kohlrabi,
    Kuerbis,
    Lauch,
    Fruehlingszwiebeln,
    Mais,
    Mangold,
    Karotten,
    Paprika,
    Pastinaken,
    Radieschen,
    Rosenkohl,
    RoteBeete,
    Rotkohl,
    Schwarzwurzeln,
    Spargel,
    Spinat,
    Spitzkohl,
    Staudensellerie,
    Steckrueben,
    Tomaten,
    Topinambur,
    Weisskohl,
    Wirsing,
    Zucchini,
    Zuckerschoten,
    Zwiebeln,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, EnumIter)]
pub enum Obst {
    Apfel,
    Aprikose,
    Birne,
    Blaubeeren,
    Brombeeren,
    Erdbeeren,
    Himbeeren,
    Holunderbeeren,
    Johannisbeeren,
    Kirschen,
    Mirabellen,
    Pflaumen,
    Quitten,
    Rhabarber,
    Stachelbeeren,
    Wassermelone,
    Weintrauben,
    Zwetschgen,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, EnumIter)]
pub enum Salat {
    Batavia,
    Chicoree,
    Eichblattsalat,
    Eisbergsalat,
    Endiviensalat,
    Feldsalat,
    Kopfsalat,
    LolloRosso,
    Portulak,
    Radicchio,
    Rucola,
}

impl TryFrom<&str> for Gemuese {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "aubergine" | "auberginen" => Ok(Gemuese::Aubergine),
            "blumenkohl" => Ok(Gemuese::Blumenkohl),
            "gruenebohnen" | "gruene bohnen" | "grüne bohnen" | "grünebohnen" | "bohnen" => {
                Ok(Gemuese::GrueneBohnen)
            }
            "dickebohnen" | "dicke bohnen" => Ok(Gemuese::DickeBohnen),
            "brokkoli" => Ok(Gemuese::Brokkoli),
            "butterrueben" | "butterrüben" | "butterrübe" | "butterruebe" => {
                Ok(Gemuese::Butterrueben)
            }
            "champignons" | "champignon" | "pilze" | "pilz" | "portobello" => {
                Ok(Gemuese::Champignons)
            }
            "erbsen" | "erbse" => Ok(Gemuese::Erbsen),
            "fenchel" => Ok(Gemuese::Fenchel),
            "gruenkohl" | "grünkohl" => Ok(Gemuese::Gruenkohl),
            "gurke" | "gurken" => Ok(Gemuese::Gurke),
            "kartoffeln" | "kartoffel" => Ok(Gemuese::Kartoffeln),
            "kohlrabi" => Ok(Gemuese::Kohlrabi),
            "kuerbis" => Ok(Gemuese::Kuerbis),
            "lauch" | "porree" => Ok(Gemuese::Lauch),
            "fruehlingszwiebeln" | "schlotten" | "lauchzwiebeln" => Ok(Gemuese::Fruehlingszwiebeln),
            "mais" => Ok(Gemuese::Mais),
            "mangold" => Ok(Gemuese::Mangold),
            "karotten" | "karotte" | "möhre" | "möhren" => Ok(Gemuese::Karotten),
            "paprika" => Ok(Gemuese::Paprika),
            "pastinaken" | "pastinake" => Ok(Gemuese::Pastinaken),
            "radieschen" => Ok(Gemuese::Radieschen),
            "rosenkohl" => Ok(Gemuese::Rosenkohl),
            "rotebeete" | "rotebete" | "rote bete" | "rote beete" => Ok(Gemuese::RoteBeete),
            "rotkohl" => Ok(Gemuese::Rotkohl),
            "schwarzwurzeln" | "schwarzwurzel" => Ok(Gemuese::Schwarzwurzeln),
            "spargel" => Ok(Gemuese::Spargel),
            "spinat" => Ok(Gemuese::Spinat),
            "spitzkohl" => Ok(Gemuese::Spitzkohl),
            "staudensellerie" | "sellerie" | "selleriestaude" => Ok(Gemuese::Staudensellerie),
            "steckrueben" | "steckrüben" | "steckruebe" | "steckrübe" => Ok(Gemuese::Steckrueben),
            "tomaten" | "tomate" => Ok(Gemuese::Tomaten),
            "topinambur" => Ok(Gemuese::Topinambur),
            "weisskohl" => Ok(Gemuese::Weisskohl),
            "wirsing" => Ok(Gemuese::Wirsing),
            "zucchini" => Ok(Gemuese::Zucchini),
            "zuckerschoten" | "zuckerschote" | "zuckererbsen" | "zuckererbse" => {
                Ok(Gemuese::Zuckerschoten)
            }
            "zwiebeln" | "zwiebel" => Ok(Gemuese::Zwiebeln),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Obst {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "apfel" | "äpfel" => Ok(Obst::Apfel),
            "aprikose" | "aprikosen" => Ok(Obst::Aprikose),
            "birne" | "birnen" => Ok(Obst::Birne),
            "blaubeeren" | "blaubeere" => Ok(Obst::Blaubeeren),
            "brombeeren" | "brombeere" => Ok(Obst::Brombeeren),
            "erdbeeren" | "erdbeere" => Ok(Obst::Erdbeeren),
            "himbeeren" | "himbeere" => Ok(Obst::Himbeeren),
            "holunderbeeren" | "holunderbeere" | "holunder" => Ok(Obst::Holunderbeeren),
            "johannisbeeren" | "johannisbeere" | "johannesbeeren" => Ok(Obst::Johannisbeeren),
            "kirschen" | "kirsche" | "kirche" | "kirchen" => Ok(Obst::Kirschen),
            "mirabellen" | "mirabelle" => Ok(Obst::Mirabellen),
            "pflaumen" | "pflaume" => Ok(Obst::Pflaumen),
            "quitten" | "quitte" => Ok(Obst::Quitten),
            "rhabarber" => Ok(Obst::Rhabarber),
            "stachelbeeren" | "stachelbeere" => Ok(Obst::Stachelbeeren),
            "wassermelone" | "wassermelonen" => Ok(Obst::Wassermelone),
            "weintrauben" | "weintraube" | "trauben" | "traube" => Ok(Obst::Weintrauben),
            "zwetschgen" | "zwetschge" | "quetsche" => Ok(Obst::Zwetschgen),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Salat {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "batavia" => Ok(Salat::Batavia),
            "chicoree" => Ok(Salat::Chicoree),
            "eichblattsalat" => Ok(Salat::Eichblattsalat),
            "eisbergsalat" => Ok(Salat::Eisbergsalat),
            "endiviensalat" => Ok(Salat::Endiviensalat),
            "feldsalat" => Ok(Salat::Feldsalat),
            "kopfsalat" => Ok(Salat::Kopfsalat),
            "lollo rosso" | "lollorosso" => Ok(Salat::LolloRosso),
            "portulak" => Ok(Salat::Portulak),
            "radicchio" => Ok(Salat::Radicchio),
            "rucola" => Ok(Salat::Rucola),
            _ => Err(()),
        }
    }
}

impl From<Gemuese> for Obst {
    fn from(_: Gemuese) -> Self {
        panic!("This should never happen!")
    }
}

impl From<Gemuese> for Salat {
    fn from(_: Gemuese) -> Self {
        panic!("This should never happen!")
    }
}

impl From<Obst> for Gemuese {
    fn from(_: Obst) -> Self {
        panic!("This should never happen!")
    }
}

impl From<Obst> for Salat {
    fn from(_: Obst) -> Self {
        panic!("This should never happen!")
    }
}

impl From<Salat> for Gemuese {
    fn from(_: Salat) -> Self {
        panic!("This should never happen!")
    }
}

impl From<Salat> for Obst {
    fn from(_: Salat) -> Self {
        panic!("This should never happen!")
    }
}
