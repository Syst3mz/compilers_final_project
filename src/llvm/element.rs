#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Scope(Vec<Element>),
    Elem(String)
}

impl Element {
    pub fn flatten(self) -> Vec<String> {
        match self {
            Element::Scope(v) => {
                let mut ret = vec![];

                for mut flat in v.into_iter().map(|x| x.flatten()) {
                    for str in flat.iter_mut() {
                        *str = format!("\t{}", str)
                    }

                    ret.append(&mut flat)
                }

                ret
            }
            Element::Elem(e) => vec![e]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_deep() {
        let t = Element::Elem(String::from("a"));
        assert_eq!(t.flatten(), vec![String::from("a")])
    }

    #[test]
    fn one_deep() {
        let t = Element::Scope(vec![Element::Elem(String::from("a")), Element::Elem(String::from("b"))]);
        assert_eq!(t.flatten(), vec![String::from("\ta"), String::from("\tb")])
    }

    #[test]
    fn two_deep() {
        let t = Element::Scope(vec![Element::Elem(String::from("a")), Element::Scope(vec![Element::Elem(String::from("b")), Element::Elem(String::from("c"))])]);
        assert_eq!(t.flatten(), vec![String::from("\ta"), String::from("\t\tb"), String::from("\t\tc")])
    }

    #[test]
    fn universe() {
        let t = vec![
            Element::Elem(String::from("define int @main() {")),
            Element::Scope(vec![
                Element::Elem(String::from("%i321 = i32 42")),
                Element::Elem(String::from("ret i32 %i321"))
            ]),
            Element::Elem(String::from("}"))
        ];
        let t: Vec<String> = t.into_iter().map(|x| x.flatten()).flatten().collect();

        assert_eq!(t, vec![
            String::from("define int @main() {"),
            String::from("\t%i321 = i32 42"),
            String::from("\tret i32 %i321"),
            String::from("}")
        ])
    }
}