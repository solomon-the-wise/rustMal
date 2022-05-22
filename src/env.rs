use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Error;
use std::rc::Rc;
use crate::MalType;
use crate::MalType::{Num, PrFunc};
use crate::reader::BoxResult;
use crate::types::{MalList, PrimitiveFuncs};

pub type MalEnv = HashMap<String, MalType>;
#[derive(Debug)]
pub struct Env{
    pub(crate) outer:  Option<RcEnv>,
    pub(crate) data: RefCell<MalEnv>
}
pub type RcEnv = Rc<Env>;

pub(crate) trait Environment{
    fn set(&self, symbol: String, m: MalType);
    fn find(&self, symbol: String) -> Option<MalType>;
    fn get(&self, symbol: String) -> BoxResult<MalType>;
    fn new_env(&self) -> RcEnv;
    fn new_env_with_binds(&self, binds: MalList, exprs: MalList) -> RcEnv;
}
impl Environment for RcEnv{
    fn set(&self, symbol: String, m: MalType){
        self.data.borrow_mut().insert(symbol, m);
    }
    fn find(&self, symbol: String) -> Option<MalType> {
        match self.data.borrow().get(&symbol) {
            Some(t) => Some(t.clone()),
            None => match self.outer.borrow() {
                Some(o) => o.find(symbol),
                None => None
            }
        }
    }
    fn get(&self, symbol: String) -> BoxResult<MalType> {
        Ok(self.find(symbol).ok_or(Error)?)

    }
    fn new_env(&self) -> RcEnv{
        Rc::new(Env {
            outer: Some(self.clone()),
            data: RefCell::new(HashMap::new())
        })
    }
    fn new_env_with_binds(&self, binds: MalList, exprs: MalList) -> RcEnv{
        let result = self.new_env();
        let mut s1 = binds.into_iter().peekable();
        let mut s2 = exprs.into_iter().peekable();
        while s1.peek().is_some() && s2.peek().is_some(){
            result.set(s1.next().unwrap().to_string(), s2.next().unwrap())
        }
        result

    }



}
