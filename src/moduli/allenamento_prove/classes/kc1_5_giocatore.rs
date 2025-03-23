use rand::Rng;

#[derive (Debug)]
pub struct Giocatore{
    pub primo_tiro:i32,
    pub punteggio: i32,
    pub turno: TurnoGiocatore,
    pub vittoria: VittoriaGiocatore
}
#[derive(Debug, PartialEq)]
pub enum TurnoGiocatore{
    Primo,
    Secondo
}
#[derive(Debug, PartialEq)]
pub enum VittoriaGiocatore{
    Vinto, Perso, Tbd
}

impl Giocatore{
    pub fn new(turno:TurnoGiocatore) -> Giocatore {
        return Giocatore {
            primo_tiro:0,
            punteggio:0,
            vittoria:VittoriaGiocatore::Tbd,
            turno,
        }
    }
    
    pub fn tiro_dadi(&mut self){
        let mut rand = rand::rng();
        let tiro= rand.random_range(1..=6)+ rand.random_range(1..=6);
        println!("Il {:?} giocatore ha tirato: {}", self.turno, tiro );
        if self.primo_tiro==0{
            self.primo_tiro=tiro;
            if tiro==11 || tiro==7{
                self.vittoria=VittoriaGiocatore::Vinto;
            }else if tiro==2 || tiro==3 || tiro==12{
                self.vittoria=VittoriaGiocatore::Perso;
            }else if tiro==4 || tiro==5|| tiro==6|| tiro==8|| tiro==9 || tiro==10{
                // self.punteggio+=tiro;
            }
            return;
        }
        self.punteggio+=tiro;
        if self.punteggio==7 || self.punteggio>self.primo_tiro{
            self.vittoria=VittoriaGiocatore::Perso;
        }else if self.punteggio == self.primo_tiro{
            self.vittoria=VittoriaGiocatore::Vinto;
        }
    }
}