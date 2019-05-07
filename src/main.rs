extern crate sdl2;

use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::render::BlendMode;

pub mod animointi;
pub mod fysiikka;
pub mod maailma;
pub mod paivitys;
pub mod piirtaja;
pub mod silmukka;
pub mod spawneri;
pub mod syotteet;
pub mod tekoaly;
pub mod tormays;

mod peli;

use crate::paivitys::{Paivitys, YhdistettyPaivitys};
use crate::peli::pelihahmonpaivitys::PelihahmonPaivitys;
use crate::peli::pelinpaivitys::{
    AnimaatioidenPaivitys, FysiikanPaivitys, PeliAjanPaivitys, SpawnerinPaivitys, TekoalynPaivitys,
};
use crate::piirtaja::{Peruspiirtaja, Piirtaja};
use crate::silmukka::perussilmukka::Perussilmukka;
use crate::silmukka::saannollinensilmukka::SaannollinenSilmukka;
use crate::silmukka::Paasilmukka;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let window = video_subsystem
        .window("Peli", 1280, 720)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    // Sallii aplha värin.
    canvas.set_blend_mode(BlendMode::Blend);

    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));

    let texture_creator = canvas.texture_creator();
    let events = sdl_context.event_pump()?;
    let mut piirtaja = Peruspiirtaja::new(canvas)?;

    let seuraus = (0.2, 0.2);
    let zoomi = 1.0;
    println!("Seuraus on {0} ja {1}", seuraus.0, seuraus.1);
    println!("Zoomi on {}", zoomi);

    // Asetetaan piirtäjän asetukset
    piirtaja.aseta_kameran_seurauksen_etaisyys(seuraus)?;
    piirtaja.aseta_kameran_zoomi(zoomi);

    // Lisätään pelissä käytettävät kuvat
    let texture = texture_creator.load_texture("ympyra.png")?;
    piirtaja.lisaa_tekstuuri(texture, "ammus".to_string());

    let ajan_paivitys: &mut Paivitys = &mut PeliAjanPaivitys;
    let animaatioiden_paivitys: &mut Paivitys = &mut AnimaatioidenPaivitys;
    let fysiikan_paivitys: &mut Paivitys = &mut FysiikanPaivitys;
    let spawnerin_paivitys: &mut Paivitys = &mut SpawnerinPaivitys::new();
    let tekoalyn_paivitys: &mut Paivitys = &mut TekoalynPaivitys;
    let pelihahmon_paivitys: &mut Paivitys = &mut PelihahmonPaivitys;

    let mut paivitys: YhdistettyPaivitys;
    let mut silmukka: Box<Paasilmukka> = match 2 {
        1 => {
            paivitys = YhdistettyPaivitys::new(vec![
                ajan_paivitys,
                spawnerin_paivitys,
                tekoalyn_paivitys,
                pelihahmon_paivitys,
                fysiikan_paivitys,
                animaatioiden_paivitys,
            ]);
            Box::new(Perussilmukka::new(
                events,
                sdl_context,
                &mut piirtaja,
                &mut paivitys,
            ))
        }
        2 => {
            paivitys = YhdistettyPaivitys::new(vec![
                ajan_paivitys,
                spawnerin_paivitys,
                tekoalyn_paivitys,
                pelihahmon_paivitys,
                fysiikan_paivitys,
                animaatioiden_paivitys,
            ]);
            Box::new(SaannollinenSilmukka::new(
                events,
                sdl_context,
                &mut piirtaja,
                &mut paivitys,
                60, // Kuinka monta kertaa sekunnissa päivitetään
            ))
        }
        //
        _ => unreachable!(),
    };
    println!("{}", silmukka);
    silmukka.kaynnista_silmukka()
}
