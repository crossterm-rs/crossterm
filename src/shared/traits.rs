/// This trait is used for creating an instance of an concrete implementation from an base trait. 
/// This trait allows the output to be different in size.
pub trait Construct
{
    fn new() -> Box<Self> where Self: Sized;    
} 