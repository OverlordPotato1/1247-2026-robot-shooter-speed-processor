
#[macro_export]
macro_rules! hypot {
  ($($x:expr),*) => {
    {
      let args = vec![$($x),*];
      args.iter().map(|val| val * val).reduce(|a, b| a + b).unwrap().sqrt()
    }
  };
}