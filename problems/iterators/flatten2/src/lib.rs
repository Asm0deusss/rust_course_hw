#![forbid(unsafe_code)]

pub struct FlatMap<InIter, F, OutIter>
where
    InIter: Iterator,
    F: FnMut(InIter::Item) -> OutIter,
    OutIter: IntoIterator,
{
    iter: std::iter::FlatMap<InIter, OutIter, F>,
}

impl<InIter, F, OutIter> FlatMap<InIter, F, OutIter>
where
    InIter: Iterator,
    F: FnMut(InIter::Item) -> OutIter,
    OutIter: IntoIterator,
{
    fn new(outer: InIter, function: F) -> Self {
        Self {
            iter: (outer.flat_map(function)),
        }
    }
}

impl<InIter, F, OutIter> Iterator for FlatMap<InIter, F, OutIter>
where
    InIter: Iterator,
    F: FnMut(InIter::Item) -> OutIter,
    OutIter: IntoIterator,
{
    type Item = <OutIter>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub fn flat_map<InputIterator, Mapping, OutputIterator>(
    iter: InputIterator,
    f: Mapping,
) -> FlatMap<InputIterator, Mapping, OutputIterator>
where
    InputIterator: Iterator,
    Mapping: FnMut(InputIterator::Item) -> OutputIterator,
    OutputIterator: IntoIterator,
{
    FlatMap::new(iter, f)
}
