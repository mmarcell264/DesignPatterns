use super::Component;

pub struct Folder<'a> {
    name: &'static str,
    components: Vec<Box<dyn Component + 'a >>,
}

impl<'a> Folder<'a> {
    pub fn new(name: &'static str) -> Self {
        Self { name, components: vec![] }
    }

    pub fn add(&mut self, component: impl Component + 'a) {
        self.components.push(Box::new(component));
    }

}

impl Component for Folder<'_>  {
    fn search(&self, keyword: &str) {
        println!(
            "Searching recursively for keyword {} in folder {}",
            keyword, self.name
        );

        for component in self.components.iter() {
            component.search(keyword);
        }
    }
}
