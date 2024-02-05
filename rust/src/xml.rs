use crate::api::worker::Worker;
use chrono::prelude::*;
use simple_xml_serialize::XMLElement;

pub fn render_xml(workers: Vec<Worker>) -> String {
    let pad = |i: usize, size: usize| -> String {
        let mut istr = i.to_string();
        while istr.len() < size {
            istr = format!("0{}", istr);
        }
        istr
    };

    let mut mezok = XMLElement::new("mezok");

    mezok.add_elements(vec![
        XMLElement::new("mezo")
            .attr("eazon", "0A0001C0001AA")
            .text("23127182215"),
        XMLElement::new("mezo")
            .attr("eazon", "0A0001E001A")
            .text("Mezei Istvánné"),
        XMLElement::new("mezo")
            .attr("eazon", "0A0001E002A")
            .text("06305236153"),
    ]);

    for (i, worker) in workers.iter().enumerate() {
        mezok.add_elements(vec![
            // Name
            XMLElement::new("mezo")
                .attr("eazon", &format!("0B{}C0001AA", pad(i + 1, 4)))
                .text(&worker.name),
            // Taxnumber
            XMLElement::new("mezo")
                .attr("eazon", &format!("0B{}C0002AA", pad(i + 1, 4)))
                .text(&worker.taxnumber),
            // TAJ
            XMLElement::new("mezo")
                .attr("eazon", &format!("0B{}C0003AA", pad(i + 1, 4)))
                .text(&worker.taj), // TODO: TAJ szám valamiért a korábbi algoritmusban 9-re van paddolva. Miért?
            // TYPE MODE U => Új bejelentés
            XMLElement::new("mezo")
                .attr("eazon", &format!("0B{}D0005AA", pad(i + 1, 4)))
                .text("U"),
            // TYPE 03
            XMLElement::new("mezo")
                .attr("eazon", &format!("0B{}D0007AA", pad(i + 1, 4)))
                .text("03"),
            // Record ID
            XMLElement::new("mezo")
                .attr("eazon", &format!("0B{}A001A", pad(i + 1, 4)))
                .text(&(i + 1).to_string()),
            // Time period till valid
            XMLElement::new("mezo")
                .attr("eazon", &format!("0B{}D0009AA", pad(i + 1, 4)))
                .text(1),
            // Date report
            XMLElement::new("mezo")
                .attr("eazon", &format!("0B{}D0008AA", pad(i + 1, 4)))
                .text(&format!(
                    "{}{}{}",
                    Utc::now().year(),
                    &pad(Utc::now().month() as usize, 2),
                    &pad(Utc::now().day() as usize, 2)
                )),
        ]);
    }

    let mut root = XMLElement::new("nyomtatvanyok")
        .attr("xmlns", "http://www.apeh.hu/abev/nyomtatvanyok/2005/01");

    let mut nyomtatvany = XMLElement::new("nyomtatvany");

    let mut adozo = XMLElement::new("adozo");

    adozo.add_element(XMLElement::new("adoszam").text("23127182215"));

    let mut nyomtatvanyinformacio = XMLElement::new("nyomtatvanyinformacio");
    nyomtatvanyinformacio.add_elements(vec![
        XMLElement::new("nyomtatvanyazonosito")
            .text(format!("{}T1042E", Utc::now().naive_local().format("%y"))),
        XMLElement::new("nyomtatvanyverzio").text("1.0"),
        adozo,
        XMLElement::new("megjegyzes").text("Bejelentés"),
    ]);

    nyomtatvany.add_element(nyomtatvanyinformacio);
    nyomtatvany.add_element(mezok);
    root.add_element(nyomtatvany);

    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>{}",
        root.to_string()
    )
    // root.to_string_pretty_prolog("\n", "   ")

    // root.to_string()
}
