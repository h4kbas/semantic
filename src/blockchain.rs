use crate::block::Block;

pub struct Blockchain<'a>{
  pub blocks: Vec<&'a Block<'a>>
}

impl<'a> Blockchain<'a>{
  
  pub fn new() -> Self{
    Self{ blocks: vec![] }
  }
  
  pub fn add_block(&'a mut self, block: &'a Block<'a>){
    self.blocks.push(block)
  }
}


#[cfg(test)]
mod tests {
    #[test]
    fn new() {
    
    }
}