extern crate regex;

#[derive(Eq,PartialEq,Copy,Clone,Debug)]
enum Team {
    ImmuneSystem,
    Infection,
}

type GID = usize;

#[derive(Debug, Clone)]
struct Group {
    gid: GID,
    team: Team,
    units: u64,
    unit_hp: u64,
    attack: String,
    damage: u64,
    initiative: u64,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
    targeting: Option<GID>,
    targeted_by: Option<GID>,
}

impl Group {
    fn effective_power(&self) -> u64 {
        self.units * self.damage
    }

    fn get_target_damage(&self, target: &Group) -> u64 {
        //! How much would we damage this target?
        if target.immunities[..].contains(&self.attack) {
            0
        } else if target.weaknesses[..].contains(&self.attack) {
            self.effective_power() * 2
        } else {
            self.effective_power()
        }
    }

    fn choose_target<'i,I>(&self, targets: I) -> Option<GID>
     where I: Iterator<Item=&'i Group> {
        /* 
        The attacking group chooses to target the group in the enemy army to which it would deal the most damage (after accounting for weaknesses and immunities, but not accounting for whether the defending group has enough units to actually receive all of that damage).

        If an attacking group is considering two defending groups to which it would deal equal damage, it chooses to target the defending group with the largest effective power; if there is still a tie, it chooses the defending group with the highest initiative.
        */
        if let Some(target) = targets.filter(|t| t.team != self.team && t.targeted_by.is_none())
            .max_by(|t1,t2| 
                self.get_target_damage(t1).cmp(&self.get_target_damage(t2))
                .then(t1.effective_power().cmp(&t2.effective_power()))
                .then(t1.initiative.cmp(&t2.initiative)))
        {
            if self.get_target_damage(target) > 0 {
                return Some(target.gid);
            }
        }
        None
    }
}

fn main() {
    let input = include_str!("24_input");
    //let input = include_str!("24_test");
    let re = regex::Regex::new(r"(\d+) units each with (\d+) hit points.*with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
    let weak_re = regex::Regex::new(r"weak to ([^;\)]+)").unwrap();
    let immune_re = regex::Regex::new(r"immune to ([^;\)]+)").unwrap();
    let mut groups = Vec::new();
    let mut team = Team::ImmuneSystem;
    for line in input.lines() {
        if line == "Immune System:" {
            team = Team::ImmuneSystem;
        } else if line == "Infection:" {
            team = Team::Infection;
        } else if line == "" {
            continue;
        } else {
            let caps = re.captures(line);
            let weak_caps = weak_re.captures(line);
            let immune_caps = immune_re.captures(line);
            if let Some(caps) = caps {
                let mut group = Group {gid: groups.len(), 
                    team,
                    units: caps[1].parse().unwrap(), 
                    unit_hp: caps[2].parse().unwrap(),
                    damage: caps[3].parse().unwrap(), 
                    attack: caps[4].into(),
                    initiative: caps[5].parse().unwrap(), 
                    weaknesses: Vec::new(),
                    immunities: Vec::new(),
                    targeting: None,
                    targeted_by: None};
                if let Some(weak_caps) = weak_caps {
                    group.weaknesses = weak_caps[1].split(", ").map(|s| s.to_string()).collect();
                }
                if let Some(immune_caps) = immune_caps {
                    group.immunities = immune_caps[1].split(", ").map(|s| s.to_string()).collect();
                }
                groups.push(group);
            }
        }
    }

    for boost in 0.. {
        let mut boosted_groups = groups.clone();
        for g in boosted_groups.iter_mut() {
            if g.team == Team::ImmuneSystem {
                g.damage += boost;
            }
        }
        let (immune_units, infection_units) = war(boosted_groups);
        println!("war with boost +{}: immune {}, infection: {}", boost, immune_units, infection_units);
        if infection_units == 0 {
            break;
        }
    }
}

fn war(mut groups: Vec<Group>) -> (u64, u64) {
    'game: loop {
        groups.iter_mut().for_each(|g| {g.targeting = None; g.targeted_by = None;});

        // order gids by targeting order
        let mut target_order : Vec<GID> = (0..groups.len()).collect();
        target_order.sort_by(|a,b| {
            let ga = &groups[*a];
            let gb = &groups[*b];
            ga.effective_power().cmp(&gb.effective_power()).reverse()
            .then(ga.initiative.cmp(&gb.initiative).reverse())
        });
        
        for gid in target_order {
            if let Some(target_gid) = groups[gid].choose_target(groups.iter().filter(|g| g.units > 0)) {
                groups[gid].targeting = Some(target_gid);
                groups[target_gid].targeted_by = Some(gid);
            }
        }

        // print targeting
        // for group in groups.iter() {
        //     if let Some(target_gid) = group.targeting {
        //         let target_group = &groups[target_gid];
        //         println!("group {} targeting {} with expected damage {}", group.gid, target_group.gid, group.get_target_damage(target_group));
        //     }
        // }

        // sort by max initiative
        let mut initiative_order : Vec<_> = (0..groups.len()).collect();
        initiative_order.sort_by(|a,b| {
            let ga = &groups[*a];
            let gb = &groups[*b];
            ga.initiative.cmp(&gb.initiative).reverse()
        });

        let mut damage_dealt = false;
        for gid in initiative_order {
            if groups[gid].units > 0 {
                if let Some(target_id) = groups[gid].targeting {
                    let damage = groups[gid].get_target_damage(&groups[target_id]);
                    let units_killed = (damage/groups[target_id].unit_hp).min(groups[target_id].units);
                    // println!("gid {} attacks gid {} and kills {} units", gid, target_id, units_killed);
                    groups[target_id].units -= units_killed;
                    if units_killed > 0 {
                        damage_dealt = true;
                    }
                }
            }
        }

        let immune_groups : u64 = groups.iter().filter(|g| g.team == Team::ImmuneSystem && g.units > 0).map(|g| g.units).sum();
        let infection_groups : u64 = groups.iter().filter(|g| g.team == Team::Infection && g.units > 0).map(|g| g.units).sum();
        // println!("round done, immune: {} infection: {}", immune_groups, infection_groups);
        if immune_groups == 0 || infection_groups == 0 {
            return (immune_groups, infection_groups);
        }
        if !damage_dealt {
            return (immune_groups, infection_groups);
        }
    }
}