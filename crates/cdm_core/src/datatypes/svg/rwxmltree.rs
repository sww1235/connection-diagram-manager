/*!

A reimplementation of `[roxmltree::Document]`. The two major changes are owned datatypes where
needed instead of references, and mutability.

Some names and functions have changed slightly, but attempts are made to keep the interface as
close as possible to `roxmltree`.

This tree still relies on the XML parsing from `roxmltree`.

The mutability is used for editing the tree in specific ways, mainly to insert text from the
application into symbols.


*/

use core::fmt;
use core::ops::Range;
use core::num::NonZeroU32;
use core::cmp::Ordering;
use core::hash::{Hash, Hasher};


/// The <http://www.w3.org/XML/1998/namespace> URI.
pub const NS_XML_URI: &str = "http://www.w3.org/XML/1998/namespace";
/// The prefix 'xml', which is by definition bound to NS_XML_URI
const NS_XML_PREFIX: &str = "xml";

/// The <http://www.w3.org/2000/xmlns/> URI.
pub const NS_XMLNS_URI: &str = "http://www.w3.org/2000/xmlns/";
/// The string 'xmlns', which is used to declare new namespaces
const XMLNS: &str = "xmlns";

/// An XML tree
pub struct Tree {
    text: String,
    nodes: Vec<NodeData>,
    attributes: Vec<AttributeData>,
    namespaces: Namespaces,
}

impl Tree {
    /// Returns the root node.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse("<e/>").unwrap();
    /// assert!(tree.root().is_root());
    /// assert!(tree.root().first_child().unwrap().has_tag_name("e"));
    /// ```
    #[inline]
    pub fn root(&self) -> Node {
        Node {
            id: NodeId::new(0),
            d: self.nodes[0],
            tree: self,
        }
    }

    /// Returns the node of the tree with the given NodeId.
    ///
    /// Note: NodeId::new(0) represents the root node
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse("\
    /// <p>
    ///     text
    /// </p>
    /// ").unwrap();
    ///
    /// use roxmltree::NodeId;
    /// assert_eq!(tree.get_node(NodeId::new(0)).unwrap(), tree.root());
    /// assert_eq!(tree.get_node(NodeId::new(1)), tree.descendants().find(|n| n.has_tag_name("p")));
    /// assert_eq!(tree.get_node(NodeId::new(2)), tree.descendants().find(|n| n.is_text()));
    /// assert_eq!(tree.get_node(NodeId::new(3)), None);
    /// ```
    #[inline]
    pub fn get_node(& self, id: NodeId) -> Option<Node> {
        self.nodes.get(id.get_usize()).map(|data| Node {
            id,
            d: data,
            tree: self,
        })
    }

    /// Returns the root element of the tree.
    ///
    /// Unlike `root`, will return a first element node.
    ///
    /// The root element always exists.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse("<!-- comment --><e/>").unwrap();
    /// assert!(tree.root_element().has_tag_name("e"));
    /// ```
    #[inline]
    pub fn root_element(&self) -> Node {
        // `expect` is safe, because the `Tree` is guarantee to have at least one element.
        self.root()
            .first_element_child()
            .expect("XML trees must contain a root element")
    }

    /// Returns an iterator over tree's descendant nodes.
    ///
    /// Shorthand for `tree.root().descendants()`.
    #[inline]
    pub fn descendants(&self) -> Descendants {
        self.root().descendants()
    }

    /// Calculates `TextPos` in the original tree from position in bytes.
    ///
    /// **Note:** this operation is expensive.
    ///
    /// # Examples
    ///
    /// ```
    /// use roxmltree::*;
    ///
    /// let tree = Tree::parse("\
    /// <!-- comment -->
    /// <e/>"
    /// ).unwrap();
    ///
    /// assert_eq!(tree.text_pos_at(10), TextPos::new(1, 11));
    /// assert_eq!(tree.text_pos_at(9999), TextPos::new(2, 5));
    /// ```
    #[inline]
    pub fn text_pos_at(&self, pos: usize) -> TextPos {
        tokenizer::Stream::new(self.text).gen_text_pos_from(pos)
    }

    /// Returns the input text of the original tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use roxmltree::*;
    ///
    /// let tree = Tree::parse("<e/>").unwrap();
    ///
    /// assert_eq!(tree.input_text(), "<e/>");
    /// ```
    #[inline]
    pub fn input_text(&self) -> String {
        self.text.clone()
    }
}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if !self.root().has_children() {
            return write!(f, "Tree []");
        }

        macro_rules! writeln_indented {
            ($depth:expr, $f:expr, $fmt:expr) => {
                for _ in 0..$depth { write!($f, "    ")?; }
                writeln!($f, $fmt)?;
            };
            ($depth:expr, $f:expr, $fmt:expr, $($arg:tt)*) => {
                for _ in 0..$depth { write!($f, "    ")?; }
                writeln!($f, $fmt, $($arg)*)?;
            };
        }

        fn print_into_iter<
            T: fmt::Debug,
            E: ExactSizeIterator<Item = T>,
            I: IntoIterator<Item = T, IntoIter = E>,
        >(
            prefix: &str,
            data: I,
            depth: usize,
            f: &mut fmt::Formatter,
        ) -> Result<(), fmt::Error> {
            let data = data.into_iter();
            if data.len() == 0 {
                return Ok(());
            }

            writeln_indented!(depth, f, "{}: [", prefix);
            for v in data {
                writeln_indented!(depth + 1, f, "{:?}", v);
            }
            writeln_indented!(depth, f, "]");

            Ok(())
        }

        fn print_children(
            parent: Node,
            depth: usize,
            f: &mut fmt::Formatter,
        ) -> Result<(), fmt::Error> {
            for child in parent.children() {
                if child.is_element() {
                    writeln_indented!(depth, f, "Element {{");
                    writeln_indented!(depth, f, "    tag_name: {:?}", child.tag_name());
                    print_into_iter("attributes", child.attributes(), depth + 1, f)?;
                    print_into_iter("namespaces", child.namespaces(), depth + 1, f)?;

                    if child.has_children() {
                        writeln_indented!(depth, f, "    children: [");
                        print_children(child, depth + 2, f)?;
                        writeln_indented!(depth, f, "    ]");
                    }

                    writeln_indented!(depth, f, "}}");
                } else {
                    writeln_indented!(depth, f, "{:?}", child);
                }
            }

            Ok(())
        }

        writeln!(f, "Tree [")?;
        print_children(self.root(), 1, f)?;
        writeln!(f, "]")?;

        Ok(())
    }
}

/// A list of supported node types.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeType {
    /// The root node of the `Tree`.
    Root,
    /// An element node.
    ///
    /// Only an element can have a tag name and attributes.
    Element,
    /// A processing instruction.
    PI,
    /// A comment node.
    Comment,
    /// A text node.
    Text,
}

/// A processing instruction.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PI {
    pub target: String,
    pub value: Option<String>,
}

/// A short range.
///
/// Just like Range, but only for `u32` and copyable.
#[derive(Clone, Copy, Debug)]
struct ShortRange {
    start: u32,
    end: u32,
}

impl From<Range<usize>> for ShortRange {
    #[inline]
    fn from(range: Range<usize>) -> Self {
        debug_assert!(range.start <= u32::MAX as usize);
        debug_assert!(range.end <= u32::MAX as usize);
        ShortRange::new(range.start as u32, range.end as u32)
    }
}

impl ShortRange {
    #[inline]
    fn new(start: u32, end: u32) -> Self {
        ShortRange { start, end }
    }

    #[inline]
    fn to_urange(self) -> Range<usize> {
        self.start as usize..self.end as usize
    }
}

/// A node ID stored as `u32`.
///
/// An index into a `Tree`-internal `Vec`.
///
/// Note that this value should be used with care since `roxmltree` doesn't
/// check that `NodeId` actually belongs to a selected `Tree`.
/// So you can end up in a situation, when `NodeId` produced by one `Tree`
/// is used to select a node in another `Tree`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeId(NonZeroU32);

impl NodeId {
    /// Construct a new `NodeId` from a `u32`.
    #[inline]
    pub fn new(id: u32) -> Self {
        debug_assert!(id < u32::MAX);

        // We are using `NonZeroU32` to reduce overhead of `Option<NodeId>`.
        NodeId(NonZeroU32::new(id + 1).unwrap())
    }

    /// Returns the `u32` representation of the `NodeId`.
    #[inline]
    pub fn get(self) -> u32 {
        self.0.get() - 1
    }

    /// Returns the `usize` representation of the `NodeId`.
    #[inline]
    pub fn get_usize(self) -> usize {
        self.get() as usize
    }
}

impl From<u32> for NodeId {
    #[inline]
    fn from(id: u32) -> Self {
        NodeId::new(id)
    }
}

impl From<usize> for NodeId {
    #[inline]
    fn from(id: usize) -> Self {
        // We already checked that `id` is limited by u32::MAX.
        debug_assert!(id <= u32::MAX as usize);
        NodeId::new(id as u32)
    }
}

#[derive(Debug)]
enum NodeKind {
    Root,
    Element {
        tag_name: ExpandedNameIndexed,
        attributes: ShortRange,
        namespaces: ShortRange,
    },
    PI(PI),
    Comment(String),
    Text(String),
}

#[derive(Debug)]
struct NodeData {
    parent: Option<NodeId>,
    prev_sibling: Option<NodeId>,
    next_subtree: Option<NodeId>,
    last_child: Option<NodeId>,
    kind: NodeKind,
    range: Range<usize>,
}


#[derive(Clone, Debug)]
struct AttributeData {
    name: ExpandedNameIndexed,
    value: String,
    range: Range<usize>,
    qname_len: u16,
    eq_len: u8, // includes any surrounding spaces
}

/// An attribute.
#[derive(Clone)]
pub struct Attribute {
    //TODO: is this even needed?
    tree: Tree,
    data: AttributeData,
}

impl Attribute {
    /// Returns attribute's namespace URI.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse(
    ///     "<e xmlns:n='http://www.w3.org' a='b' n:a='c'/>"
    /// ).unwrap();
    ///
    /// assert_eq!(tree.root_element().attributes().nth(0).unwrap().namespace(), None);
    /// assert_eq!(tree.root_element().attributes().nth(1).unwrap().namespace(), Some("http://www.w3.org"));
    /// ```
    #[inline]
    pub fn namespace(&self) -> Option<String> {
        self.data.name.namespace(self.tree).map(Namespace::uri)
    }

    /// Returns attribute's name.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse(
    ///     "<e xmlns:n='http://www.w3.org' a='b' n:a='c'/>"
    /// ).unwrap();
    ///
    /// assert_eq!(tree.root_element().attributes().nth(0).unwrap().name(), "a");
    /// assert_eq!(tree.root_element().attributes().nth(1).unwrap().name(), "a");
    /// ```
    #[inline]
    pub fn name(&self) -> String {
        self.data.name.local_name.clone()
    }

    /// Returns attribute's value.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse(
    ///     "<e xmlns:n='http://www.w3.org' a='b' n:a='c'/>"
    /// ).unwrap();
    ///
    /// assert_eq!(tree.root_element().attributes().nth(0).unwrap().value(), "b");
    /// assert_eq!(tree.root_element().attributes().nth(1).unwrap().value(), "c");
    /// ```
    #[inline]
    pub fn value(&self) -> String {
        self.data.value.clone()
    }

    /// Returns attribute's value storage.
    ///
    /// Useful when you need a more low-level access to an allocated string.
    #[inline]
    pub fn value_storage(&self) -> String {
        self.data.value.clone()
    }

    /// Returns attribute's range in bytes in the original tree.
    ///
    /// You can calculate a human-readable text position via [Tree::text_pos_at].
    ///
    /// ```text
    /// <e n:attr='value'/>
    ///    ^^^^^^^^^^^^^^
    /// ```
    #[inline]
    pub fn range(&self) -> Range<usize> {
        self.data.range.clone()
    }

    /// Returns attribute's qname's range in bytes in the original tree.
    ///
    /// ```text
    /// <e n:attr='value'/>
    ///    ^^^^^^
    /// ```
    ///
    /// To reduce memory usage the qname length is limited by u16::MAX.
    /// If the attribute exceeds that limit then the end of the returned range will be incorrect.
    #[inline]
    pub fn range_qname(&self) -> Range<usize> {
        let end = self.data.range.start + usize::from(self.data.qname_len);
        self.data.range.start..end
    }

    /// Returns attribute's value's range in bytes in the original tree, excluding the surrounding quotes.
    ///
    /// If the attribute's value is an empty string then the `start` and `end` of this `Range` are equal, and indicate the closing quote.
    ///
    /// ```text
    /// <e n:attr='value'/>
    ///            ^^^^^
    /// ```
    ///
    /// To reduce memory usage the qname length is limited by u16::MAX,
    /// and the number of spaces around the equal sign is limited by u8::MAX.
    /// If the attribute exceeds those limits then the start of the returned range will be incorrect.
    #[inline]
    pub fn range_value(&self) -> Range<usize> {
        // +1 on start and -1 on end are to exclude the quotes around the value (all valid quotes are 1 byte)
        let start = self.data.range.start + usize::from(self.data.qname_len) + usize::from(self.data.eq_len) + 1;
        let end = self.data.range.end - 1;
        start..end
    }
}

impl PartialEq for Attribute {
    #[inline]
    fn eq(&self, other: &Attribute) -> bool {
        self.data.name.as_expanded_name(self.tree) == other.data.name.as_expanded_name(other.tree)
            && self.data.value == other.data.value
    }
}

impl fmt::Debug for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "Attribute {{ name: {:?}, value: {:?} }}",
            self.data.name.as_expanded_name(self.tree),
            self.data.value
        )
    }
}

/// A namespace.
///
/// Contains URI and *prefix* pair.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Namespace {
    name: Option<String>,
    uri: String,
}

impl Namespace {
    /// Returns namespace name/prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse(
    ///     "<e xmlns:n='http://www.w3.org'/>"
    /// ).unwrap();
    ///
    /// assert_eq!(tree.root_element().namespaces().nth(0).unwrap().name(), Some("n"));
    /// ```
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse(
    ///     "<e xmlns='http://www.w3.org'/>"
    /// ).unwrap();
    ///
    /// assert_eq!(tree.root_element().namespaces().nth(0).unwrap().name(), None);
    /// ```
    #[inline]
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    /// Returns namespace URI.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse(
    ///     "<e xmlns:n='http://www.w3.org'/>"
    /// ).unwrap();
    ///
    /// assert_eq!(tree.root_element().namespaces().nth(0).unwrap().uri(), "http://www.w3.org");
    /// ```
    #[inline]
    pub fn uri(&self) -> &str {
        self.uri.as_ref()
    }
}

#[derive(Default)]
struct Namespaces {
    // Deduplicated namespace values used throughout the tree
    values: Vec<Namespace>,
    // Indices into the above in tree order as the tree is parsed
    tree_order: Vec<NamespaceIdx>,
    // Indices into the above sorted by value used for deduplication
    sorted_order: Vec<NamespaceIdx>,
}

//Error here is roxmltree::parse::Error which is private
impl Namespaces {
    fn push_ns(
        &mut self,
        name: Option<&str>,
        uri: String,
    ) -> Result<(), Error> {
        debug_assert_ne!(name, Some(""));

        let idx = match self.sorted_order.binary_search_by(|idx| {
            let value = &self.values[idx.0 as usize];

            (value.name, value.uri.as_ref()).cmp(&(name, uri.as_str()))
        }) {
            Ok(sorted_idx) => self.sorted_order[sorted_idx],
            Err(sorted_idx) => {
                if self.values.len() > u16::MAX as usize {
                    return Err(Error::NamespacesLimitReached);
                }
                let idx = NamespaceIdx(self.values.len() as u16);
                self.values.push(Namespace { name, uri });
                self.sorted_order.insert(sorted_idx, idx);
                idx
            }
        };

        self.tree_order.push(idx);

        Ok(())
    }

    #[inline]
    fn push_ref(&mut self, tree_idx: usize) {
        let idx = self.tree_order[tree_idx];
        self.tree_order.push(idx);
    }

    #[inline]
    fn exists(&self, start: usize, prefix: Option<&str>) -> bool {
        self.tree_order[start..]
            .iter()
            .any(|idx| self.values[idx.0 as usize].name == prefix)
    }

    fn shrink_to_fit(&mut self) {
        self.values.shrink_to_fit();
        self.tree_order.shrink_to_fit();
        // Only needed to deduplicate namespaces during parsing.
        self.sorted_order = Vec::new();
    }

    #[inline]
    fn get(&self, idx: NamespaceIdx) -> Namespace {
        &self.values[idx.0 as usize]
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
struct NamespaceIdx(u16);

#[derive(Clone, Debug)]
struct ExpandedNameIndexed {
    namespace_idx: Option<NamespaceIdx>,
    local_name: String,
}

impl ExpandedNameIndexed {
    #[inline]
    fn namespace(&self, tree: &Tree) -> Option<Namespace> {
        self.namespace_idx.map(|idx| tree.namespaces.get(idx))
    }

    #[inline]
    fn as_expanded_name(&self, tree: &Tree) -> ExpandedName {
        ExpandedName {
            uri: self.namespace(tree).map(Namespace::uri),
            name: self.local_name,
        }
    }
}

/// An expanded name.
///
/// Contains an namespace URI and name pair.
#[derive(Clone, PartialEq, Eq)]
pub struct ExpandedName {
    uri: Option<String>,
    name: String,
}

impl ExpandedName {
    /// Returns a namespace URI.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse("<e xmlns='http://www.w3.org'/>").unwrap();
    ///
    /// assert_eq!(tree.root_element().tag_name().namespace(), Some("http://www.w3.org"));
    /// ```
    #[inline]
    pub fn namespace(&self) -> Option<String> {
        self.uri.clone()
    }

    /// Returns a local name.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse("<e/>").unwrap();
    ///
    /// assert_eq!(tree.root_element().tag_name().name(), "e");
    /// ```
    #[inline]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}


impl fmt::Debug for ExpandedName {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.namespace() {
            Some(ns) => write!(f, "{{{}}}{}", ns, self.name),
            None => write!(f, "{}", self.name),
        }
    }
}

impl From<&str> for ExpandedName {
    #[inline]
    fn from(v: &str) -> Self {
        ExpandedName { uri: None, name: v }
    }
}

impl From<(&str, &str)> for ExpandedName {
    #[inline]
    fn from(v: (&str, &str)) -> Self {
        ExpandedName {
            uri: Some(v.0),
            name: v.1,
        }
    }
}

/// A node in a tree.
///
/// # Tree Order
///
/// The implementation of the `Ord` traits for `Node` is based on the concept of *tree-order*.
/// In layman's terms, tree-order is the order in which one would see each element if
/// one opened a tree in a text editor or web browser and scrolled down.
/// Tree-order convention is followed in XPath, CSS Counters, and DOM selectors API
/// to ensure consistent results from selection.
/// One difference in `roxmltree` is that there is the notion of more than one tree
/// in existence at a time. While Nodes within the same tree are in tree-order,
/// Nodes in different trees will be grouped together, but not in any particular
/// order.
///
/// As an example, if we have a Tree `a` with Nodes `[a0, a1, a2]` and a
/// Tree `b` with Nodes `[b0, b1]`, these Nodes in order could be either
/// `[a0, a1, a2, b0, b1]` or `[b0, b1, a0, a1, a2]` and roxmltree makes no
/// guarantee which it will be.
///
/// Tree-order is defined here in the
/// [W3C XPath Recommendation](https://www.w3.org/TR/xpath-3/#id-tree-order)
/// The use of tree-order in DOM Selectors is described here in the
/// [W3C Selectors API Level 1](https://www.w3.org/TR/selectors-api/#the-apis)
#[derive(Clone)]
pub struct Node {
    /// Node's ID.
    id: NodeId,

    //TODO: don't think we need this here with some rearchitecting
    /// The tree containing the node.
    tree: Tree,

    /// Node's data.
    d: NodeData,
}

impl Eq for Node {}

impl PartialEq for Node {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.id, self.tree as *const _) == (other.id, other.tree as *const _)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.id.0, self.tree as *const _).cmp(&(other.id.0, other.tree as *const _))
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.0.hash(state);
        (self.tree as *const Tree).hash(state);
        (self.d as *const NodeData).hash(state);
    }
}

impl Node {
    /// Returns node's type.
    #[inline]
    pub fn node_type(&self) -> NodeType {
        match self.d.kind {
            NodeKind::Root => NodeType::Root,
            NodeKind::Element { .. } => NodeType::Element,
            NodeKind::PI { .. } => NodeType::PI,
            NodeKind::Comment(_) => NodeType::Comment,
            NodeKind::Text(_) => NodeType::Text,
        }
    }

    /// Checks that node is a root node.
    #[inline]
    pub fn is_root(&self) -> bool {
        self.node_type() == NodeType::Root
    }

    /// Checks that node is an element node.
    #[inline]
    pub fn is_element(&self) -> bool {
        self.node_type() == NodeType::Element
    }

    /// Checks that node is a processing instruction node.
    #[inline]
    pub fn is_pi(&self) -> bool {
        self.node_type() == NodeType::PI
    }

    /// Checks that node is a comment node.
    #[inline]
    pub fn is_comment(&self) -> bool {
        self.node_type() == NodeType::Comment
    }

    /// Checks that node is a text node.
    #[inline]
    pub fn is_text(&self) -> bool {
        self.node_type() == NodeType::Text
    }

    /// Returns node's tree.
    #[inline]
    pub fn tree(&self) -> Tree {
        self.tree
    }

    /// Returns node's tag name.
    ///
    /// Returns an empty name with no namespace if the current node is not an element.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse("<e xmlns='http://www.w3.org'/>").unwrap();
    ///
    /// assert_eq!(tree.root_element().tag_name().namespace(), Some("http://www.w3.org"));
    /// assert_eq!(tree.root_element().tag_name().name(), "e");
    /// ```
    #[inline]
    pub fn tag_name(&self) -> ExpandedName {
        match self.d.kind {
            NodeKind::Element { ref tag_name, .. } => tag_name.as_expanded_name(self.tree),
            _ => "".into(),
        }
    }

    /// Checks that node has a specified tag name.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse("<e xmlns='http://www.w3.org'/>").unwrap();
    ///
    /// assert!(tree.root_element().has_tag_name("e"));
    /// assert!(tree.root_element().has_tag_name(("http://www.w3.org", "e")));
    ///
    /// assert!(!tree.root_element().has_tag_name("b"));
    /// assert!(!tree.root_element().has_tag_name(("http://www.w4.org", "e")));
    /// ```
    pub fn has_tag_name<N>(&self, name: N) -> bool
    where
        N: Into<ExpandedName>,
    {
        let name = name.into();

        match self.d.kind {
            NodeKind::Element { ref tag_name, .. } => match name.namespace() {
                Some(_) => tag_name.as_expanded_name(self.tree) == name,
                None => tag_name.local_name == name.name,
            },
            _ => false,
        }
    }

    /// Returns node's default namespace URI.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse("<e xmlns='http://www.w3.org'/>").unwrap();
    ///
    /// assert_eq!(tree.root_element().default_namespace(), Some("http://www.w3.org"));
    /// ```
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse("<e xmlns:n='http://www.w3.org'/>").unwrap();
    ///
    /// assert_eq!(tree.root_element().default_namespace(), None);
    /// ```
    pub fn default_namespace(&self) -> Option<String> {
        self.namespaces()
            .find(|ns| ns.name.is_none())
            .map(|v| v.uri.as_ref())
    }

    /// Returns a prefix for a given namespace URI.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse("<e xmlns:n='http://www.w3.org'/>").unwrap();
    ///
    /// assert_eq!(tree.root_element().lookup_prefix("http://www.w3.org"), Some("n"));
    /// ```
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse("<e xmlns:n=''/>").unwrap();
    ///
    /// assert_eq!(tree.root_element().lookup_prefix(""), Some("n"));
    /// ```
    pub fn lookup_prefix(&self, uri: &str) -> Option<String> {
        if uri == NS_XML_URI {
            return Some(NS_XML_PREFIX);
        }

        self.namespaces()
            .find(|ns| &*ns.uri == uri)
            .map(|v| v.name)
            .unwrap_or(None)
    }

    /// Returns an URI for a given prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse("<e xmlns:n='http://www.w3.org'/>").unwrap();
    ///
    /// assert_eq!(tree.root_element().lookup_namespace_uri(Some("n")), Some("http://www.w3.org"));
    /// ```
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse("<e xmlns='http://www.w3.org'/>").unwrap();
    ///
    /// assert_eq!(tree.root_element().lookup_namespace_uri(None), Some("http://www.w3.org"));
    /// ```
    pub fn lookup_namespace_uri(&self, prefix: Option<&str>) -> Option<&String> {
        self.namespaces()
            .find(|ns| ns.name == prefix)
            .map(|v| v.uri.as_ref())
    }

    /// Returns element's attribute value.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse("<e a='b'/>").unwrap();
    ///
    /// assert_eq!(tree.root_element().attribute("a"), Some("b"));
    /// ```
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse(
    ///     "<e xmlns:n='http://www.w3.org' a='b' n:a='c'/>"
    /// ).unwrap();
    ///
    /// assert_eq!(tree.root_element().attribute("a"), Some("b"));
    /// assert_eq!(tree.root_element().attribute(("http://www.w3.org", "a")), Some("c"));
    /// ```
    pub fn attribute<N>(&self, name: N) -> Option<String>
    where
        N: Into<ExpandedName>,
    {
        self.attribute_node(name).map(|a| a.value())
    }

    /// Returns element's attribute object.
    ///
    /// The same as [`attribute()`], but returns the `Attribute` itself instead of a value string.
    ///
    /// [`attribute()`]: struct.Node.html#method.attribute
    pub fn attribute_node<N>(&self, name: N) -> Option<Attribute>
    where
        N: Into<ExpandedName>,
    {
        let name = name.into();

        match name.namespace() {
            Some(_) => self.attributes().find(|a| a.data.name.as_expanded_name(self.tree) == name),
            None => self.attributes().find(|a| a.data.name.local_name == name.name),
        }
    }

    /// Checks that element has a specified attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse(
    ///     "<e xmlns:n='http://www.w3.org' a='b' n:a='c'/>"
    /// ).unwrap();
    ///
    /// assert!(tree.root_element().has_attribute("a"));
    /// assert!(tree.root_element().has_attribute(("http://www.w3.org", "a")));
    ///
    /// assert!(!tree.root_element().has_attribute("b"));
    /// assert!(!tree.root_element().has_attribute(("http://www.w4.org", "a")));
    /// ```
    pub fn has_attribute<N>(&self, name: N) -> bool
    where
        N: Into<ExpandedName>,
    {
        self.attribute_node(name).is_some()
    }

    /// Returns element's attributes.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse(
    ///     "<e xmlns:n='http://www.w3.org' a='b' n:a='c'/>"
    /// ).unwrap();
    ///
    /// assert_eq!(tree.root_element().attributes().len(), 2);
    /// ```
    #[inline]
    pub fn attributes(&self) -> Attributes {
        Attributes::new(self)
    }

    /// Returns element's namespaces.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse(
    ///     "<e xmlns:n='http://www.w3.org'/>"
    /// ).unwrap();
    ///
    /// assert_eq!(tree.root_element().namespaces().len(), 1);
    /// ```
    #[inline]
    pub fn namespaces(&self) -> NamespaceIter {
        let namespaces = match self.d.kind {
            NodeKind::Element { ref namespaces, .. } => {
                &self.tree.namespaces.tree_order[namespaces.to_urange()]
            }
            _ => &[],
        };

        NamespaceIter {
            tree: self.tree,
            namespaces: namespaces.iter(),
        }
    }

    /// Returns node's text.
    ///
    /// - for an element will return a first text child
    /// - for a comment will return a self text
    /// - for a text node will return a self text
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse("\
    /// <p>
    ///     text
    /// </p>
    /// ").unwrap();
    ///
    /// assert_eq!(tree.root_element().text(),
    ///            Some("\n    text\n"));
    /// assert_eq!(tree.root_element().first_child().unwrap().text(),
    ///            Some("\n    text\n"));
    /// ```
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse("<!-- comment --><e/>").unwrap();
    ///
    /// assert_eq!(tree.root().first_child().unwrap().text(), Some(" comment "));
    /// ```
    #[inline]
    pub fn text(&self) -> Option<String> {
        self.text_storage().map(|s| s.as_str())
    }

    /// Returns node's text storage.
    ///
    /// Useful when you need a more low-level access to an allocated string.
    pub fn text_storage(&self) -> Option<String> {
        match self.d.kind {
            NodeKind::Element { .. } => match self.first_child() {
                Some(child) if child.is_text() => match self.tree.nodes[child.id.get_usize()].kind {
                    NodeKind::Text(ref text) => Some(text),
                    _ => None,
                },
                _ => None,
            },
            NodeKind::Comment(ref text) => Some(text),
            NodeKind::Text(ref text) => Some(text),
            _ => None,
        }
    }

    /// Returns element's tail text.
    ///
    /// # Examples
    ///
    /// ```
    /// let tree = roxmltree::Tree::parse("\
    /// <root>
    ///     text1
    ///     <p/>
    ///     text2
    /// </root>
    /// ").unwrap();
    ///
    /// let p = tree.descendants().find(|n| n.has_tag_name("p")).unwrap();
    /// assert_eq!(p.tail(), Some("\n    text2\n"));
    /// ```
    #[inline]
    pub fn tail(&self) -> Option<String> {
        self.tail_storage().map(|s| s.as_str())
    }

    /// Returns element's tail text storage.
    ///
    /// Useful when you need a more low-level access to an allocated string.
    pub fn tail_storage(&self) -> Option<String> {
        if !self.is_element() {
            return None;
        }

        match self.next_sibling().map(|n| n.id) {
            Some(id) => match self.tree.nodes[id.get_usize()].kind {
                NodeKind::Text(ref text) => Some(text),
                _ => None,
            },
            None => None,
        }
    }

    /// Returns node as Processing Instruction.
    #[inline]
    pub fn pi(&self) -> Option<PI> {
        match self.d.kind {
            NodeKind::PI(pi) => Some(pi),
            _ => None,
        }
    }

    /// Returns the parent of this node.
    #[inline]
    pub fn parent(&self) -> Option<Self> {
        self.d.parent.map(|id| self.tree.get_node(id).unwrap())
    }

    /// Returns the parent element of this node.
    pub fn parent_element(&self) -> Option<Self> {
        self.ancestors().skip(1).find(|n| n.is_element())
    }

    /// Returns the previous sibling of this node.
    #[inline]
    pub fn prev_sibling(&self) -> Option<Self> {
        self.d.prev_sibling.map(|id| self.tree.get_node(id).unwrap())
    }

    /// Returns the previous sibling element of this node.
    pub fn prev_sibling_element(&self) -> Option<Self> {
        self.prev_siblings().skip(1).find(|n| n.is_element())
    }

    /// Returns the next sibling of this node.
    #[inline]
    pub fn next_sibling(&self) -> Option<Self> {
        self.d
            .next_subtree
            .map(|id| self.tree.get_node(id).unwrap())
            .and_then(|node| {
                let possibly_self = node
                    .d
                    .prev_sibling
                    .expect("next_subtree will always have a previous sibling");
                if possibly_self == self.id {
                    Some(node)
                } else {
                    None
                }
            })
    }

    /// Returns the next sibling element of this node.
    pub fn next_sibling_element(&self) -> Option<Self> {
        self.next_siblings().skip(1).find(|n| n.is_element())
    }

    /// Returns the first child of this node.
    #[inline]
    pub fn first_child(&self) -> Option<Self> {
        self.d
            .last_child
            .map(|_| self.tree.get_node(NodeId::new(self.id.get() + 1)).unwrap())
    }

    /// Returns the first element child of this node.
    pub fn first_element_child(&self) -> Option<Self> {
        self.children().find(|n| n.is_element())
    }

    /// Returns the last child of this node.
    #[inline]
    pub fn last_child(&self) -> Option<Self> {
        self.d.last_child.map(|id| self.tree.get_node(id).unwrap())
    }

    /// Returns the last element child of this node.
    pub fn last_element_child(&self) -> Option<Self> {
        self.children().filter(|n| n.is_element()).next_back()
    }

    /// Returns true if this node has siblings.
    #[inline]
    pub fn has_siblings(&self) -> bool {
        self.d.prev_sibling.is_some() || self.next_sibling().is_some()
    }

    /// Returns true if this node has children.
    #[inline]
    pub fn has_children(&self) -> bool {
        self.d.last_child.is_some()
    }

    /// Returns an iterator over ancestor nodes starting at this node.
    #[inline]
    pub fn ancestors(&self) -> AxisIter {
        AxisIter {
            node: Some(*self),
            next: Node::parent,
        }
    }

    /// Returns an iterator over previous sibling nodes starting at this node.
    #[inline]
    pub fn prev_siblings(&self) -> AxisIter {
        AxisIter {
            node: Some(*self),
            next: Node::prev_sibling,
        }
    }

    /// Returns an iterator over next sibling nodes starting at this node.
    #[inline]
    pub fn next_siblings(&self) -> AxisIter {
        AxisIter {
            node: Some(*self),
            next: Node::next_sibling,
        }
    }

    /// Returns an iterator over first children nodes starting at this node.
    #[inline]
    pub fn first_children(&self) -> AxisIter {
        AxisIter {
            node: Some(*self),
            next: Node::first_child,
        }
    }

    /// Returns an iterator over last children nodes starting at this node.
    #[inline]
    pub fn last_children(&self) -> AxisIter {
        AxisIter {
            node: Some(*self),
            next: Node::last_child,
        }
    }

    /// Returns an iterator over children nodes.
    #[inline]
    pub fn children(&self) -> Children {
        Children {
            front: self.first_child(),
            back: self.last_child(),
        }
    }

    /// Returns an iterator over this node and its descendants.
    #[inline]
    pub fn descendants(&self) -> Descendants {
        Descendants::new(*self)
    }

    /// Returns node's range in bytes in the original tree.
    #[inline]
    pub fn range(&self) -> Range<usize> {
        self.d.range.clone()
    }

    /// Returns node's NodeId
    #[inline]
    pub fn id(&self) -> NodeId {
        self.id
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.d.kind {
            NodeKind::Root => write!(f, "Root"),
            NodeKind::Element { .. } => {
                write!(
                    f,
                    "Element {{ tag_name: {:?}, attributes: {:?}, namespaces: {:?} }}",
                    self.tag_name(),
                    self.attributes(),
                    self.namespaces()
                )
            }
            NodeKind::PI(pi) => {
                write!(f, "PI {{ target: {:?}, value: {:?} }}", pi.target, pi.value)
            }
            NodeKind::Comment(ref text) => write!(f, "Comment({:?})", text.as_str()),
            NodeKind::Text(ref text) => write!(f, "Text({:?})", text.as_str()),
        }
    }
}

/// Iterator over a node's attributes
#[derive(Clone)]
pub struct Attributes {
    tree: &Tree,
    attrs: core::slice::Iter<AttributeData>,
}

impl Attributes {
    #[inline]
    fn new(node: &Node) -> Attributes {
        let attrs = match node.d.kind {
            NodeKind::Element { ref attributes, .. } => {
                &node.tree.attributes[attributes.to_urange()]
            }
            _ => &[],
        };
        Attributes {
            tree: node.tree,
            attrs: attrs.iter(),
        }
    }
}

impl Iterator for Attributes {
    type Item = Attribute;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.attrs.next().map(|attr| Attribute {
            tree: self.tree,
            data: attr,
        })
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.attrs.nth(n).map(|attr| Attribute {
            tree: self.tree,
            data: attr,
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.attrs.size_hint()
    }
}

impl DoubleEndedIterator for Attributes {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.attrs.next_back().map(|attr| Attribute {
            tree: self.tree,
            data: attr,
        })
    }
}

impl ExactSizeIterator for Attributes {}

impl fmt::Debug for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Attributes")
            .field("attrs", &self.attrs)
            .finish()
    }
}

/// Iterator over specified axis.
#[derive(Clone)]
pub struct AxisIter {
    node: Option<Node>,
    next: fn(&Node) -> Option<Node>,
}

impl Iterator for AxisIter {
    type Item = Node;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node.take();
        self.node = node.as_ref().and_then(self.next);
        node
    }
}

impl fmt::Debug for AxisIter {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("AxisIter")
            .field("node", &self.node)
            .field("next", &"fn()")
            .finish()
    }
}

/// Iterator over children.
#[derive(Clone, Debug)]
pub struct Children {
    front: Option<Node>,
    back: Option<Node>,
}

impl Iterator for Children {
    type Item = Node;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            let node = self.front.take();
            self.back = None;
            node
        } else {
            let node = self.front.take();
            self.front = node.as_ref().and_then(Node::next_sibling);
            node
        }
    }
}

impl DoubleEndedIterator for Children {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back == self.front {
            let node = self.back.take();
            self.front = None;
            node
        } else {
            let node = self.back.take();
            self.back = node.as_ref().and_then(Node::prev_sibling);
            node
        }
    }
}

/// Iterator over a node and its descendants.
#[derive(Clone)]
pub struct Descendants {
    tree: Tree,
    nodes: core::iter::Enumerate<core::slice::Iter<NodeData>>,
    from: usize,
}

impl Descendants {
    #[inline]
    fn new(start: Node) -> Self {
        let from = start.id.get_usize();

        let until = start
            .d
            .next_subtree
            .map(NodeId::get_usize)
            .unwrap_or(start.tree.nodes.len());

        let nodes = start.tree.nodes[from..until].iter().enumerate();

        Self {
            tree: start.tree,
            nodes,
            from,
        }
    }
}

impl Iterator for Descendants {
    type Item = Node;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.next().map(|(idx, data)| Node {
            id: NodeId::from(self.from + idx),
            d: data,
            tree: self.tree,
        })
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.nodes.nth(n).map(|(idx, data)| Node {
            id: NodeId::from(self.from + idx),
            d: data,
            tree: self.tree,
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.nodes.size_hint()
    }
}

impl DoubleEndedIterator for Descendants {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.nodes.next_back().map(|(idx, data)| Node {
            id: NodeId::from(self.from + idx),
            d: data,
            tree: self.tree,
        })
    }
}

impl ExactSizeIterator for Descendants {}

impl fmt::Debug for Descendants {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Descendants")
            .field("nodes", &self.nodes)
            .field("from", &self.from)
            .finish()
    }
}

/// Iterator over the namespaces attached to a node.
#[derive(Clone)]
pub struct NamespaceIter {
    tree: Tree,
    namespaces: core::slice::Iter<NamespaceIdx>,
}

impl Iterator for NamespaceIter {
    type Item = Namespace;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.namespaces
            .next()
            .map(|idx| self.tree.namespaces.get(*idx))
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.namespaces
            .nth(n)
            .map(|idx| self.tree.namespaces.get(*idx))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.namespaces.size_hint()
    }
}

impl DoubleEndedIterator for NamespaceIter {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.namespaces
            .next()
            .map(|idx| self.tree.namespaces.get(*idx))
    }
}

impl ExactSizeIterator for NamespaceIter {}

impl fmt::Debug for NamespaceIter {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("NamespaceIter")
            .field("namespaces", &self.namespaces)
            .finish()
    }
}
/// Position in text.
///
/// Position indicates a row/line and a column in the original text. Starting from 1:1.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TextPos {
    pub row: u32,
    pub col: u32,
}

impl TextPos {
    /// Constructs a new `TextPos`.
    pub fn new(row: u32, col: u32) -> TextPos {
        TextPos { row, col }
    }
}

impl fmt::Display for TextPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.row, self.col)
    }
}
