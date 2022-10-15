use crate::{concept::Concept, pair::Pair, data::Data};

pub struct Block<'a>{
  pub number: u32,
  pub concepts: Vec<&'a Concept<'a>>,
  pub pairs: Vec<&'a Pair<'a>>,
  pub data: Vec<&'a Data<'a>>
}

impl<'a> Block<'a>{
  pub fn new(number: Option<u32>) -> Self{
    Self {
      number: number.unwrap_or(0), 
      concepts: vec![], 
      pairs: vec![], 
      data: vec![] 
    }
  }

  pub fn add_concept(&mut self, concept: &'a Concept<'a>){
    self.concepts.push(concept)
  }

  pub fn add_pair(&mut self, pair: &'a Pair<'a>){
    self.pairs.push(pair)
  }

  pub fn add_data(&mut self, data: &'a Data<'a>){
    self.data.push(data)
  }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
      let block = Block::new(None);
      assert_eq!(block.concepts.len(), 0);
      assert_eq!(block.pairs.len(), 0);
      assert_eq!(block.data.len(), 0);
      assert_eq!(block.number, 0);

      let block = Block::new(Some(5));
      assert_eq!(block.number, 5);
    }

    #[test]
    fn add_concept(){
      let cat_concept = Concept::new("Cat");
      let mut block =  Block::new(None);

      block.add_concept(&cat_concept);
      assert_eq!(block.concepts.len(), 1)
    }

    #[test]
    fn add_pair(){
      let cat_concept = Concept::new("Cat");
      let paw_concept = Concept::new("Paw");
      let pair = Pair::new(&cat_concept,&paw_concept);


      let mut block = Block::new(None);
      block.add_pair(&pair);
      assert_eq!(block.pairs.len(), 1)
    }

    #[test]
    fn add_data(){
      let cat_concept = Concept::new("Cat");
      let paw_concept = Concept::new("Paw");
      let pair = Pair::new(&cat_concept,&paw_concept);
      let data = Data::new(&pair, true);

      let mut block = Block::new(None);
      block.add_data(&data);
      assert_eq!(block.data.len(), 1)
    }
}
