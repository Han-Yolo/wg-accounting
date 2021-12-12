use std::cmp;
use std::fmt;

pub struct Account {
    acronym: String,
    name: String,
}
impl Account {
    pub fn new(captures: &regex::Captures) -> Self {
        Account {
            acronym: captures.name("acronym").unwrap().as_str().to_owned(),
            name: captures.name("name").unwrap().as_str().to_owned(),
        }
    }
    pub fn acronym(&self) -> &String {
        &self.acronym
    }
    pub fn name(&self) -> &String {
        &self.name
    }
}
impl cmp::PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.acronym == other.acronym
    }
}
impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.acronym, self.name)
    }
}

pub fn find_index(acronym: &String, accounts: &Vec<Account>) -> usize {
    accounts
        .iter()
        .position(|account: &Account| account.acronym() == acronym)
        .unwrap()
}
