//TODO: différente strat de meilleur programme

use std::io;

use crate::progamme::Programme;

pub trait Selector {
    fn select<'a>(&mut self, populace: &'a mut [Programme]) -> Result<&'a mut [Programme], io::Error>;
}

pub struct Truncation{
    pub portion: f32
}

impl Selector for Truncation{
    fn select<'a>(&mut self, populace: &'a mut [Programme]) -> Result<&'a mut [Programme], io::Error>{
        if self.portion < 0.0 || self.portion > 1.0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Portion must be between 0.0 and 1.0",
            ));
        }

        use ordered_float::OrderedFloat;

        populace.sort_by_key(|p| OrderedFloat(p.score));
        populace.reverse();

        let trunc_index = self.portion * (populace.len() as f32);

        //Split the slice
        let (selected, _ ) =  populace.split_at_mut(trunc_index as usize);

        
        Ok(selected)
    }
}