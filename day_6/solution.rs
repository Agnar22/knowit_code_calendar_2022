use std::fs;
use std::collections::HashMap;


fn file_to_hashmap(file_name: &str) -> HashMap<String, f64> {
    let mut actions = HashMap::new();
    let contents = fs::read_to_string(file_name).expect("Test");
    let lines: Vec<String> = contents.split("\n").map(|s: &str| s.to_string()).collect();
    for i in &lines {
        let kv: Vec<String> = i.split(':').map(str::to_string).collect();
        actions.insert(kv[0].clone(), kv[1].parse::<f64>().unwrap());
    }
    actions
}


fn file_to_lines(file_name: &str) -> Vec<String>  {
    let cont = fs::read_to_string(file_name).expect("Test");
    cont.split("\n").map(str::to_string).collect()
}


fn main() {
    let actions = file_to_hashmap("slemmehandlinger.txt");
    let votes = file_to_lines("stemmer.txt");
    let mut tot_votes: HashMap<String, f64> = HashMap::new();

    for vote in votes {
        let v: Vec<String> = vote.split(':').map(str::to_string).collect();
        let actions_commited: Vec<String> = v[0].split(',').map(str::to_string).collect();
        let mut vote_percentage: f64 = 1.0;
        for action_commited in actions_commited {
            vote_percentage = vote_percentage.min(
                match actions.get(&action_commited as &str) {
                    Some(score) => *score,
                    None => 1.0
                }
            );
        }
        let prev_vote = match tot_votes.get(&v[1] as &str) {
            Some(score) => *score,
            None => 0.0
        };
        tot_votes.insert(v[1].clone(), vote_percentage + prev_vote);
    }
    let mut votes_ordered: Vec<&f64> = tot_votes.values().collect::<Vec<_>>();
    votes_ordered.sort_by(|a, b| a.partial_cmp(b).unwrap());
    votes_ordered.reverse();
    println!("{:?}", (votes_ordered[0]-votes_ordered[1]).round() as i64);
}

