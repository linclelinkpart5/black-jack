
use std::ops::Range;
use std::iter::{Iterator, FromIterator};
use containers::{Data, DType};
use num::*;
use num_traits::*;


pub trait LumberJackData: Num + Integer + Float {
    fn dtype(&self) -> DType;
}



#[derive(Clone, Debug, PartialEq)]
pub struct Series<T>
    where 
        T: LumberJackData
{
    pub name: Option<String>,
    pub data: Data<T>
}

impl<T> Series<T>
    where 
        T: LumberJackData
{
    pub fn arange<I: Integer>(start: I, stop: I) -> Series<T>
        where I:
            Integer,
            Range<I>: Iterator, 
            Vec<T>: FromIterator<<Range<I> as Iterator>::Item>, 
            Vec<I>: From<Vec<I>>
    {
        let data: Vec<T> = (start..stop).collect::<Vec<T>>().into();
        Series { name: None, data: Data::Integer(data)}
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn from_vec<A>(vec: Vec<A>) -> Self 
        where A: LumberJackData, Data<T>: FromIterator<A>
    {
        let data = Data::from_iter(vec.into_iter());
        Series { name: None, data }
    }
}