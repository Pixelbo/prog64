use crate::progamme::{Programme, PROG_NB_LINE};
//let mut regs: [f32; 4];

/**
 * Une ligne de programme:
 * OUT  OP IN1 IN2
 *  2b  2b  3b   3b
 */

//Nb de bits dans une ligne du programme 2+2+3+3


// Décodage de l'opération
#[derive(Debug, PartialEq, Clone, Copy)]
enum OpCode {
    Add   = 0,
    Minus = 1,
    Mult  = 2,
    Div   = 3,
}

impl OpCode {
    fn from_i16(value: i16) -> Option<Self> {
        match value {
            0 => Some(OpCode::Add),
            1 => Some(OpCode::Minus),
            2 => Some(OpCode::Mult),
            3 => Some(OpCode::Div),
            _ => None,
        }
    }
}

// Traitement des données
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operand {
    Reg(usize),
    Input(usize),
}

impl Operand {

    pub fn from_i16(valeur_brute: i16) -> Self {
        // 100 = 4 (décimal) -> Registre 0
        // 000 = 0 (décimal) -> Input 0
        
        let est_registre = (valeur_brute & 0b100) != 0; 
        let index = (valeur_brute & 0b011) as usize;

        if est_registre {
            Operand::Reg(index)
        } else {
            Operand::Input(index)
        }
    }
}

// Execution des lignes 

pub struct Machine {
    pub r: [i32; 4], // Registres
    pub i: [i32; 4], // Entrées
}

impl Machine {
    fn read_val(&self, operand: Operand)-> i32{
        match operand {
            Operand::Reg(idx) => self.r[idx],
            Operand::Input(idx) => self.i[idx],
        }
    }
    fn ecrire_valeur(&mut self, operand: Operand, valeur: i32) {
        match operand {
            Operand::Reg(idx) => self.r[idx] = valeur,
            Operand::Input(_) => {}
        }
    }

    pub fn executer_ligne(&mut self, prog: &Programme, ligne: u64) {
        
        // Val from programme
        let encod_op = prog.getOp(ligne);
        let encod_out = prog.getOut(ligne);
        let encod_ina = prog.getInA(ligne);
        let encod_inb = prog.getInB(ligne);

        if encod_op == -1 {
            return;
        }
        
        let op = OpCode::from_i16(encod_op).expect("OpCode invalide");
        let val_a = Operand::from_i16(encod_ina);
        let val_b = Operand::from_i16(encod_inb);
        let dest = encod_out as usize;

        let read_valA = self.read_val(val_a);
        let read_valB = self.read_val(val_b);

        let resultat = match op {
            OpCode::Add   => read_valA + read_valB,
            OpCode::Minus => read_valA - read_valB,
            OpCode::Mult  => read_valA * read_valB,
            OpCode::Div   => {
                if read_valB == 0 {
                    0
                } else {
                    read_valA / read_valB
                }
            }         
        };

        self.r[dest] = resultat;
    }
    
    pub fn executer_programme(&mut self, prog: &Programme) {
        for ligne in 0..PROG_NB_LINE {
            self.executer_ligne(prog, ligne);
        }
    }
}

// ==========================================
// TESTS AUTOMATIQUES 
// ==========================================

// Pour executer les test cmd : cargo test

#[cfg(test)]
mod tests {
    use super::*;
    
    // 1xx = Registre, 0xx = Input 

    #[test]
    fn test_execution_addition() {
        // Scénario : R[1] = I[0] + R[2]
        // OUT = R[1]   -> 01
        // OP  = Add    -> 00
        // INA = I[0]   -> 000 (0 pour Input, 00 pour l'index 0)
        // INB = R[2]   -> 110 (1 pour Registre, 10 pour l'index 2)
        
        let prog_64_bits = 0b01_00_000_110_u64; 
        let prog = crate::progamme::Programme::init(String::from("Test_Add"), prog_64_bits);

        let mut machine = Machine {
            r: [0, 0, 15, 0],   // R[2] = 15
            i: [25, 0, 0, 0],   // I[0] = 25
        };

        machine.executer_ligne(&prog, 0);

        assert_eq!(machine.r[1], 40);
    }

    #[test]
    fn test_execution_soustraction() {
        // Scénario : R[2] = R[0] - I[1]
        // OUT = R[2]   -> 10
        // OP  = Minus  -> 01
        // INA = R[0]   -> 100 (1 pour Registre, 00 pour l'index 0)
        // INB = I[1]   -> 001 (0 pour Input, 01 pour l'index 1)
        
        let prog_64_bits = 0b10_01_100_001_u64; 
        let prog = crate::progamme::Programme::init(String::from("Test_Minus"), prog_64_bits);

        let mut machine = Machine {
            r: [50, 0, 0, 0],   // R[0] = 50
            i: [0, 12, 0, 0],   // I[1] = 12
        };

        machine.executer_ligne(&prog, 0);

        assert_eq!(machine.r[2], 38);
    }

    #[test]
    fn test_execution_multiplication() {
        // Scénario : R[3] = I[3] * I[2]
        // OUT = R[3]   -> 11
        // OP  = Mult   -> 10
        // INA = I[3]   -> 011 (0 pour Input, 11 pour l'index 3)
        // INB = I[2]   -> 010 (0 pour Input, 10 pour l'index 2)
        
        let prog_64_bits = 0b11_10_011_010_u64; 
        let prog = crate::progamme::Programme::init(String::from("Test_Mult"), prog_64_bits);

        let mut machine = Machine {
            r: [0, 0, 0, 0],    // R[3] va recevoir le résultat
            i: [0, 0, 6, 7],    // I[2] = 6, I[3] = 7
        };

        machine.executer_ligne(&prog, 0);

        assert_eq!(machine.r[3], 42);
    }

    #[test]
    fn test_execution_division() {
        // Scénario : R[0] = I[2] / R[3]
        // OUT = R[0]   -> 00
        // OP  = Div    -> 11
        // INA = I[2]   -> 010 (0 pour Input, 10 pour l'index 2)
        // INB = R[3]   -> 111 (1 pour Registre, 11 pour l'index 3)
        
        let prog_64_bits = 0b00_11_010_111_u64; 
        let prog = crate::progamme::Programme::init(String::from("Test_Div"), prog_64_bits);

        let mut machine = Machine {
            r: [0, 0, 0, 2],    // R[3] = 2
            i: [0, 0, 10, 0],   // I[2] = 10
        };

        machine.executer_ligne(&prog, 0);

        assert_eq!(machine.r[0], 5);
        assert_eq!(machine.r[3], 2);    
    }

    #[test]
    fn test_execution_programme_complet() {

        // Ligne 0 : R[0] = I[0] + I[1] -> 2 + 3 = 5
        let l0: u64 = 0b00_00_000_001; 
        
        // Ligne 1 : R[1] = I[2] * R[0] -> 5 * 5 = 25 (utilise le R[0] calculé juste au-dessus)
        let l1: u64 = 0b01_10_010_100; 

        // Ligne 2 : R[2] = R[1] - I[3] -> 25 - 10 = 15
        let l2: u64 = 0b10_01_101_011; 

        // Ligne 3 : R[3] = R[2] / I[0] -> 15 / 2 = 7
        let l3: u64 = 0b11_11_110_000; 

        // Ligne 4 : R[0] = R[3] + R[3] -> 7 + 7 = 14 (Écrase l'ancienne valeur de R[0])
        let l4: u64 = 0b00_00_111_111; 

        // Ligne 5 : R[1] = R[0] * I[0] -> 14 * 2 = 28 (Écrase l'ancienne valeur de R[1])
        let l5: u64 = 0b01_10_100_000; 

        // Assemblage
        let prog_64_bits = (l5 << 50) | (l4 << 40) | (l3 << 30) | (l2 << 20) | (l1 << 10) | l0;
        
        let prog = crate::progamme::Programme::init(String::from("Test_Complet"), prog_64_bits);

        // Initialisation de la machine
        let mut machine = Machine {
            r: [0, 0, 0, 0],
            i: [2, 3, 5, 10], // I[0]=2, I[1]=3, I[2]=5, I[3]=10
        };

        // Exécution de TOUT le programme 
        machine.executer_programme(&prog);

        // Vérification de l'état final des 4 registres
        assert_eq!(machine.r[0], 14);
        assert_eq!(machine.r[1], 28);
        assert_eq!(machine.r[2], 15);
        assert_eq!(machine.r[3], 7);
    }

}