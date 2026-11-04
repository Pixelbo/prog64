use env::{cartpole::CartPole, Environment, pcgrng::PCGRng};

use crate::progamme::Programme;


pub mod interpretor;
pub mod progamme;
const NB_GENERATIONS: usize = 100;
const NB_POPULATION: usize = 100;



fn main() {
    let seed = 1;
    let mut rng = PCGRng::seed(seed);
    let mut env = CartPole::default();
    let mut total_reward = 0.0;

    let default_temp = Programme {nom: "OuiOui", prog64: 0, score: 0.};
    let mut population: [Programme; NB_POPULATION] = [default_temp; NB_POPULATION];

    let mut iterations = 0;

    println!("I\taction\treward\tdone\tx\tx_dot\ttheta\ttheta_dot");
    //Todo: 2 boulces NB_GEN * NB_POP
    loop {
        
        let action: f32 = (rng.next_u32() % 4) as f32;

        
        let (curr_state, reward, done) = env.step(action, &mut rng);

        println!("{}\t{}\t{}\t{}\t{:.2}\t{:.2}\t{:.2}\t{:.2}",iterations ,action, reward, done, curr_state[0], curr_state[1], curr_state[2], curr_state[3]);

        total_reward += reward;
        iterations+=1;
        if done {
            break;
        }
    }
    //Faire logging et recap
    println!("Total reward: {}", total_reward)
}
