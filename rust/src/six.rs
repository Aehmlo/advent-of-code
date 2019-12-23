use std::{collections::HashMap, iter::FromIterator, str::FromStr};

pub(crate) fn run(input: String) -> [String; 2] {
    let planets = input
        .lines()
        .filter_map(|x| x.parse::<Pair>().ok())
        .collect::<Planets>();
    let total = planets.total_orbits();
    let total = format!("{}", total);
    let path1 = planets.path(&"YOU".parse().unwrap(), &"COM".parse().unwrap());
    let path2 = planets.path(&"SAN".parse().unwrap(), &"COM".parse().unwrap());
    let common = path1
        .iter()
        .find(|x| path2.contains(x))
        .expect("No intersection.");
    let dist = path1.iter().take_while(|x| x != &common).count()
        + path2.iter().take_while(|x| x != &common).count();
    let dist = format!("{}", dist);
    [total, dist]
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Planet {
    name: String,
}

impl FromStr for Planet {
    type Err = std::convert::Infallible;
    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            name: name.to_string(),
        })
    }
}

#[derive(Debug, Default)]
struct Planets {
    map: HashMap<Planet, Planet>,
}

impl Planets {
    fn orbits(&self, planet: &Planet) -> u32 {
        if let Some(center) = self.map.get(planet) {
            1 + self.orbits(&center)
        } else {
            0
        }
    }
    fn total_orbits(&self) -> u32 {
        self.map.keys().map(|x| self.orbits(x)).sum()
    }
    fn path(&self, from: &Planet, to: &Planet) -> Vec<Planet> {
        let mut path = Vec::new();
        let mut planet = from;
        loop {
            if let Some(p) = self.map.get(planet) {
                planet = p;
                if planet == to {
                    break;
                }
                path.push(planet.clone());
            } else {
                panic!("No parent!");
            }
        }
        path
    }
}

impl FromIterator<Pair> for Planets {
    fn from_iter<T: IntoIterator<Item = Pair>>(iter: T) -> Self {
        let mut map = HashMap::new();
        for pair in iter {
            let satellite = pair.companion;
            let center = pair.center;
            map.insert(satellite, center);
        }
        Self { map }
    }
}

struct Pair {
    center: Planet,
    companion: Planet,
}

impl FromStr for Pair {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut planets = s.split(')').filter_map(|x| x.parse::<Planet>().ok());
        if let (Some(center), Some(companion)) = (planets.next(), planets.next()) {
            Ok(Self { center, companion })
        } else {
            Err(())
        }
    }
}
