use super::{binding::Binding, Script};
use ternop::ternary;
use yaml_rust2::{yaml::Hash, Yaml};

// - if: <condition>
//   [then: <steps>]
//   [else: <steps>]
//
// condition = <expression> where true = true | !0 | !0.0 | !""
pub fn run(script: &mut Script, cond: &Yaml, step: &Hash) {
    let key = ternary!(script.binding.is_truthy(cond), "then", "else");
    let steps = Binding::hash_to_list(key, step);
    script.run_steps(&steps);
}

//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use yaml_rust2::YamlLoader;

    #[test]
    fn if_then() {
        let mut script = Script::new(String::new(), None);
        let docs = YamlLoader::load_from_str("then: [a: 42]").unwrap();
        let hash = docs[0].as_hash().unwrap();

        super::run(&mut script, &Yaml::from_str("true"), &hash);
        assert_eq!(42, script.binding.get("a"));
    }

    #[test]
    fn if_else() {
        let mut script = Script::new(String::new(), None);
        let docs = YamlLoader::load_from_str("else: [a: 42]").unwrap();
        let hash = docs[0].as_hash().unwrap();

        super::run(&mut script, &Yaml::from_str("false"), &hash);
        assert_eq!(42, script.binding.get("a"));
    }
}
