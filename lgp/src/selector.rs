//TODO: différente strat de meilleur programme

use std::io;

use crate::progamme::Programme;
use ordered_float::OrderedFloat;

//Use vectors to return owned programs
pub trait Selector {
    fn select(&mut self, populace: &mut [Programme],) -> Result<Vec<Programme>, io::Error>;
}

pub struct Truncation{
    pub portion: f32
}

impl Selector for Truncation{
    fn select(&mut self, populace: &mut [Programme],) -> Result<Vec<Programme>, io::Error> {
    if self.portion < 0.0 || self.portion > 1.0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Portion must be between 0.0 and 1.0",
        ));
    }

    populace.sort_by_key(|p| OrderedFloat(p.score));
    populace.reverse();

    let trunc_index = (self.portion * populace.len() as f32) as usize;

    Ok(populace[..trunc_index].to_vec())
}
}



#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Check if the truncation is well done
     */
    #[test]
    fn truncate(){
        use rand::{RngExt};
        //Init the population
        let mut rng = rand::rng();
        let mut population: [Programme; 100] = std::array::from_fn( |i|{
            Programme{
                nom:petname::petname(1, "-").unwrap(),
                prog64: 0,
                score: rng.random_range(0.0..100.0)
            }
        });

        //Process to the selection
        let mut selection_method = Truncation{portion:0.1};

        let selected = selection_method.select(&mut population).unwrap();

        //Tests
        assert_eq!(selected.len(), 10);
        assert!(selected.is_sorted_by_key(|p| OrderedFloat(-p.score)));
    }

    /**
     * Check if an bad portion value is well taken care of
     */
    #[test]
    fn trunc_oob(){
        use rand::{RngExt};
        //Init the population
        let mut rng = rand::rng();
        let mut population: [Programme; 100] = std::array::from_fn( |i|{
            Programme{
                nom:petname::petname(1, "-").unwrap(),
                prog64: 0,
                score: rng.random_range(0.0..100.0)
            }
        });

        //Process to the selection
        let mut selection_method = Truncation{portion:1.1};

        assert!(selection_method.select(&mut population).is_err());

                //Process to the selection
        let mut selection_method = Truncation{portion:-100.};

        assert!(selection_method.select(&mut population).is_err());
    }
}