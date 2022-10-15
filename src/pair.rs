use crate::concept::Concept;

pub struct Pair<'a>(&'a Concept<'a>, &'a Concept<'a>);

impl<'a> Pair<'a>{
  pub fn new(concept_one: &'a Concept, concept_two: &'a Concept) -> Self{
    Self(concept_one, concept_two)
  }
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use crate::concept::Concept;

    use super::*;

    #[test]
    fn new() {
      let cat_concept = Concept::new("Cat");
      let paw_concept = Concept::new("Paw");
      let pair = Pair::new(&cat_concept,&paw_concept);

      assert!(ptr::eq(pair.0, &cat_concept));
      assert!(ptr::eq(pair.1, &paw_concept));
    }
}