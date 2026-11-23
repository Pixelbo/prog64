use std::process::exit;

use env::{cartpole::CartPole, Environment, pcgrng::PCGRng};

use crate::{interpretor::Machine, progamme::Programme, selector::{Selector, Truncation}};


pub mod interpretor;
pub mod progamme;
pub mod selector;

pub mod mutation;
const NB_GENERATIONS: usize = 100;
const NB_POPULATION: usize = 100;



fn main() {


    //Init the population
    let mut population: [Programme; NB_POPULATION] = std::array::from_fn( |i|{
        Programme::default()
    });

    
    let mut selection_method = Truncation{portion:0.1};

    let mut selected = selection_method.select(&mut population).unwrap();

    
    //Todo: 2 boulces NB_GEN * NB_POP
    
    for curr_prog in &mut population {
        //Evolution is Action -> Selection -> Mutation (repopulation) -> ...

        //Init the env and the cart
        let seed = 1;
        let mut rng = PCGRng::seed(seed);
        let mut env = CartPole::default();
        let mut total_reward = 0.0;

        //Init the machine (that hold the registers) and the program
        let mut machine = Machine {
            r: [0., 0., 0., 0.],
            i: [0., 0., 0., 0.],
        };

        let mut action: f32 = (rng.next_u32() % 4) as f32;
        loop {
            let (curr_state, reward, done) = env.step(action, &mut rng);

            machine.i = curr_state; //Copy the inputs
            machine.executer_programme(curr_prog);

            action = (machine.r[0] % 4.) as f32;

            total_reward += reward;
            if done {
                break;
            }
        }

        curr_prog.score += total_reward;
    }
    println!("{:?}", population);
}
