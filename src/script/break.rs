use super::Script;
use std::io::{Error, ErrorKind::Interrupted};
use yaml_rust2::{yaml::Hash, Yaml};

// - break: [<condition>]
//   [message: <string>]
pub fn run(script: &mut Script, cond: &Yaml, step: &Hash) -> Result<(), Error> {
    match script.binding.is_truthy(cond) {
        true => Err(Error::new(Interrupted, message(step))),
        false => Ok(()),
    }
}

fn message(step: &Hash) -> String {
    step.get(&Yaml::from_str("message"))
        .unwrap_or(&Yaml::from_str("(break)"))
        .as_str()
        .unwrap()
        .to_string()
}

//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use yaml_rust2::YamlLoader;

    #[test]
    fn run() {
        let mut script = Script::new(String::new(), None);
        let docs = YamlLoader::load_from_str("foo:").unwrap();
        let hash = docs[0].as_hash().unwrap();

        let err = super::run(&mut script, &Yaml::from_str("true"), &hash).unwrap_err();
        assert_eq!(Interrupted, err.kind());
        assert_eq!("(break)", err.to_string());
    }

    #[test]
    fn run_message() {
        let mut script = Script::new(String::new(), None);
        let docs = YamlLoader::load_from_str("message: foo").unwrap();
        let hash = docs[0].as_hash().unwrap();

        let err = super::run(&mut script, &Yaml::from_str("true"), &hash).unwrap_err();
        assert_eq!("foo", err.to_string());
    }
}