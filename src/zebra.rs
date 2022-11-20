use crate::backtracking::State;
use bitvec::prelude::*;
use std::cell::RefCell;
use std::clone::Clone;
use std::fmt::{self, Debug, Display};
use std::rc::Rc;

#[derive(Debug)]
pub struct Fact {
    choice1: (usize, usize),
    choice2: (usize, usize),
}

pub struct Predicate {
    choice1: (usize, usize),
    choice2: (usize, usize),
    property3: usize,
    property4: usize,
    test: Box<dyn Fn(&str, &str) -> bool>,
}

#[derive(Debug)]
pub struct ZebraProperties<'a> {
    object_count: usize,
    properties: Vec<&'a str>,
    options: Vec<Vec<&'a str>>,
    facts: Vec<Fact>,
    predicates: Vec<Predicate>,
}

impl Debug for Predicate {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        write!(
            dest,
            "Predicate {:?} {:?} {} {}",
            self.choice1, self.choice2, self.property3, self.property4
        )
    }
}

#[derive(Debug)]
pub struct Zebra<'a> {
    props: Rc<RefCell<ZebraProperties<'a>>>,
    values: BitVec,
}

impl<'a> Clone for Zebra<'a> {
    fn clone(&self) -> Self {
        Zebra {
            props: Rc::clone(&self.props),
            values: self.values.clone(),
        }
    }
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
        let oc = self.props.borrow().get_object_count();
        property * oc * oc + object * oc + choice
    }

    pub fn fix_first_property(&mut self) {
        // First object will be assigned the first choice, second object second ...
        let property = 0;
        let cnt = self.props.borrow().get_object_count();
        for object in 0..cnt {
            self.determine_choice(property, object, object);
        }
    }

    fn determine_choice(&mut self, property: usize, object: usize, choice: usize) {
        let cnt = self.props.borrow().get_object_count();
        for ch in 0..cnt {
            self.set_choice_enabled(property, object, ch, choice == ch)
        }
    }

    fn is_determined(&self, property: usize, object: usize) -> bool {
        ((0..self.props.borrow().get_object_count())
            .filter(|choice| self.is_choice_enabled(property, object, *choice)))
        .count()
            == 1
    }

    fn find_determined(&self, property: usize, object: usize) -> usize {
        ((0..self.props.borrow().get_object_count())
            .find(|choice| self.is_choice_enabled(property, object, *choice)))
        .unwrap()
    }

    fn apply_half_fact(
        &mut self,
        property: usize,
        object: usize,
        choice1: (usize, usize),
        choice2: (usize, usize),
        change_counter: &mut i32,
    ) -> bool {
        // assume that the choice (property, object) is determined
        let (p1, ch1) = choice1;
        let (p2, ch2) = choice2;
        if property == p1 && self.is_choice_enabled(property, object, ch1) {
            if !self.is_choice_enabled(p2, object, ch2) {
                return false;
            }
            if !self.is_determined(p2, object) {
                self.determine_choice(p2, object, ch2);
                *change_counter += 1;
            }
        }
        return true;
    }
}

impl Display for Zebra<'_> {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        for property in 0..self.props.borrow().get_property_count() {
            write!(
                dest,
                "{} -----\n",
                self.props.borrow().get_property_name(property)
            )?;
            for object in 0..self.props.borrow().get_object_count() {
                write!(dest, "{}:", object)?;
                for choice in 0..self.props.borrow().get_object_count() {
                    if self.is_choice_enabled(property, object, choice) {
                        write!(
                            dest,
                            "{} ",
                            self.props
                                .borrow()
                                .get_property_choice_name_by_nr(property, choice)
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
    fn is_solution(&self) -> bool {
        for object in 0..self.props.borrow().get_object_count() {
            for property in 0..self.props.borrow().get_property_count() {
                if !self.is_determined(object, property) {
                    return false;
                }
            }
        }
        true
    }
    fn split(&self) -> Vec<Self> {
        for property in 0..self.props.borrow().get_property_count() {
            for object in 0..self.props.borrow().get_object_count() {
                if !self.is_determined(property, object) {
                    let mut result = vec![];
                    for choice in 0..self.props.borrow().get_object_count() {
                        if self.is_choice_enabled(property, object, choice) {
                            let mut z = self.clone();
                            z.determine_choice(property, object, choice);
                            result.push(z);
                        }
                    }
                    return result;
                }
            }
        }
        vec![]
    }

    fn apply_facts(&mut self, change_counter: &mut i32) -> bool {
        let props2 = self.props.clone();
        for f in &props2.borrow().facts {
            for property in 0..props2.borrow().get_property_count() {
                for object in 0..props2.borrow().get_object_count() {
                    if self.is_determined(property, object) {
                        if !self.apply_half_fact(
                            property,
                            object,
                            f.choice1,
                            f.choice2,
                            change_counter,
                        ) {
                            return false;
                        }
                        if !self.apply_half_fact(
                            property,
                            object,
                            f.choice2,
                            f.choice1,
                            change_counter,
                        ) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn apply_predicates(&mut self) -> bool {
        let props2 = self.props.clone();
        for pred in &props2.borrow().predicates {
            let (p1, ch1) = pred.choice1;
            let (p2, ch2) = pred.choice2;
            for object1 in 0..props2.borrow().get_object_count() {
                for object2 in 0..props2.borrow().get_object_count() {
                    if self.is_determined(p1, object1)
                        && self.is_determined(p2, object2)
                        && self.is_choice_enabled(p1, object1, ch1)
                        && self.is_choice_enabled(p2, object2, ch2)
                        && self.is_determined(pred.property3, object1)
                        && self.is_determined(pred.property4, object2)
                    {
                        let ch3 = self.find_determined(pred.property3, object1);

                        let ch4 = self.find_determined(pred.property4, object2);
                        let s1 = props2
                            .borrow()
                            .get_property_choice_name_by_nr(pred.property3, ch3);
                        let s2 = props2
                            .borrow()
                            .get_property_choice_name_by_nr(pred.property4, ch4);
                        if !(pred.test)(s1, s2) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn apply_permutations(&mut self, change_counter: &mut i32) -> bool {
        let props2 = self.props.clone();
        for property in 0..props2.borrow().get_property_count() {
            for object in 0..props2.borrow().get_object_count() {
                if self.is_determined(property, object) {
                    let choice = self.find_determined(property, object);
                    for object2 in 0..props2.borrow().get_object_count() {
                        if object != object2 && self.is_choice_enabled(property, object2, choice) {
                            *change_counter += 1;
                            self.set_choice_enabled(property, object2, choice, false);
                        }
                    }
                }
            }
        }
        true
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
    zebra: Rc<RefCell<ZebraProperties<'a>>>,
}

impl<'a> ZebraBuilder<'a> {
    pub fn new() -> ZebraBuilder<'a> {
        ZebraBuilder {
            zebra: Rc::new(RefCell::new(ZebraProperties {
                object_count: 0,
                properties: vec![],
                options: vec![],
                facts: vec![],
                predicates: vec![],
            })),
        }
    }

    pub fn build(&mut self) -> Zebra<'a> {
        let zebra = self.zebra.borrow();
        for i in 0..zebra.options.len() {
            let chlen = zebra.options[i].len();
            if chlen != zebra.object_count {
                panic!(
                    "Invalid number choices in property {} is {} should be {}",
                    zebra.get_property_name(i),
                    chlen,
                    zebra.object_count
                );
            }
        }
        let bit_count =
            zebra.get_property_count() * zebra.get_object_count() * zebra.get_object_count();
        let mut result = Zebra {
            props: Rc::clone(&self.zebra),
            values: bitvec![1; bit_count],
        };
        result.fix_first_property();
        result
    }

    pub fn set_object_count(&mut self, size: usize) -> &mut Self {
        self.zebra.borrow_mut().object_count = size;
        self
    }

    pub fn fact(
        &mut self,
        property1: &'a str,
        choice1: &'a str,
        property2: &'a str,
        choice2: &'a str,
    ) -> &mut Self {
        let choice1 = self.ensure_choice(property1, choice1);
        let choice2 = self.ensure_choice(property2, choice2);
        self.zebra
            .borrow_mut()
            .facts
            .push(Fact { choice1, choice2 });
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
        test: Box<dyn Fn(&str, &str) -> bool>,
    ) -> &mut Self {
        let choice1 = self.ensure_choice(property1, choice1);
        let choice2 = self.ensure_choice(property2, choice2);
        let property3 = self.ensure_property(property3);
        let property4 = self.ensure_property(property4);
        self.zebra.borrow_mut().predicates.push(Predicate {
            choice1,
            choice2,
            property3,
            property4,
            test,
        });
        self
    }

    pub fn choice(&mut self, property: &'a str, choices: Vec<&'a str>) -> &mut Self {
        for ch in choices {
            self.ensure_choice(property, ch);
        }
        self
    }

    fn ensure_property(&mut self, property: &'a str) -> usize {
        let pr = self.zebra.borrow().get_property(property);

        if pr.is_none() {
            self.zebra.borrow_mut().properties.push(property);
            self.zebra.borrow_mut().options.push(vec![]);
            self.zebra.borrow().get_property_count() - 1
        } else {
            pr.unwrap()
        }
    }

    fn ensure_choice(&mut self, property: &'a str, choice: &'a str) -> (usize, usize) {
        let prix = self.ensure_property(property);
        let ch = self.zebra.borrow().get_property_choice(property, choice);
        let chix = if ch.is_none() {
            self.zebra.borrow_mut().options[prix].push(choice);
            self.zebra.borrow_mut().options[prix].len() - 1
        } else {
            ch.unwrap()
        };

        (prix, chix)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn make_zebra<'a>() -> Zebra<'a> {
        ZebraBuilder::new()
            .set_object_count(3)
            .fact("p1", "p1a", "p2", "p2a")
            .fact("p1", "p1b", "p2", "p2b")
            .choice("p1", vec!["p1c"])
            .choice("p2", vec!["p2c"])
            .choice("p3", vec!["p3a", "p3b", "p3c"])
            .choice("p4", vec!["p4a", "p4b", "p4c"])
            .predicate(
                "p1",
                "p1a",
                "p2",
                "p2c",
                "p4",
                "p4",
                Box::new(&|ch1: &str, ch2: &str| ch1 == "p4a" && ch2 == "p4c"),
            )
            .build()
    }

    #[test]
    fn test_zebra_builder() {
        let zebra = make_zebra();

        assert_eq!(4, zebra.props.borrow().get_property_count());

        assert_eq!(3, zebra.props.borrow().get_object_count());

        assert_eq!("p1", zebra.props.borrow().get_property_name(0));

        assert_eq!(Some(0), zebra.props.borrow().get_property("p1"));

        assert_eq!(
            "p1a",
            zebra.props.borrow().get_property_choice_name("p1", 0)
        );

        assert_eq!(
            Some(0),
            zebra.props.borrow().get_property_choice("p1", "p1a")
        );
    }

    #[test]
    fn test_state_is_determined() {
        let zebra = make_zebra();

        assert_eq!(true, zebra.is_determined(0, 0));
        assert_eq!(true, zebra.is_determined(0, 1));
        assert_eq!(true, zebra.is_determined(0, 2));
        assert_eq!(false, zebra.is_determined(1, 0));
    }

    #[test]
    fn test_state_is_solution() {
        let mut zebra = make_zebra();
        assert_eq!(false, zebra.is_solution());

        for prop in 1..4 {
            for ch in 0..3 {
                zebra.determine_choice(prop, ch, ch);
            }
        }

        print!("{}", zebra);
        assert_eq!(true, zebra.is_solution());
    }

    #[test]
    fn test_split() {
        let zebra = make_zebra();
        let zz = zebra.split();

        assert_eq!(3, zz.len());
        assert_eq!(true, zz[0].is_determined(1, 0));
        assert_eq!(true, zz[0].is_choice_enabled(1, 0, 0));

        assert_eq!(true, zz[1].is_determined(1, 0));
        assert_eq!(true, zz[1].is_choice_enabled(1, 1, 1));

        assert_eq!(true, zz[2].is_determined(1, 0));
        assert_eq!(true, zz[2].is_choice_enabled(1, 2, 2));
    }

    #[test]
    fn test_apply_facts() {
        let mut zebra = make_zebra();
        println!("{}", zebra);
        let mut change_counter = 0;
        assert_eq!(true, zebra.apply_facts(&mut change_counter));
        println!("{}", zebra);
        assert_eq!(true, zebra.is_determined(1, 0));
        assert_eq!(true, zebra.is_choice_enabled(1, 0, 0));
        assert_eq!(true, zebra.is_determined(1, 0));
        assert_eq!(true, zebra.is_choice_enabled(1, 1, 1));
        assert_eq!(2, change_counter);
    }

    #[test]
    fn test_apply_permutations() {
        let mut zebra = make_zebra();
        zebra.set_choice_enabled(1, 0, 1, false);

        zebra.set_choice_enabled(1, 0, 2, false);
        println!("{}", zebra);
        let mut change_counter = 0;
        assert_eq!(true, zebra.apply_permutations(&mut change_counter));
        println!("{}", zebra);
        assert_eq!(2, change_counter);
    }

    #[test]
    fn test_apply_predicates() {
        let mut zebra = make_zebra();
        zebra.set_choice_enabled(1, 1, 0, false);

        zebra.set_choice_enabled(1, 1, 1, false);
        zebra.set_choice_enabled(3, 0, 1, false);

        zebra.set_choice_enabled(3, 0, 2, false);
        zebra.set_choice_enabled(3, 1, 0, false);

        zebra.set_choice_enabled(3, 1, 1, false);
        println!("{}", zebra);

        assert_eq!(true, zebra.apply_predicates());
    }
}
