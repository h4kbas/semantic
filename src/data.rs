use crate::pair::Pair;

pub struct Data<'a>(&'a Pair<'a>, bool);

impl<'a> Data<'a>{
  pub fn new(pair: &'a Pair<'a>, val: bool) -> Self{
    Self(pair , val)
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
      let data = Data::new(&pair, true);
      assert!(ptr::eq(data.0, &pair));
      assert_eq!(data.1, true);
    }
}