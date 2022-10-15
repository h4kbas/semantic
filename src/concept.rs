pub struct Concept<'a>{
  pub name: &'a str
}

impl<'a> Concept<'a>{
  pub fn new(name: &'a str) -> Self{
    Self{
      name
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
      // raw string
      let cat = Concept::new("Cat");
      assert_eq!(cat.name, "Cat");
      // &str
      let strcat = "Cat";
      let cat = Concept::new(strcat);
       assert_eq!(cat.name, "Cat");
       // String
       let string_cat = String::from("Cat");
       let cat = Concept::new(&string_cat);
       assert_eq!(cat.name, "Cat")
    }
}
