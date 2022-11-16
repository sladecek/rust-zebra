use crate::backtracking::State;
use bitvec::prelude::*;
use std::fmt::{self, Display};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ZebraProperties<'a> {
    object_count: usize,
    properties: Vec<&'a str>,
    options: Vec<Vec<&'a str>>,
}

#[derive(Debug, Clone)]
pub struct Zebra<'a> {
    props: Rc<ZebraProperties<'a>>,
    values: BitVec,
}

impl Zebra<'_> {
    pub fn is_choice_enabled(&self, property: usize, object: usize, choice: usize) -> bool {
        self.values[self.index(property, object, choice)]
    }

    pub fn set_choice_enabled(
        &mut self,
        property: usize,
        object: usize,
        choice: usize,
        value: bool,
    ) {
        let ix = self.index(property, object, choice);
        self.values.set(ix, value);
    }

    fn index(&self, property: usize, object: usize, choice: usize) -> usize {
        let oc = self.props.get_object_count();
        let pc = self.props.get_property_count();
        
        property * oc * oc + object * oc + choice
    }

    pub fn fix_first_property(&mut self) {
        // First object will be assigned the first choice, second object second ...
        let property = 0;
        for object in 0..self.props.get_object_count() {
            self.determine_choice(property, object, object);
        }
    }

    fn determine_choice(&mut self, property: usize, object: usize, choice: usize) {
        for ch in 0..self.props.get_object_count() {
            self.set_choice_enabled(property, object, ch, choice == ch)
        }
    }

    fn apply_facts(&mut self){

    }

    fn has_no_choice(&self, property: usize, object: usize)->bool {
        !((0..self.props.get_property_count()).any(|choice| self.is_choice_enabled(property, object, choice)))
    }

}

impl Display for Zebra<'_> {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        for property in 0..self.props.get_property_count() {
            write!(dest, "{} -----\n", self.props.get_property_name(property))?;
            for object in 0..self.props.get_object_count() {
                write!(dest, "{}:", object)?;
                for choice in 0..self.props.get_object_count() {
                    if self.is_choice_enabled(property, object, choice) {
                        write!(
                            dest,
                            "{} ",
                            self.props.get_property_choice_name_by_nr(property, choice)
                        )?;
                    }
                }
                write!(dest, "\n")?;
            }
        }

        Ok(())
    }
}

impl State for Zebra<'_> {
    fn is_valid(&mut self) -> bool {
        self.apply_facts();
        for object in 0..self.props.get_object_count() {
            for property in 0..self.props.get_property_count() {
                if self.has_no_choice(object, property) {
                    return false
                }
            }
        }
        true
        
    }
    fn split(&self) -> Vec<Self> {
        vec![]
    }
}

impl<'a> ZebraProperties<'a> {
    pub fn get_object_count(&self) -> usize {
        self.object_count
    }

    pub fn get_property_count(&self) -> usize {
        self.properties.len()
    }

    pub fn get_property_name(&self, i: usize) -> &'a str {
        &self.properties[i]
    }

    pub fn get_property(&self, name: &str) -> Option<usize> {
        self.properties.iter().position(|s| *s == name)
    }

    pub fn get_property_choice_name(&self, property: &str, i: usize) -> &'a str {
        self.options[self.get_property(property).unwrap()][i]
    }

    pub fn get_property_choice_name_by_nr(&self, p: usize, i: usize) -> &'a str {
        self.options[p][i]
    }

    pub fn get_property_choice(&self, property: &str, name: &str) -> Option<usize> {
        let option_pos = self.get_property(property).unwrap();
        self.options[option_pos].iter().position(|s| *s == name)
    }
}

pub struct ZebraBuilder<'a> {
    zebra: ZebraProperties<'a>,
}

impl<'a> ZebraBuilder<'a> {
    pub fn new() -> ZebraBuilder<'a> {
        ZebraBuilder {
            zebra: ZebraProperties {
                object_count: 0,
                properties: vec![],
                options: vec![],
            },
        }
    }

    pub fn build(&mut self) -> Zebra<'a> {
        for i in 0..self.zebra.options.len() {
            let chlen = self.zebra.options[i].len();
            if chlen != self.zebra.object_count {
                panic!(
                    "Invalid number choices in property {} is {} should be {}",
                    self.zebra.get_property_name(i),
                    chlen,
                    self.zebra.object_count
                );
            }
        }
        let bit_count = self.zebra.get_property_count()
            * self.zebra.get_object_count()
            * self.zebra.get_object_count();
        let mut result = Zebra {
            props: Rc::new(self.zebra.clone()),
            values: bitvec![1; bit_count],
        };
        result.fix_first_property();
        result
    }

    pub fn set_object_count(&mut self, size: usize) -> &mut Self {
        self.zebra.object_count = size;
        self
    }

    pub fn fact(
        &mut self,
        property1: &'a str,
        choice1: &'a str,
        property2: &'a str,
        choice2: &'a str,
    ) -> &mut Self {
        let ch1 = self.ensure_choice(property1, choice1);
        let ch2 = self.ensure_choice(property2, choice2);
        self
    }

    pub fn predicate(
        &mut self,
        property1: &'a str,
        choice1: &'a str,
        property2: &'a str,
        choice2: &'a str,
        property3: &'a str,
        property4: &'a str,
        test: &dyn Fn(&str, &str) -> bool,
    ) -> &mut Self {
        let ch1 = self.ensure_choice(property1, choice1);
        let ch2 = self.ensure_choice(property2, choice2);
        let p3 = self.ensure_property(property3);
        let p4 = self.ensure_property(property4);

        self
    }

    pub fn choice(&mut self, property: &'a str, choices: Vec<&'a str>) -> &mut Self {
        for ch in choices {
            self.ensure_choice(property, ch);
        }
        self
    }

    fn ensure_property(&mut self, property: &'a str) -> usize {
        let pr = self.zebra.get_property(property);

        if pr.is_none() {
            self.zebra.properties.push(property);
            self.zebra.options.push(vec![]);
            self.zebra.get_property_count() - 1
        } else {
            pr.unwrap()
        }
    }

    fn ensure_choice(&mut self, property: &'a str, choice: &'a str) -> (usize, usize) {
        let prix = self.ensure_property(property);
        let ch = self.zebra.get_property_choice(property, choice);
        let chix = if ch.is_none() {
            self.zebra.options[prix].push(choice);
            self.zebra.options[prix].len() - 1
        } else {
            ch.unwrap()
        };

        (prix, chix)
    }
}

#[test]
fn test_zebra_builder() {
    let zebra = ZebraBuilder::new()
        .set_object_count(3)
        .fact("p1", "p1a", "p2", "p2a")
        .fact("p1", "p1b", "p2", "p2b")
        .choice("p1", vec!["p1c"])
        .choice("p2", vec!["p2c"])
        .choice("p3", vec!["p3a", "p3b", "p3c"])
        .choice("p4", vec!["p4a", "p4b", "p4c"])
        .predicate("p1", "p1a", "p2", "p2c", "p4", "p4", &|a, b| true)
        .build();

    assert_eq!(4, zebra.props.get_property_count());

    assert_eq!(3, zebra.props.get_object_count());

    assert_eq!("p1", zebra.props.get_property_name(0));

    assert_eq!(Some(0), zebra.props.get_property("p1"));

    assert_eq!("p1a", zebra.props.get_property_choice_name("p1", 0));

    assert_eq!(Some(0), zebra.props.get_property_choice("p1", "p1a"));
}
