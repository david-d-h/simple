use std::iter::Peekable;

pub(crate) trait Consume<T: Iterator> {
    fn consume(&mut self, expected: T::Item) -> Option<&mut Self>;
}

impl<T: Iterator> Consume<T> for Peekable<T>
    where T::Item: PartialEq,
{
    fn consume(&mut self, expected: T::Item) -> Option<&mut Self> {
        let _ = self.next_if(|i| *i == expected)?;
        Some(self)
    }
}