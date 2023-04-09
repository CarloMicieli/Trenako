//! This module provides the functionality to aggregate results when they include a relationship

use crate::queries::converters::{ConversionErrors, ToOutputConverter};
use std::hash::Hash;

/// An aggregate has the main purpose to merge results when they are in a sort of relationship.  
pub trait Aggregate {
    /// the unique identifier type
    type Id: Eq + Hash + Clone;

    /// the root row type
    type RootRowType: WithId<Self::Id> + ToOutputConverter<Self::RootOutput>;
    /// the children row type
    type ChildRowType: WithRootId<Self::Id> + ToOutputConverter<Self::ChildOutput>;

    /// the root output type
    type RootOutput: AggregateRoot<Self::Id, Self::ChildOutput>;
    /// the children output type
    type ChildOutput;

    /// Initialize an aggregate with a root and zero, one or more children
    fn init(
        root_row: Option<Self::RootRowType>,
        children_rows: Vec<Self::ChildRowType>,
    ) -> Result<Option<Self::RootOutput>, ConversionErrors> {
        if let Some(root_row) = root_row {
            let mut root_output = root_row.to_output()?;

            let children_len = children_rows.len();
            if children_len > 0 {
                let mut children_output: Vec<Self::ChildOutput> = Vec::with_capacity(children_len);
                for child in children_rows.into_iter() {
                    if child.root_id() == root_output.id() {
                        let c = child.to_output()?;
                        children_output.push(c);
                    }
                }

                root_output.add_children(children_output);
            }

            Ok(Some(root_output))
        } else {
            Ok(None)
        }
    }
}

/// A trait for types with an identifier
pub trait AggregateRoot<Id, T> {
    /// Return the root identifier
    fn id(&self) -> &Id;

    /// Add a new child to the current root
    fn add_child(&mut self, child: T);

    /// Add children to the current root
    fn add_children(&mut self, children: Vec<T>);
}

/// A trait for types with the reference id for their parent type
pub trait WithId<Id> {
    /// Returns the root identifier
    fn id(&self) -> &Id;
}

/// A trait for types with the reference id for their parent type
pub trait WithRootId<Id> {
    /// Returns the root identifier
    fn root_id(&self) -> &Id;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_return_a_conversion_error_when_it_is_not_possible_to_convert_the_root() {
        let root = Some(RootRow {
            id: 0,
            label: String::from("test"),
        });

        let result = TestAggregate::init(root, Vec::new());

        assert!(result.is_err());
    }

    #[test]
    fn it_should_return_a_conversion_error_when_it_is_not_possible_to_convert_the_child() {
        let root = Some(RootRow {
            id: 42,
            label: String::from("test"),
        });
        let child = ChildRow {
            id: 0,
            root_id: 42,
            label: String::from("child 1"),
        };
        let result = TestAggregate::init(root, vec![child]);

        assert!(result.is_err());
    }

    #[test]
    fn it_should_return_a_none_when_there_is_no_root_row() {
        let result = TestAggregate::init(None, Vec::new());

        assert!(result.is_ok());

        let root = result.unwrap();
        assert!(root.is_none());
    }

    #[test]
    fn it_should_join_one_root_row_with_no_children_row() {
        let root = Some(RootRow {
            id: 42,
            label: String::from("test"),
        });

        let result = TestAggregate::init(root, Vec::new());

        assert!(result.is_ok());

        let root = result.unwrap().expect("root is empty");
        assert_eq!(root.id, 42);
        assert_eq!(root.label, "test");
        assert_eq!(root.children.len(), 0);
    }

    #[test]
    fn it_should_join_one_root_row_with_children_rows() {
        let root = Some(RootRow {
            id: 42,
            label: String::from("test"),
        });

        let child1 = ChildRow {
            id: 1,
            root_id: 42,
            label: String::from("child 1"),
        };
        let child2 = ChildRow {
            id: 2,
            root_id: 42,
            label: String::from("child 2"),
        };

        let result = TestAggregate::init(root, vec![child1, child2]);

        assert!(result.is_ok());

        let root = result.unwrap().expect("root is empty");
        assert_eq!(root.id, 42);
        assert_eq!(root.label, "test");
        assert_eq!(root.children.len(), 2);
        assert_eq!(root.children[0].id, 1);
        assert_eq!(root.children[0].label, "child 1");
        assert_eq!(root.children[1].id, 2);
        assert_eq!(root.children[1].label, "child 2");
    }

    #[test]
    fn it_should_join_one_root_row_with_children_rows_with_the_same_root_it() {
        let root = Some(RootRow {
            id: 42,
            label: String::from("test"),
        });

        let child1 = ChildRow {
            id: 1,
            root_id: 42,
            label: String::from("child 1"),
        };
        let child2 = ChildRow {
            id: 2,
            root_id: 42,
            label: String::from("child 2"),
        };
        let child3 = ChildRow {
            id: 3,
            root_id: 41,
            label: String::from("child 3"),
        };

        let result = TestAggregate::init(root, vec![child1, child2, child3]);

        assert!(result.is_ok());

        let root = result.unwrap().expect("root is empty");
        assert_eq!(root.id, 42);
        assert_eq!(root.label, "test");
        assert_eq!(root.children.len(), 2);
        assert_eq!(root.children[0].id, 1);
        assert_eq!(root.children[0].label, "child 1");
        assert_eq!(root.children[1].id, 2);
        assert_eq!(root.children[1].label, "child 2");
    }

    pub struct TestAggregate;

    impl Aggregate for TestAggregate {
        type Id = u32;
        type RootRowType = RootRow;
        type ChildRowType = ChildRow;
        type RootOutput = Root;
        type ChildOutput = Child;
    }

    #[derive(Debug)]
    pub struct RootRow {
        pub id: u32,
        pub label: String,
    }

    impl WithId<u32> for RootRow {
        fn id(&self) -> &u32 {
            &self.id
        }
    }

    impl ToOutputConverter<Root> for RootRow {
        fn to_output(self) -> Result<Root, ConversionErrors> {
            if self.id == 0 {
                Err(ConversionErrors::new())
            } else {
                Ok(Root {
                    id: self.id,
                    label: self.label,
                    children: Vec::new(),
                })
            }
        }
    }

    #[derive(Debug)]
    pub struct Root {
        pub id: u32,
        pub label: String,
        pub children: Vec<Child>,
    }

    impl AggregateRoot<u32, Child> for Root {
        fn id(&self) -> &u32 {
            &self.id
        }

        fn add_child(&mut self, child: Child) {
            self.children.push(child);
        }

        fn add_children(&mut self, children: Vec<Child>) {
            self.children = children;
        }
    }

    #[derive(Debug)]
    pub struct ChildRow {
        pub id: u32,
        pub root_id: u32,
        pub label: String,
    }

    impl WithRootId<u32> for ChildRow {
        fn root_id(&self) -> &u32 {
            &self.root_id
        }
    }

    impl ToOutputConverter<Child> for ChildRow {
        fn to_output(self) -> Result<Child, ConversionErrors> {
            if self.id == 0 {
                Err(ConversionErrors::new())
            } else {
                Ok(Child {
                    id: self.id,
                    root_id: self.root_id,
                    label: self.label,
                })
            }
        }
    }

    #[derive(Debug)]
    pub struct Child {
        pub id: u32,
        pub root_id: u32,
        pub label: String,
    }
}
