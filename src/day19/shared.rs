use std::ops;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
pub struct ResourceVec {
    pub ore: i32,
    pub clay: i32,
    pub obsidian: i32,
    pub geode: i32,
}

impl ResourceVec {
    pub fn max(&self) -> i32 {
        *[self.ore, self.clay, self.obsidian, self.geode]
            .iter()
            .max()
            .unwrap()
    }
    #[allow(dead_code)]
    pub fn min(&self) -> i32 {
        *[self.ore, self.clay, self.obsidian, self.geode]
            .iter()
            .min()
            .unwrap()
    }
    // divides each field, rounding up
    pub fn div_round_up(&self, other: ResourceVec) -> ResourceVec {
        ResourceVec {
            ore: if other.ore == 0 {
                0
            } else {
                (self.ore as f32 / other.ore as f32).ceil() as i32
            },
            clay: if other.clay == 0 {
                0
            } else {
                (self.clay as f32 / other.clay as f32).ceil() as i32
            },
            obsidian: if other.obsidian == 0 {
                0
            } else {
                (self.obsidian as f32 / other.obsidian as f32).ceil() as i32
            },
            geode: if other.geode == 0 {
                0
            } else {
                (self.geode as f32 / other.geode as f32).ceil() as i32
            },
        }
    }
}

impl ops::Add for ResourceVec {
    type Output = ResourceVec;

    fn add(self, other: ResourceVec) -> ResourceVec {
        ResourceVec {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

impl ops::Sub for ResourceVec {
    type Output = ResourceVec;

    fn sub(self, other: ResourceVec) -> ResourceVec {
        ResourceVec {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

impl ops::Div<i32> for ResourceVec {
    type Output = ResourceVec;

    fn div(self, other: i32) -> ResourceVec {
        ResourceVec {
            ore: self.ore / other,
            clay: self.clay / other,
            obsidian: self.obsidian / other,
            geode: self.geode / other,
        }
    }
}

impl ops::Div for ResourceVec {
    type Output = ResourceVec;

    fn div(self, other: ResourceVec) -> ResourceVec {
        ResourceVec {
            ore: if other.ore == 0 {
                0
            } else {
                self.ore / other.ore
            },
            clay: if other.clay == 0 {
                0
            } else {
                self.clay / other.clay
            },
            obsidian: if other.obsidian == 0 {
                0
            } else {
                self.obsidian / other.obsidian
            },
            geode: if other.geode == 0 {
                0
            } else {
                self.geode / other.geode
            },
        }
    }
}

impl ops::Mul<i32> for ResourceVec {
    type Output = ResourceVec;

    fn mul(self, other: i32) -> ResourceVec {
        ResourceVec {
            ore: self.ore * other,
            clay: self.clay * other,
            obsidian: self.obsidian * other,
            geode: self.geode * other,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Blueprint {
    pub id: i32,
    pub ore_ingredients: ResourceVec,
    pub clay_ingredients: ResourceVec,
    pub obsidian_ingredients: ResourceVec,
    pub geode_ingredients: ResourceVec,
}

impl Blueprint {
    pub fn max_ore(&self) -> i32 {
        *[
            self.ore_ingredients.ore,
            self.clay_ingredients.ore,
            self.obsidian_ingredients.ore,
            self.geode_ingredients.ore,
        ]
        .iter()
        .max()
        .unwrap()
    }
    pub fn max_clay(&self) -> i32 {
        *[
            self.ore_ingredients.clay,
            self.clay_ingredients.clay,
            self.obsidian_ingredients.clay,
            self.geode_ingredients.clay,
        ]
        .iter()
        .max()
        .unwrap()
    }
    pub fn max_obsidian(&self) -> i32 {
        *[
            self.ore_ingredients.obsidian,
            self.clay_ingredients.obsidian,
            self.obsidian_ingredients.obsidian,
            self.geode_ingredients.obsidian,
        ]
        .iter()
        .max()
        .unwrap()
    }
    #[allow(dead_code)]
    pub fn max_geode(&self) -> i32 {
        *[
            self.ore_ingredients.geode,
            self.clay_ingredients.geode,
            self.obsidian_ingredients.geode,
            self.geode_ingredients.geode,
        ]
        .iter()
        .max()
        .unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub inventory: ResourceVec,
    pub inputs: ResourceVec,
    pub blueprint: Blueprint,
    pub time_remaining: i32,
}

impl State {
    pub fn create_ore_robot(&self) -> Option<State> {
        if self.inputs.ore >= self.blueprint.max_ore() {
            // we have enough ore
            return None;
        }
        if (self.inventory.obsidian + self.time_remaining * self.inputs.obsidian)
            > self.blueprint.max_obsidian() * self.time_remaining
        {
            // we have enough obsidian to last the rest of the time remaining
            return None;
        }
        if let Some((time, new_inventory)) =
            wait_for_resource(self.blueprint.ore_ingredients, self.inventory, self.inputs)
        {
            if time >= self.time_remaining {
                return None;
            }
            let mut new_inputs = self.inputs;
            new_inputs.ore += 1;
            return Some(State {
                inventory: new_inventory + self.inputs - self.blueprint.ore_ingredients,
                inputs: new_inputs,
                blueprint: self.blueprint,
                time_remaining: self.time_remaining - time - 1,
            });
        }
        None
    }
    pub fn create_clay_robot(&self) -> Option<State> {
        if self.inputs.clay >= self.blueprint.max_clay() {
            // we have enough clay
            return None;
        }
        if (self.inventory.clay + self.time_remaining * self.inputs.clay)
            > self.blueprint.max_clay() * self.time_remaining
        {
            // we have enough clay to last the rest of the time remaining
            return None;
        }
        if (self.inventory.obsidian + self.time_remaining * self.inputs.obsidian)
            > self.blueprint.max_obsidian() * self.time_remaining
        {
            // we have enough obsidian to last the rest of the time remaining
            return None;
        }
        if let Some((time, new_inventory)) =
            wait_for_resource(self.blueprint.clay_ingredients, self.inventory, self.inputs)
        {
            if time >= self.time_remaining {
                return None;
            }
            let mut new_inputs = self.inputs;
            new_inputs.clay += 1;
            return Some(State {
                inventory: new_inventory + self.inputs - self.blueprint.clay_ingredients,
                inputs: new_inputs,
                blueprint: self.blueprint,
                time_remaining: self.time_remaining - time - 1,
            });
        }
        None
    }
    pub fn create_obsidian_robot(&self) -> Option<State> {
        if self.inputs.obsidian >= self.blueprint.max_obsidian() {
            // we have enough obsidian
            return None;
        }
        if (self.inventory.obsidian + self.time_remaining * self.inputs.obsidian)
            > self.blueprint.max_obsidian() * self.time_remaining
        {
            // we have enough obsidian to last the rest of the time remaining
            return None;
        }
        if let Some((time, new_inventory)) = wait_for_resource(
            self.blueprint.obsidian_ingredients,
            self.inventory,
            self.inputs,
        ) {
            if time >= self.time_remaining {
                return None;
            }
            let mut new_inputs = self.inputs;
            new_inputs.obsidian += 1;
            return Some(State {
                inventory: new_inventory + self.inputs - self.blueprint.obsidian_ingredients,
                inputs: new_inputs,
                blueprint: self.blueprint,
                time_remaining: self.time_remaining - time - 1,
            });
        }
        None
    }
    pub fn create_geode_robot(&self) -> Option<State> {
        if let Some((time, new_inventory)) = wait_for_resource(
            self.blueprint.geode_ingredients,
            self.inventory,
            self.inputs,
        ) {
            if time >= self.time_remaining {
                return None;
            }
            let mut new_inputs = self.inputs;
            new_inputs.geode += 1;
            return Some(State {
                inventory: new_inventory + self.inputs - self.blueprint.geode_ingredients,
                inputs: new_inputs,
                blueprint: self.blueprint,
                time_remaining: self.time_remaining - time - 1,
            });
        }
        None
    }
    pub fn idle(&self) -> State {
        State {
            inventory: self.inventory + self.inputs * self.time_remaining,
            inputs: self.inputs,
            blueprint: self.blueprint,
            time_remaining: 0,
        }
    }
}

pub fn parse_blueprints(input: &str) -> Vec<Blueprint> {
    let mut blueprints = vec![];
    let regex = Regex::new(r"Blueprint (?P<id>\d+): Each ore robot costs (?P<ore_ore>\d+) ore. Each clay robot costs (?P<clay_ore>\d+) ore. Each obsidian robot costs (?P<obsidian_ore>\d+) ore and (?P<obsidian_clay>\d+) clay. Each geode robot costs (?P<geode_ore>\d+) ore and (?P<geode_obsidian>\d+) obsidian.").unwrap();
    for line in input.lines() {
        let caps = regex.captures(line).unwrap();
        blueprints.push(Blueprint {
            id: caps.name("id").map_or(0, |m| m.as_str().parse().unwrap()),
            ore_ingredients: ResourceVec {
                ore: caps
                    .name("ore_ore")
                    .map_or(0, |m| m.as_str().parse().unwrap()),
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            clay_ingredients: ResourceVec {
                ore: caps
                    .name("clay_ore")
                    .map_or(0, |m| m.as_str().parse().unwrap()),
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            obsidian_ingredients: ResourceVec {
                ore: caps
                    .name("obsidian_ore")
                    .map_or(0, |m| m.as_str().parse().unwrap()),
                clay: caps
                    .name("obsidian_clay")
                    .map_or(0, |m| m.as_str().parse().unwrap()),
                obsidian: 0,
                geode: 0,
            },
            geode_ingredients: ResourceVec {
                ore: caps
                    .name("geode_ore")
                    .map_or(0, |m| m.as_str().parse().unwrap()),
                clay: 0,
                obsidian: caps
                    .name("geode_obsidian")
                    .map_or(0, |m| m.as_str().parse().unwrap()),
                geode: 0,
            },
        })
    }

    blueprints
}

pub fn wait_for_resource(
    target: ResourceVec,
    inventory: ResourceVec,
    input: ResourceVec,
) -> Option<(i32, ResourceVec)> {
    let diff = target - inventory;
    if diff.max() <= 0 {
        return Some((0, inventory));
    }
    let wait = diff.div_round_up(input).max().abs();
    let inventory = inventory + input * wait;
    let diff = target - inventory;
    if diff.max() <= 0 {
        return Some((wait, inventory));
    }
    None
}

/**
 * Returns a list of robots that could be built next, given the current inputs and time remaining.
 *
 * If the input for a given resource type is greater than or equal to the amount needed for any robot
 * blueprint, we don't need to build any more of that type of robot.
 *
 * If we don't have an input for one of the robot's ingredients,, or if it would take longer than
 * time_remaining for the resources to become available, we can't build that robot.
 */
pub fn build_robot_options(state: State) -> Vec<State> {
    let mut states = vec![];
    if let Some(new_ore_bot) = state.create_ore_robot() {
        states.push(new_ore_bot);
    }
    if let Some(new_clay_bot) = state.create_clay_robot() {
        states.push(new_clay_bot);
    }
    if let Some(new_obsidian_bot) = state.create_obsidian_robot() {
        states.push(new_obsidian_bot);
    }
    if let Some(new_geode_bot) = state.create_geode_robot() {
        states.push(new_geode_bot);
    }
    if states.is_empty() {
        states.push(state.idle());
    }
    states
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_rounding() {
        let state = State {
            inventory: ResourceVec {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            inputs: ResourceVec {
                ore: 2,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            blueprint: Blueprint {
                id: 1,
                ore_ingredients: ResourceVec {
                    ore: 4,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                clay_ingredients: ResourceVec {
                    ore: 2,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                obsidian_ingredients: ResourceVec {
                    ore: 3,
                    clay: 14,
                    obsidian: 0,
                    geode: 0,
                },
                geode_ingredients: ResourceVec {
                    ore: 2,
                    clay: 0,
                    obsidian: 7,
                    geode: 0,
                },
            },
            time_remaining: 10,
        };

        let next_states = build_robot_options(state);
        assert_eq!(next_states[0].time_remaining, 7);
    }

    #[test]
    fn test_clay_bot() {
        let state = State {
            inventory: ResourceVec {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            inputs: ResourceVec {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            blueprint: Blueprint {
                id: 1,
                ore_ingredients: ResourceVec {
                    ore: 4,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                clay_ingredients: ResourceVec {
                    ore: 2,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                obsidian_ingredients: ResourceVec {
                    ore: 3,
                    clay: 14,
                    obsidian: 0,
                    geode: 0,
                },
                geode_ingredients: ResourceVec {
                    ore: 2,
                    clay: 0,
                    obsidian: 7,
                    geode: 0,
                },
            },
            time_remaining: 10,
        };

        let next_states = build_robot_options(state);
        assert_eq!(next_states[1].time_remaining, 7);
    }

    #[test]
    fn test_geode_bot() {
        let state = State {
            inventory: ResourceVec {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            inputs: ResourceVec {
                ore: 1,
                clay: 0,
                obsidian: 1,
                geode: 0,
            },
            blueprint: Blueprint {
                id: 1,
                ore_ingredients: ResourceVec {
                    ore: 4,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                clay_ingredients: ResourceVec {
                    ore: 2,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                obsidian_ingredients: ResourceVec {
                    ore: 3,
                    clay: 14,
                    obsidian: 0,
                    geode: 0,
                },
                geode_ingredients: ResourceVec {
                    ore: 2,
                    clay: 0,
                    obsidian: 7,
                    geode: 0,
                },
            },
            time_remaining: 10,
        };

        let next_states = build_robot_options(state);
        for state in &next_states {
            println!("geode test {:?}", state);
        }
        assert_eq!(next_states[2].inputs.geode, 1);
        assert_eq!(next_states[2].time_remaining, 2);
    }
}
