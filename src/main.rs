use crate::chain::Chain;
use crate::vote::Vote;

mod block;
mod vote;
mod hashable;
mod chain;

fn main() {
    let mut chain = Chain::new();

    let vote1 = Vote::new(1, "Candidate 1");
    chain.add_block(vote1);

    let vote2 = Vote::new(2, "Candidate 2");
    chain.add_block(vote2);

    let vote3 = Vote::new(3, "Candidate 3");
    chain.add_block(vote3);
    
    assert!(chain.validate(), "Chain validation failed!");

    chain.blocks[1].vote = Vote::new(2, "Candidate 3");
    assert!(chain.validate(), "Chain validation did not detect tampering!");
}
