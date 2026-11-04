#[derive(Debug, Clone, Copy)]
pub struct Programme {
    pub nom: &'static str,
    pub prog64: u64,
    pub score: f32,
}

impl Programme {
    //TODO: faire init default et random

    fn getOp(prog: u64, ligne: u64) -> u8{
        //TODO
        0
    }

    fn getOut(prog: u64, ligne: u64) -> u8{
        //TODO
        0
    }

    fn getInA(prog: u64, ligne: u64) -> u8{
        //TODO
        0
    }

    fn getInB(prog: u64, ligne: u64) -> u8{
        //TODO
        0
    }
}