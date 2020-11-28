#[allow(unused_imports)]
use super::prelude::*;
type Input = HashMap<String, (u32, Vec<String>)>;

pub fn input_generator(input: &str) -> Input {
    Regex::new(r"(?P<name>[a-z]+) \((?P<weight>[0-9]+)\)(?: -> (?P<deps>(?:(?:, )?[a-z]+)+))?")
        .unwrap()
        .captures_iter(input)
        .map(|capture| {
            (
                capture.name("name").unwrap().as_str().to_string(),
                (
                    capture["weight"].parse().unwrap(),
                    capture
                        .name("deps")
                        .map(|deps| deps.as_str().split(", ").map(str::to_string).collect())
                        .unwrap_or_else(Vec::new)
                ),
            )
        })
        .collect()
}

pub fn part1(input: &Input) -> String {
    let programs_map = input.clone();

    let deps_programs_set = programs_map
        .values()
        .flat_map(|(_, deps)| deps.iter().cloned())
        .collect::<HashSet<_>>();

    programs_map
        .keys()
        .filter(|&name| !deps_programs_set.contains(name))
        .exactly_one()
        .expect("Cyclic deps")
        .to_string()
}

pub fn part2(input: &Input) -> u32 {
    let programs_map = input.clone();

    let mut summed_weight_map = HashMap::new();

    while summed_weight_map.len() != programs_map.len() {
        for (program, (weight, deps)) in &programs_map {
            let &weight = weight;
            // We need to borrow the map between `contains` and `insert`
            #[allow(clippy::map_entry)]
            if !summed_weight_map.contains_key(program.as_str()) {
                let deps_weight: Option<u32> =
                    deps.iter().map(|dep| summed_weight_map.get(dep.as_str())).sum();
                if let Some(deps_weight) = deps_weight {
                    summed_weight_map.insert(program.as_str(), weight + deps_weight);
                }
            }
        }
    }

    let is_balanced = |program: &str| {
        programs_map[program]
            .1
            .iter()
            .map(|dep_name| summed_weight_map[dep_name.as_str()])
            .all_equal()
    };

    let root_of_wrong_program = programs_map
        .iter()
        .filter(|&(name, (_, deps))| !is_balanced(name) && deps.iter().all(|dep| is_balanced(dep)))
        .map(|(name, _)| name)
        .exactly_one()
        .expect("More than one program is wrong");

    let wrong_deps = &programs_map[root_of_wrong_program].1;

    let first_dep_weight = summed_weight_map[&wrong_deps[0].as_str()];
    let (mut right, mut wrong): (Vec<&str>, Vec<&str>) = wrong_deps
        .iter()
        .map(String::as_str)
        .partition(|&dep| summed_weight_map[dep] == first_dep_weight);

    if right.len() == 1 && wrong.len() == 1 {
        panic!("It's impossible to decide which program is wrong");
    }
    if right.len() == 1 {
        std::mem::swap(&mut right, &mut wrong);
    }

    let right_summed_weight = summed_weight_map[right[0]];
    let wrong_summed_weight = summed_weight_map[wrong[0]];

    programs_map[wrong[0]].0 + right_summed_weight - wrong_summed_weight
}
