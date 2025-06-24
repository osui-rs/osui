#[macro_export]
macro_rules! components {
    ($($a:ty: $e:expr),*) => {
        {
            let mut hm: HashMap<TypeId, Box<dyn Component>> = HashMap::new();
            $(
                hm.insert(TypeId::of::<$a>(), Box::new($e));
            )*
            hm
        }
    };
}
