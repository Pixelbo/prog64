use rand::{RngExt};

/**
 * Une ligne de programme:
 * OUT  OP IN1 IN2
 *  2b  2b  3b   3b
 */
//TODO: better size mgmt ?
//Nb de bits dans une ligne du programme 2+2+3+3
const PROG_LINE_WIDTH: u64 = 10;
const PROG_LINE_OUT_MASK: u64 = 0b1100000000;
const PROG_LINE_OP_MASK: u64 =  0b0011000000;
const PROG_LINE_INA_MASK: u64 = 0b0000111000;
const PROG_LINE_INB_MASK: u64 = 0b0000000111;

const PROG_LINE_OUT_POS: u64 = 0x8;
const PROG_LINE_OP_POS: u64 =  0x6;
const PROG_LINE_INA_POS: u64 = 0x3;
const PROG_LINE_INB_POS: u64 = 0x0;

//Le nombre de lignes maximum dans un programme
pub const PROG_NB_LINE: u64 = 64/PROG_LINE_WIDTH;
//Padding de fin du u64
pub const PROG_END_PADDING: u64 = 64-PROG_NB_LINE*PROG_LINE_WIDTH;

#[derive(Debug, Clone)]
pub struct Programme {
    pub nom: String,
    pub prog64: u64,
    pub score: f32,
}

impl Programme {
    pub fn init(nom: String, prog: u64) -> Self{
        Programme { nom, prog64: prog, score: 0. }
    }

    pub fn getOp(&self, ligne: u64) -> Option<u16>{
        if ligne>PROG_NB_LINE {return None;}
        //Apply the mask
        let ope = (self.prog64>>(ligne*PROG_LINE_WIDTH)) & PROG_LINE_OP_MASK;
        //Convert the value
        (ope >> PROG_LINE_OP_POS).try_into().ok()
    }

    pub fn getOut(&self,ligne: u64) -> Option<u16>{
        if ligne>PROG_NB_LINE {return None;}
        //Apply the mask
        let out = (self.prog64>>(ligne*PROG_LINE_WIDTH)) & PROG_LINE_OUT_MASK;
        //Convert the cavaluelue
        (out >> PROG_LINE_OUT_POS).try_into().ok()
    }

    pub fn getInA(&self,ligne: u64) -> Option<u16>{
        if ligne>PROG_NB_LINE {return None;}
        //Apply the mask
        let ina = (self.prog64>>(ligne*PROG_LINE_WIDTH)) & PROG_LINE_INA_MASK;
        //Convert the value
        (ina >> PROG_LINE_INA_POS).try_into().ok()
    }

    pub fn getInB(&self,ligne: u64) -> Option<u16>{
        if ligne>PROG_NB_LINE {return None;}
        //Apply the mask
        let inb = (self.prog64>>(ligne*PROG_LINE_WIDTH)) & PROG_LINE_INB_MASK;
        //Convert the value
        (inb >> PROG_LINE_INB_POS).try_into().ok()
    }
}

impl Default for Programme {
    fn default() -> Self {
        let name = petname::petname(1, "-").unwrap();
        let mut rng = rand::rng();
        Programme {
            nom: name,
            prog64: rng.random_range(0..u64::MAX),
            score:0.,
        }
    }
}