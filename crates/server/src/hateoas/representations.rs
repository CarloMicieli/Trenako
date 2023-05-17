use crate::hateoas::links::{Link, LinkRelation};
use serde::Serialize;

/// A trait for representation models to collect links.
pub trait RepresentationModel {
    /// Adds the given link to the resource.
    fn add_link(&mut self, link: Link);

    /// Adds all given Links to the resource.
    fn add_links(&mut self, links: &mut Vec<Link>);

    /// Returns all Links contained in this resource.
    fn get_links(&self) -> &[Link];

    /// Returns whether the resource contains a Link with the given rel.
    fn has_link(&self, rel: LinkRelation) -> bool {
        self.get_links().iter().any(|it| it.rel == rel)
    }

    /// Returns whether the resource contains Links at all.
    fn has_links(&self) -> bool {
        !self.get_links().is_empty()
    }
}

/// A simple EntityModel wrapping a domain object and adding links to it.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct EntityModel<T>
where
    T: Serialize + PartialEq + Clone,
{
    #[serde(flatten)]
    pub content: T,

    #[serde(rename = "_links")]
    pub links: Vec<Link>,
}

impl<T> EntityModel<T>
where
    T: Serialize + PartialEq + Clone,
{
    /// Creates a new EntityModel for the given content and links.
    pub fn of(content: T, links: Vec<Link>) -> Self {
        EntityModel { content, links }
    }
}

impl<T> RepresentationModel for EntityModel<T>
where
    T: Serialize + PartialEq + Clone,
{
    fn add_link(&mut self, link: Link) {
        self.links.push(link);
    }

    fn add_links(&mut self, links: &mut Vec<Link>) {
        self.links.append(links);
    }

    fn get_links(&self) -> &[Link] {
        &self.links
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct CollectionModel<T>
where
    T: Serialize + PartialEq + Clone,
{
    pub items: Vec<T>,

    #[serde(rename = "_links")]
    pub links: Vec<Link>,
}

impl<T> RepresentationModel for CollectionModel<T>
where
    T: Serialize + PartialEq + Clone,
{
    fn add_link(&mut self, link: Link) {
        self.links.push(link);
    }

    fn add_links(&mut self, links: &mut Vec<Link>) {
        self.links.append(links);
    }

    fn get_links(&self) -> &[Link] {
        &self.links
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod entity_model_tests {
        use super::*;

        #[test]
        fn it_should_serialize_entity_models_without_links() {
            let content = TestStruct {
                label: "label".to_string(),
                value: 42,
            };
            let entity_model = EntityModel::of(content, Vec::new());

            let result = serde_json::json!(entity_model);
            let json = result.to_string();

            assert_eq!(r#"{"_links":[],"label":"label","value":42}"#, json);
        }

        #[test]
        fn it_should_serialize_entity_models_with_links() {
            let content = TestStruct {
                label: "label".to_string(),
                value: 42,
            };

            let link = Link::of("http://localhost", LinkRelation::SelfLink).expect("the self link is invalid");
            let mut entity_model = EntityModel::of(content, Vec::new());
            entity_model.add_link(link);

            let result = serde_json::json!(entity_model);
            let json = result.to_string();

            assert_eq!(
                r#"{"_links":[{"href":"http://localhost/","rel":"self"}],"label":"label","value":42}"#,
                json
            );
        }

        #[test]
        fn it_should_add_links_to_entity_models() {
            let mut entity_model = new_entity_model(Vec::new());

            let link = Link::of("http://localhost", LinkRelation::SelfLink).expect("the self link is invalid");
            entity_model.add_link(link);

            let links = entity_model.get_links();

            assert_eq!(1, links.len());
            assert!(entity_model.has_links());
            assert!(entity_model.has_link(LinkRelation::SelfLink));
            assert!(!entity_model.has_link(LinkRelation::Item));
        }

        fn new_entity_model(links: Vec<Link>) -> EntityModel<TestStruct> {
            let content = TestStruct {
                label: "label".to_string(),
                value: 42,
            };
            EntityModel::of(content, links)
        }

        #[derive(Debug, Serialize, Clone, PartialEq)]
        pub struct TestStruct {
            pub label: String,
            pub value: i32,
        }
    }
}
