use SpellRange::*;
use SpellComponent::*;

#[derive(Debug)]
pub struct Spell {
    pub name: String,
    pub desc: String,
    pub school: String,
    pub range: SpellRange,
    pub cast_time: u32,
    pub duration: u32,
    pub components: Option<Vec<SpellComponent>>,
    pub prepared: bool,
}

impl Spell {
    pub fn cast(&mut self) -> Result<(), &'static str> {
        println!("abra cadabra!");

        Ok(())
    }

    pub fn new() -> Self {
        Spell {
            name: String::from(""),
            desc: String::from(""),
            school:  String::from(""),
            range: Oneself(),
            cast_time: 0,
            duration: 0,
            components: None,
            prepared: false,
        }
    }

    pub fn prepare(&mut self) {
        self.prepared = true;
    }

    pub fn unprepare(&mut self) {
        self.prepared = false;
    }
}

#[derive(Debug)]
enum SpellRange {
    Oneself(),
    Touch(),
    Distance(u32),
}

#[derive(Debug)]
enum SpellComponent {
    Verbal(),
    Somatic(),
    Material(Vec<String>),
}
