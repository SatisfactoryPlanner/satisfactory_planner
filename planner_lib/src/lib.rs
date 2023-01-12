pub mod error;

#[cfg(test)]
mod tests {
    use planner_registry::Registry;

    use super::*;

    #[test]
    fn it_works() {
        let registry = Registry::new();

        let res = registry.search("ir");
        for res in res {
            println!("{}", res.0);
        }
    }
}
