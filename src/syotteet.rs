use sdl2::keyboard::Scancode;
use sdl2::EventPump;

#[derive(Default)]
pub struct Syotteet {
    /// Lista tarkkailtavista näppäimistä
    tarkkailtavat_nappaimet: Vec<Nappain>,
}

impl Syotteet {
    /// Luo uuden tyhjän syötteet otuksen
    pub fn new() -> Self {
        Syotteet {
            tarkkailtavat_nappaimet: Vec::new(),
        }
    }

    /// Lisää uudeltavan näppäimen listaan, jos sitä ei ole jo lisätty
    /// # Arguments
    /// * `events` - Tapahtumalista, josta tarkastetaan, onko näppäin luotaessa jo pohjassa
    /// * `nappain_koodi` - Lisättävän näppäimen koodi
    pub fn lisaa_nappain(&mut self, events: &EventPump, nappain_koodi: Scancode) {
        // Tarkastetaan löytyykö valmiiksi kyseistä annetulle näppäimelle kuuntelijaa
        let sisaltaa = self
            .tarkkailtavat_nappaimet
            .iter()
            .any(|x| x.nappain == nappain_koodi);
        // Lisätään kuunneltava, jos sitä ei ennestään ole.
        if !sisaltaa {
            self.tarkkailtavat_nappaimet
                .push(Nappain::new(&events, nappain_koodi));
        }
    }

    /// Antaa etsittävän näppäimen tilan, jos annettua näppäintä ylipäätään kuunnellaan
    /// # Arguments
    /// * `nappain_koodi` - Etsittävän näppäimen koodi
    pub fn anna_nappaimen_tila(&self, nappain_koodi: Scancode) -> Option<Tila> {
        self.tarkkailtavat_nappaimet
            .iter()
            .find(|x| x.nappain == nappain_koodi)
            .map(|x| x.tila)
    }

    /// Päivittää kaikkien näppäinten tilan
    /// # Arguments
    /// * `events` - Tapahtumalista, josta tarkastetaan, onko näppäin luotaessa jo pohjassa
    pub fn paivita_nappainten_tilat(&mut self, events: &EventPump) {
        for nappain in self.tarkkailtavat_nappaimet.iter_mut() {
            nappain.paivita_tila(&events);
        }
    }
}

/// Näppäin, jolla on tieto omasta tilastaan
struct Nappain {
    /// Tarkkailtavan näppäimen koodi
    pub nappain: Scancode,
    /// Tarkkailtavan näppäimen tila
    pub tila: Tila,
}

impl Nappain {
    /// Luo uuden näppäimen asettaen sille tilan valmiiksi. Luotava näppäin on oletuksena pysynyt luodussa tilassa.
    /// # Arguments
    /// * `events` - Tapahtumalista, josta tarkastetaan, onko näppäin luotaessa jo pohjassa
    /// * `nappain_koodi` - Luotavan näppäimen koodi
    fn new(events: &EventPump, nappain_koodi: Scancode) -> Self {
        if events.keyboard_state().is_scancode_pressed(nappain_koodi) {
            Nappain {
                nappain: nappain_koodi,
                tila: Tila::Pohjassa(Muutos::Pysynyt),
            }
        } else {
            Nappain {
                nappain: nappain_koodi,
                tila: Tila::EiPohjassa(Muutos::Pysynyt),
            }
        }
    }

    /// Päivittää näppäimen tilan
    /// # Arguments
    /// * `events` - Tapahtumalista, josta tarkastetaan, onko näppäin luotaessa jo pohjassa
    fn paivita_tila(&mut self, events: &EventPump) {
        match (
            events.keyboard_state().is_scancode_pressed(self.nappain),
            self.tila.pohjassa(),
        ) {
            (false, true) => self.tila = Tila::EiPohjassa(Muutos::Muuttunut),
            (false, false) => self.tila = Tila::EiPohjassa(Muutos::Pysynyt),
            (true, true) => self.tila = Tila::Pohjassa(Muutos::Pysynyt),
            (true, false) => self.tila = Tila::Pohjassa(Muutos::Muuttunut),
        }
    }
}

/// Kuvaa näppäimen tilaa eli onko se pohjassa vai ei.
#[derive(Copy, Clone)]
pub enum Tila {
    /// Näppäin on pohjassa
    Pohjassa(Muutos),
    /// Näppäin ei ole pohjassa
    EiPohjassa(Muutos),
}

impl Tila {
    pub fn pohjassa(self) -> bool {
        match self {
            Tila::Pohjassa(_) => true,
            Tila::EiPohjassa(_) => false,
        }
    }
}

/// Kuvaa onko näppäimen tila muuttunut viime syötteen tarkistukselta
#[derive(Copy, Clone)]
pub enum Muutos {
    /// Näppäimen tila on muuttunut. Esim. juuri vapautettu pohjasta.
    Muuttunut,
    /// Näppäimen tila on pysynyt samana.
    Pysynyt,
}
