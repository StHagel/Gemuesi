use crate::gemuese::{Gemuese, Obst, Salat};
use std::collections::HashMap;
use std::hash::Hash;
use time::Month;
use time::Month::*;

#[inline]
pub fn fill_gemuese<T>(frisch: &mut HashMap<T, Vec<Month>>, lagerware: &mut HashMap<T, Vec<Month>>)
where
    T: Eq + Hash + From<Gemuese>,
{
    frisch.insert(
        Gemuese::Aubergine.into(),
        vec![July, August, September, October],
    );
    frisch.insert(
        Gemuese::Blumenkohl.into(),
        vec![May, June, July, August, September, October],
    );
    frisch.insert(
        Gemuese::GrueneBohnen.into(),
        vec![July, August, September, October],
    );
    frisch.insert(Gemuese::DickeBohnen.into(), vec![June, July, August]);
    frisch.insert(
        Gemuese::Brokkoli.into(),
        vec![June, July, August, September, October],
    );
    frisch.insert(
        Gemuese::Butterrueben.into(),
        vec![August, September, October, November, December],
    );
    frisch.insert(
        Gemuese::Champignons.into(),
        vec![
            January, February, March, April, May, June, July, August, September, October, November,
            December,
        ],
    );
    frisch.insert(Gemuese::Erbsen.into(), vec![June, July, August]);
    frisch.insert(
        Gemuese::Fenchel.into(),
        vec![June, July, August, September, October, November],
    );
    frisch.insert(
        Gemuese::Gruenkohl.into(),
        vec![November, December, January, February],
    );
    frisch.insert(
        Gemuese::Gurke.into(),
        vec![June, July, August, September, October],
    );
    frisch.insert(
        Gemuese::Kartoffeln.into(),
        vec![June, July, August, September, October],
    );
    frisch.insert(
        Gemuese::Kohlrabi.into(),
        vec![May, June, July, August, September, October],
    );
    frisch.insert(
        Gemuese::Kuerbis.into(),
        vec![August, September, October, November],
    );
    frisch.insert(
        Gemuese::Lauch.into(),
        vec![
            January, February, March, April, May, June, July, August, September, October, November,
            December,
        ],
    );
    frisch.insert(
        Gemuese::Fruehlingszwiebeln.into(),
        vec![May, June, July, August, September, October],
    );
    frisch.insert(Gemuese::Mais.into(), vec![August, September, October]);
    frisch.insert(
        Gemuese::Mangold.into(),
        vec![May, June, July, August, September, October],
    );
    frisch.insert(
        Gemuese::Karotten.into(),
        vec![June, July, August, September, October],
    );
    frisch.insert(
        Gemuese::Paprika.into(),
        vec![July, August, September, October],
    );
    frisch.insert(Gemuese::Pastinaken.into(), vec![]);
    frisch.insert(
        Gemuese::Radieschen.into(),
        vec![May, June, July, August, September, October],
    );
    frisch.insert(
        Gemuese::Rosenkohl.into(),
        vec![October, November, December, January, February, March],
    );
    frisch.insert(
        Gemuese::RoteBeete.into(),
        vec![July, August, September, October, November],
    );
    frisch.insert(
        Gemuese::Rotkohl.into(),
        vec![June, July, August, September, October, November],
    );
    frisch.insert(
        Gemuese::Schwarzwurzeln.into(),
        vec![October, November, December, January, February],
    );
    frisch.insert(Gemuese::Spargel.into(), vec![April, May, June]);
    frisch.insert(
        Gemuese::Spinat.into(),
        vec![March, April, May, September, October, November],
    );
    frisch.insert(Gemuese::Spitzkohl.into(), vec![May, June]);
    frisch.insert(
        Gemuese::Staudensellerie.into(),
        vec![July, August, September, October],
    );
    frisch.insert(
        Gemuese::Steckrueben.into(),
        vec![August, September, October, November, December],
    );
    frisch.insert(
        Gemuese::Tomaten.into(),
        vec![July, August, September, October],
    );
    frisch.insert(
        Gemuese::Topinambur.into(),
        vec![October, November, December, January, February, March],
    );
    frisch.insert(
        Gemuese::Weisskohl.into(),
        vec![June, July, August, September, October, November],
    );
    frisch.insert(
        Gemuese::Wirsing.into(),
        vec![
            May, June, July, August, September, October, November, December, January, February,
        ],
    );
    frisch.insert(
        Gemuese::Zucchini.into(),
        vec![June, July, August, September, October],
    );
    frisch.insert(Gemuese::Zuckerschoten.into(), vec![June, July, August]);
    frisch.insert(
        Gemuese::Zwiebeln.into(),
        vec![July, August, September, October],
    );

    lagerware.insert(
        Gemuese::Butterrueben.into(),
        vec![January, February, March, April],
    );
    lagerware.insert(
        Gemuese::Kartoffeln.into(),
        vec![November, December, January, February, March, April, May],
    );
    lagerware.insert(Gemuese::Kuerbis.into(), vec![December, January, February]);
    lagerware.insert(
        Gemuese::Karotten.into(),
        vec![November, December, January, February, March, April, May],
    );
    lagerware.insert(Gemuese::Pastinaken.into(), vec![April]);
    lagerware.insert(
        Gemuese::RoteBeete.into(),
        vec![December, January, February, March, April],
    );
    lagerware.insert(
        Gemuese::Rotkohl.into(),
        vec![December, January, February, March, April, May],
    );
    lagerware.insert(Gemuese::Steckrueben.into(), vec![January, February, March]);
    lagerware.insert(
        Gemuese::Weisskohl.into(),
        vec![December, January, February, March, April],
    );
    lagerware.insert(Gemuese::Wirsing.into(), vec![March]);
    lagerware.insert(
        Gemuese::Zwiebeln.into(),
        vec![
            November, December, January, February, March, April, May, June,
        ],
    );
}

#[inline]
pub fn fill_obst<T>(frisch: &mut HashMap<T, Vec<Month>>, lagerware: &mut HashMap<T, Vec<Month>>)
where
    T: Eq + Hash + From<Obst>,
{
    frisch.insert(
        Obst::Apfel.into(),
        vec![August, September, October, November],
    );
    frisch.insert(Obst::Aprikose.into(), vec![July, August]);
    frisch.insert(Obst::Birne.into(), vec![August, September, October]);
    frisch.insert(Obst::Blaubeeren.into(), vec![June, July, August, September]);
    frisch.insert(Obst::Brombeeren.into(), vec![July, August, September]);
    frisch.insert(Obst::Erdbeeren.into(), vec![May, June, July]);
    frisch.insert(Obst::Himbeeren.into(), vec![June, July, August]);
    frisch.insert(Obst::Holunderbeeren.into(), vec![September, October]);
    frisch.insert(Obst::Johannisbeeren.into(), vec![June, July, August]);
    frisch.insert(Obst::Kirschen.into(), vec![June, July, August]);
    frisch.insert(Obst::Mirabellen.into(), vec![July, August, September]);
    frisch.insert(Obst::Pflaumen.into(), vec![July, August, September]);
    frisch.insert(Obst::Quitten.into(), vec![September, October, November]);
    frisch.insert(Obst::Rhabarber.into(), vec![April, May, June]);
    frisch.insert(Obst::Stachelbeeren.into(), vec![June, July, August]);
    frisch.insert(Obst::Wassermelone.into(), vec![August, September]);
    frisch.insert(Obst::Weintrauben.into(), vec![September, October]);
    frisch.insert(
        Obst::Zwetschgen.into(),
        vec![July, August, September, October],
    );

    lagerware.insert(
        Obst::Apfel.into(),
        vec![December, January, February, March, April, May],
    );
    lagerware.insert(Obst::Birne.into(), vec![November, December]);
}

#[inline]
pub fn fill_salat<T>(frisch: &mut HashMap<T, Vec<Month>>, lagerware: &mut HashMap<T, Vec<Month>>)
where
    T: Eq + Hash + From<Salat>,
{
    frisch.insert(
        Salat::Batavia.into(),
        vec![May, June, July, August, September],
    );
    frisch.insert(
        Salat::Chicoree.into(),
        vec![October, November, December, January, February, March, April],
    );
    frisch.insert(
        Salat::Eichblattsalat.into(),
        vec![May, June, July, August, September, October],
    );
    frisch.insert(
        Salat::Eisbergsalat.into(),
        vec![June, July, August, September, October],
    );
    frisch.insert(
        Salat::Endiviensalat.into(),
        vec![
            May, June, July, August, September, October, November, December,
        ],
    );
    frisch.insert(
        Salat::Feldsalat.into(),
        vec![October, November, December, January, February, March, April],
    );
    frisch.insert(
        Salat::Kopfsalat.into(),
        vec![May, June, July, August, September, October],
    );
    frisch.insert(
        Salat::LolloRosso.into(),
        vec![May, June, July, August, September, October],
    );
    frisch.insert(
        Salat::Portulak.into(),
        vec![
            July, August, September, October, November, December, January, February, March, April,
        ],
    );
    frisch.insert(
        Salat::Radicchio.into(),
        vec![August, September, October, November],
    );
    frisch.insert(
        Salat::Rucola.into(),
        vec![April, May, June, July, August, September],
    );

    lagerware.insert(Salat::Radicchio.into(), vec![December, January, February]);
}
