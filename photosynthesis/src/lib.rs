

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
    pub fn new(consumer: Vec<Molecules>, producer: Vec<Molecules>) -> Recipe {
        Recipe {
            consumer,
            producer,
        }
    }

    pub fn eval(&self, list: &Vec<Molecules>) -> bool {
        for it in self.consumer.iter() {
            if !list.contains(it) {
                return false;
            }
        }
        true
    }

    pub fn consume(&self, list: &mut Vec<Molecules>) {
        if self.eval(&list) {
            list.extend(self.producer.iter().cloned());
        }
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
        None
    }

    pub fn synthesize(&mut self) -> std::io::Result<()> {
        self.history.push(self.molecules.clone());
        for rec in self.recipes.iter() {
            if rec.eval(&self.molecules) {
                rec.consume(&mut self.molecules);
                break;
            }
        }
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
///
/// This is suppose to be en example, and should be extended with your own
/// recipes.
pub fn default_recipes() -> Vec<Recipe> {
    // Basic photosynthesis.

    vec!(
        Recipe::new(
            vec!(
                Molecules::new(Molecule::CarbonDioxide, 6),
                Molecules::new(Molecule::CarbonDioxide, 6)
            ),
            vec!(
                Molecules::new(Molecule::Glucose, 1),
                Molecules::new(Molecule::Oxygen, 6)
            )
        )
    )
}

