use rand::Rng;
use rand_distr::Distribution;
use strict_num::NormalizedF64;

#[derive(Debug, Clone)]
pub struct Population<A, D> {
    individuals: Vec<A>,
    generations: usize,
    probabilities: Vec<NormalizedF64>,
    dna: Vec<D>,
}
impl<A, D> Population<A, D> {
    pub fn new(individuals: Vec<A>) -> Self {
        Self {
            individuals,
            generations: 0,
            probabilities: vec![],
            dna: vec![],
        }
    }

    pub fn individuals(&self) -> &Vec<A> {
        &self.individuals
    }
    pub fn individuals_mut(&mut self) -> &mut Vec<A> {
        &mut self.individuals
    }
    pub fn generations(&self) -> usize {
        self.generations
    }
}
impl<A, D> Population<A, D>
where
    A: Agent<Dna = D>,
    D: Dna,
{
    pub fn reproduce(&mut self, scores: &[f64], rate: NormalizedF64) {
        assert_eq!(scores.len(), self.individuals.len());
        self.probabilities.clear();
        self.probabilities.extend(probabilities(scores));
        let mut rng = rand::thread_rng();
        self.dna.clear();
        for _ in 0..self.individuals.len() {
            let a = select_parent(&self.probabilities, &mut rng);
            let b = select_parent(&self.probabilities, &mut rng);
            let a = &self.individuals[a];
            let b = &self.individuals[b];
            let mut dna = a.crossover(b);
            dna.mutate(rate);
            self.dna.push(dna);
        }
        let mut i = 0;
        while let Some(dna) = self.dna.pop() {
            self.individuals[i].override_dna(dna);
            i += 1;
        }
        self.generations += 1;
    }
}

pub fn probabilities(scores: &[f64]) -> impl Iterator<Item = NormalizedF64> + Clone + '_ {
    let sum = scores.iter().sum::<f64>();
    scores
        .iter()
        .map(move |&score| score / sum)
        .map(|x| NormalizedF64::new(x).unwrap())
}

pub fn select_parent(probabilities: &[NormalizedF64], rng: &mut impl Rng) -> usize {
    let dart = rng.gen_range(0. ..1.);
    let mut cum = 0.;
    for (i, prob) in probabilities.iter().enumerate() {
        cum += prob.get();
        if dart < cum {
            return i;
        }
    }
    probabilities.len().checked_sub(1).unwrap()
}

pub trait Agent {
    type Dna: Dna;
    fn crossover(&self, other: &Self) -> Self::Dna;
    fn override_dna(&mut self, dna: Self::Dna);
}

pub trait Dna {
    fn mutate(&mut self, rate: NormalizedF64);
}

pub fn crossover(a: f64, b: f64, rng: &mut impl Rng) -> f64 {
    if rng.gen_bool(0.5) {
        a
    } else {
        b
    }
}

pub fn mutate(value: f64, rate: NormalizedF64, rng: &mut impl Rng) -> f64 {
    let normal = rand_distr::Normal::new(0., 1.).unwrap();
    let chance = rng.gen_bool(rate.get());
    if !chance {
        return value;
    }
    let change = normal.sample(rng);
    let value = value + change;
    value.clamp(-1., 1.)
}
