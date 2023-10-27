#[derive(PartialEq)]
pub enum PageType {
    Data = 0b1,
    Code = 0b10,
    All = 0b100,
}

#[derive(PartialEq, Clone, Copy)]
pub enum PrivilegeType {
    Machine = 0,
    Superviser = 1,
    User = 2,
}

pub trait IPageTable {
    fn new()->Self;
    fn map(&self, va : usize, pa : usize, page_type : PageType, privilege : PrivilegeType);
    fn from_other(other : &Self)->Self;
    fn val(&self)->usize;
}

pub trait IEnvironment {
    fn set_page<T : IPageTable>(&mut self, page : &T);
}