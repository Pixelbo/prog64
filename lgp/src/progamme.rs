use std::fmt;

use rand::{RngExt};

use crate::interpretor::{OpCode, Operand::{self, Input, Reg}};

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

    pub fn get_op(&self, ligne: u64) -> Option<u16>{
        if ligne>PROG_NB_LINE {return None;}
        //Apply the mask
        let ope = (self.prog64>>(ligne*PROG_LINE_WIDTH)) & PROG_LINE_OP_MASK;
        //Convert the value
        (ope >> PROG_LINE_OP_POS).try_into().ok()
    }

    pub fn get_out(&self,ligne: u64) -> Option<u16>{
        if ligne>PROG_NB_LINE {return None;}
        //Apply the mask
        let out = (self.prog64>>(ligne*PROG_LINE_WIDTH)) & PROG_LINE_OUT_MASK;
        //Convert the cavaluelue
        (out >> PROG_LINE_OUT_POS).try_into().ok()
    }

    pub fn get_in_a(&self,ligne: u64) -> Option<u16>{
        if ligne>PROG_NB_LINE {return None;}
        //Apply the mask
        let ina = (self.prog64>>(ligne*PROG_LINE_WIDTH)) & PROG_LINE_INA_MASK;
        //Convert the value
        (ina >> PROG_LINE_INA_POS).try_into().ok()
    }

    pub fn get_in_b(&self,ligne: u64) -> Option<u16>{
        if ligne>PROG_NB_LINE {return None;}
        //Apply the mask
        let inb = (self.prog64>>(ligne*PROG_LINE_WIDTH)) & PROG_LINE_INB_MASK;
        //Convert the value
        (inb >> PROG_LINE_INB_POS).try_into().ok()
    }
}

impl Default for Programme {
    fn default() -> Self {
        let name = petname::petname(1, "-").unwrap() + "_000";
        let mut rng = rand::rng();
        Programme {
            nom: name,
            prog64: rng.random_range(0..u64::MAX),
            score:0.,
        }
    }
}

impl fmt::Display for Programme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "------------------------\n");
        let _ = write!(f, "Name: {} Score: {}\n", self.nom, self.score);
        //let _ = write!(f, "{:b}\n", self.prog64);

        //Print programm lines
        for i in 0..PROG_NB_LINE{
            //Get out op
            let operand_out = match self.get_out(i) {
                    Some(0) => "R0",
                    Some(1) => "R1",
                    Some(2) => "R2",
                    Some(3) => "R3",
                    Null  => ".."
            };

            let operand_ina = match Operand::from_u16(self.get_in_a(i)) {
                    Reg(result) => match result {
                        0 => "R0",
                        1 => "R1",
                        2 => "R2",
                        3 => "R3",
                        4..  => ".."
                    },
                    Input(result) => match result {
                        0 => "I0",
                        1 => "I1",
                        2 => "I2",
                        3 => "I3",
                        4..  => ".."
                    }
            };

            let operand_inb = match Operand::from_u16(self.get_in_b(i)) {
                    Reg(result) => match result {
                        0 => "R0",
                        1 => "R1",
                        2 => "R2",
                        3 => "R3",
                        4..  => ".."
                    },
                    Input(result) => match result {
                        0 => "I0",
                        1 => "I1",
                        2 => "I2",
                        3 => "I3",
                        4..  => ".."
                    }
            };

            //Get opcode
            let opcode = match OpCode::from_u16(self.get_op(i)) {
                Some(OpCode::Add) => "+",
                Some(OpCode::Minus) => "-",
                Some(OpCode::Mult) => "*",
                Some(OpCode::Div) => "/",
                Null => ".."
            };

            let _ = write!(f, "{} | {} = {} {} {}\n", i, operand_out, operand_ina, opcode, operand_inb);
        }

        write!(f, "------------------------\n")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn creer_prog(bits: u64) -> Programme {
        Programme::init(String::from("Test_Programme"), bits)
    }
    
    #[test]
    fn getter_tests(){
        let myProg = creer_prog(0b10_01_101_011_01_10_010_100);
        assert_eq!(myProg.get_out(0), Some(0b01));
        assert_eq!(myProg.get_out(1), Some(0b10));
        assert_eq!(myProg.get_op(0), Some(0b10));
        assert_eq!(myProg.get_op(1), Some(0b01));
        assert_eq!(myProg.get_in_a(0), Some(0b010));
        assert_eq!(myProg.get_in_a(1), Some(0b101));
        assert_eq!(myProg.get_in_b(0), Some(0b100));
        assert_eq!(myProg.get_in_b(1), Some(0b011));
    }

}