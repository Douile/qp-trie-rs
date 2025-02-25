use core::borrow::Borrow;
use core::fmt;
use core::ops::Index;

use iter::Iter;
use node::Node;
use wrapper::{BStr, BString};

pub struct SubTrie<'a, K: 'a, V: 'a> {
    pub(crate) root: Option<&'a Node<K, V>>,
}

impl<'a, K: fmt::Debug, V: fmt::Debug> fmt::Debug for SubTrie<'a, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.root {
            Some(node) => f.debug_map().entries(node.iter()).finish(),
            None => f.debug_map().finish(),
        }
    }
}

impl<'a, K: 'a, V: 'a> IntoIterator for SubTrie<'a, K, V> {
    type IntoIter = Iter<'a, K, V>;
    type Item = (&'a K, &'a V);

    fn into_iter(self) -> Self::IntoIter {
        self.root.map(Node::iter).unwrap_or_default()
    }
}

impl<'a, K: 'a, V: 'a> SubTrie<'a, K, V> {
    /// Returns true if the subtrie has no entries.
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

impl<'a, K: Borrow<[u8]>, V> SubTrie<'a, K, V> {
    pub fn iter(&self) -> Iter<'a, K, V> {
        match self.root {
            Some(node) => node.iter(),
            None => Iter::default(),
        }
    }

    pub fn iter_prefix<L: Borrow<[u8]>>(&self, prefix: L) -> Iter<'a, K, V> {
        match self.root.and_then(|node| node.get_prefix(prefix.borrow())) {
            Some(node) => node.iter(),
            None => Iter::default(),
        }
    }

    pub fn subtrie<L: ?Sized>(&self, prefix: &L) -> SubTrie<'a, K, V>
    where
        L: Borrow<[u8]>,
    {
        SubTrie {
            root: self.root.and_then(|node| node.get_prefix(prefix.borrow())),
        }
    }

    pub fn get<L: ?Sized>(&self, key: &L) -> Option<&'a V>
    where
        L: Borrow<[u8]>,
    {
        self.root
            .and_then(|node| node.get(key.borrow()))
            .map(|leaf| &leaf.val)
    }
}

impl<'b, V> SubTrie<'b, BString, V> {
    /// Convenience function for getting with a string.
    pub fn get_str<'a, Q: ?Sized>(&'a self, key: &Q) -> Option<&'a V>
    where
        Q: Borrow<str>,
    {
        self.get(AsRef::<BStr>::as_ref(key.borrow()))
    }

    /// Convenience function for viewing subtries wit a string prefix.
    pub fn subtrie_str<'a, Q: ?Sized>(&'a self, prefix: &Q) -> SubTrie<'a, BString, V>
    where
        Q: Borrow<str>,
    {
        self.subtrie(AsRef::<BStr>::as_ref(prefix.borrow()))
    }
}

impl<'a, K: Borrow<[u8]>, V, L: Borrow<[u8]>> Index<L> for SubTrie<'a, K, V> {
    type Output = V;

    fn index(&self, key: L) -> &V {
        self.get(&key).unwrap()
    }
}
