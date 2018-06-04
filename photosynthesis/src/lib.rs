

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Molecule {
	CarbonDioxide,
	Water,
	Glucose,
	Oxygen,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Molecules {
    mol: Molecule,
    amount: usize,
}

impl Molecules {
    pub fn new(mol: Molecule, amount: usize) -> Molecules {
        Molecules {
            mol,
            amount,
        }
    }

    pub fn get_amount(&self) -> &usize {
        &self.amount
    }

    pub fn get_mol(&self) -> &Molecule {
        &self.mol
    }
}

pub struct Recipe {
    consumer: Vec<Molecules>,
    producer: Vec<Molecules>,
}

impl Recipe {
    pub fn new(consumer: Vec<Molecules>, producer: Vec<Molecules>) {
        Recipe {
            consumer,
            producer,
        }
    }

    pub fn can_consume(&self, list: &Vec<Molecules>) -> bool {
        panic!("TODO");
        false
    }

    pub fn consume(&self, list: &mut Vec<Molecules>) {
        panic!("TODO")
    }
}

pub struct MoleculeSet {
	pub molecules: Vec<Molecules>,
    pub history: Vec<Vec<Molecules>>,
    pub recipes: Vec<Recipe>,
}

impl MoleculeSet {
	pub fn new(molecules: Vec<Molecules>) -> MoleculeSet {
		MoleculeSet {
			molecules,
            history: Vec::new(),
            recipes: default_recipes(),
		}
	}

	pub fn contains(&self, molecule: &Molecules) -> bool {
		self.molecules.contains(molecule)
	}

    /// Removes all instances of molecule.
    pub fn remove(&mut self, molecule: &Molecules) -> Option<Molecules> {

        self.molecules.retain(|m: &Molecules | m.get_mol() == molecule.get_mol());
        /*
        for (i, mol) in self.molecules.iter().enumerate() {
            if mol.get_mol() == molecule.get_mol() {
                return Some(self.molecules.remove(i));
            }
        }
        */
        None

    }

    pub fn synthesize(&mut self) -> std::io::Result<()> {
        self.history.push(self.molecules.clone());

        let co2 = Molecules::new(Molecule::CarbonDioxide, 6);
        let water = Molecules::new(Molecule::Water, 6);

        // Only works for best case scenario.
        if self.contains(&co2) && self.contains(&water) {
            self.remove(&co2);
            self.remove(&water);

            let glucose = Molecules::new(Molecule::Glucose, 1);
            let oxygen = Molecules::new(Molecule::Oxygen, 6);
            self.molecules.push(glucose);
            self.molecules.push(oxygen);
        }

        panic!("TODO");

        Ok(())
    }

}


/// Synthesize the molecules to return a new set of molecules.
///
/// If no match is found for the given molecules, the set will remain the same.
///
/// The ordering of the input is unimportant.
///
/// It is assumed that sunlight is applied, otherwise the reaction can not occur.
///
/// # Examples
///
/// ```
/// use photosynthesis::{Molecules, Molecule, synthesize};
/// let mut input = Vec::new();
/// input.push(Molecules::new(Molecule::CarbonDioxide, 6));
/// input.push(Molecules::new(Molecule::Water, 6));
/// let result = synthesize(input);
///
/// let glucose = Molecules::new(Molecule::Glucose, 1);
/// 
/// assert_eq!(true, result.contains(&glucose));
/// ```
pub fn synthesize(molecules: Vec<Molecules>) -> MoleculeSet {
	let mut set = MoleculeSet::new(molecules);
    set.synthesize();
    set
}


/// Returns a vector of all the recipes that the moleculeSet will use when
/// conducting what ingredients, will produce what result.
pub fn default_recipes() -> Vec<Recipe> {
    panic!("TODO")
}

