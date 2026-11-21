// TO DO mutation file

use std::io;
use rand::RngExt;
use crate::progamme::{Programme, PROG_NB_LINE};

pub trait Mutator {
    fn mutate<'a>(&mut self, prog: &'a mut Programme) -> Result<&'a mut Programme, io::Error>;
}

pub struct UniformMutation {
    pub taux: f32,
}

impl Mutator for UniformMutation{
fn mutate<'a>(&mut self, prog: &'a mut Programme) -> Result<&'a mut Programme, io::Error> {
        if self.taux < 0.0 || self.taux > 1.0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Le taux doit être compris entre 0.0 et 1.0",
            ));
        }

        let mut rng = rand::rng();

        for bit in 0..64 {
            if rng.random_range(0.0..1.0) < self.taux {
                prog.prog64 ^= 1u64 << bit;            // XOR 
            }
        }

        Ok(prog)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn creer_prog(bits: u64) -> Programme {
        Programme::init(String::from("Test_Mutation"), bits)
    }

    #[test]
    fn test_taux_zero_aucune_mutation() {
        let valeur_initiale = 0b101010_110011_001100_u64;
        let mut prog = creer_prog(valeur_initiale);
        let mut mutator = UniformMutation { taux: 0.0 };

        mutator.mutate(&mut prog).unwrap();

        assert_eq!(prog.prog64, valeur_initiale);
    }

    #[test]
    fn test_taux_un_inverse_tous_les_bits() {
        let mut prog = creer_prog(0);
        let mut mutator = UniformMutation { taux: 1.0 };

        mutator.mutate(&mut prog).unwrap();

        assert_eq!(prog.prog64, u64::MAX);
        assert_eq!(prog.prog64.count_ones(), 64);
    }

    #[test]
    fn test_taux_hors_limites_renvoie_erreur() {
        let mut prog = creer_prog(0);

        let mut mutator_trop_grand = UniformMutation { taux: 1.1 };
        assert!(mutator_trop_grand.mutate(&mut prog).is_err());

        let mut mutator_negatif = UniformMutation { taux: -0.5 };
        assert!(mutator_negatif.mutate(&mut prog).is_err());
    }

    #[test]
    fn test_double_mutation_taux_un_revient_a_l_initial() {
        let valeur_initiale = 0x0ABC_DEF1_2345_6789;
        let mut prog = creer_prog(valeur_initiale);
        let mut mutator = UniformMutation { taux: 1.0 };

        mutator.mutate(&mut prog).unwrap();
        mutator.mutate(&mut prog).unwrap();

        assert_eq!(prog.prog64, valeur_initiale);
    }

    #[test]
    fn test_distribution_statistique_taux_50_pourcent() {
        let mut mutator = UniformMutation { taux: 0.5 };
        let nb_essais = 1000;
        let mut total_bits_mutes = 0;

        for _ in 0..nb_essais {
            let mut prog = creer_prog(0);
            mutator.mutate(&mut prog).unwrap();
            total_bits_mutes += prog.prog64.count_ones();
        }

        assert!(
            (30_000..=34_000).contains(&total_bits_mutes),
            "Nombre de bits mutés anormal : {}",
            total_bits_mutes
        );
    }
}