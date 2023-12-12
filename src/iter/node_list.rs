/// Allow iterating over a [`NodeList`].
pub struct IterableNodeList<'a>(pub &'a web_sys::NodeList);

impl<'a> IntoIterator for IterableNodeList<'a> {
    type Item = web_sys::Node;
    type IntoIter = NodeListIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.0)
    }
}

#[doc(hidden)]
pub struct NodeListIter<'a> {
    list: &'a web_sys::NodeList,
    index: u32,
}

impl<'a> NodeListIter<'a> {
    pub fn new(list: &'a web_sys::NodeList) -> Self {
        Self { list, index: 0 }
    }
}

impl<'a> Iterator for NodeListIter<'a> {
    type Item = web_sys::Node;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.list.item(self.index);
        self.index += 1;
        next
    }
}
