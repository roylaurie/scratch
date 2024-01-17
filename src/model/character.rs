pub use crate::{s, model::{error::*, identity::*, builder::*, descriptor::*, entity::*, something::*, thing::*}};

#[derive(Debug)]
pub struct Character {
    entity: Entity
}

pub struct CharacterBuilder {
    builder_mode: BuilderMode,
    id: Option<ID>,
    entity: Option<EntityBuilder>
}

impl Builder for CharacterBuilder {
    type Type = Character;

    fn creator() -> Self {
        Self {
            builder_mode: BuilderMode::Creator,
            id: None,
            entity: None
        }
    }

    fn editor() -> Self {
        Self {
            builder_mode: BuilderMode::Editor,
            ..Self::creator()
        }
    }

    fn builder_mode(&self) -> BuilderMode {
        self.builder_mode
    }

    fn create(self) -> Result<Character> {
        Ok(Character {
            entity: self.entity
                .ok_or_else(|| Error::FieldNotSet{class: "Entity", field: "entity"})?
                .create()?
        })
    }

    fn modify(self, original: &mut Self::Type) -> Result<ModifyResult> {
        todo!()
    }
}

impl Character {
    pub fn builder() -> CharacterBuilder {
        CharacterBuilder::creator()
    }
}

impl Descriptive for Character {
    fn descriptor(&self) -> &Descriptor{
        self.entity().descriptor()
    }
}

impl DescriptiveMut for Character {
    fn descriptor_mut(&mut self) -> &mut Descriptor {
        self.entity_mut().descriptor_mut()
    }
}

impl Something for Character {
    fn entity(&self) -> &Entity {
        &self.entity
    }

    fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }
}

impl ThingBuilder for CharacterBuilder {
    fn id(&mut self, id: ID) -> Result<()> {
        self.id = Some(id);
        Ok(())
    }

    fn entity(&mut self, entity: EntityBuilder) -> Result<()> {
        self.entity = Some(entity);
        Ok(())
    }

    fn build_thing(self) -> Result<Thing> {
        Ok(Thing::Character(self.create()?))
    }
}

