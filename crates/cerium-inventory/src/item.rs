use std::{
    any::{Any, TypeId},
    marker::PhantomData,
    sync::Arc,
};

use cerium_registry::material::Material;
use rustc_hash::{FxBuildHasher, FxHashMap};
use simdnbt::owned::Nbt;

use crate::Slot;

#[derive(Debug, Clone)]
pub struct ItemStack {
    material: Material,
    amount: i32,
    components: FxHashMap<i32, Arc<dyn Any + Send + Sync>>,
}

impl ItemStack {
    pub const EMPTY: ItemStack = ItemStack::new(Material::Air, 0);

    pub const fn new(material: Material, amount: i32) -> Self {
        Self {
            material,
            amount,
            components: FxHashMap::with_hasher(FxBuildHasher),
        }
    }

    pub const fn of(material: Material) -> Self {
        Self {
            material,
            amount: 1,
            components: FxHashMap::with_hasher(FxBuildHasher),
        }
    }

    pub fn with_material(self, material: Material) -> Self {
        Self {
            material,
            amount: self.amount,
            components: self.components,
        }
    }

    pub fn with_amount(self, amount: i32) -> Self {
        Self {
            material: self.material,
            amount,
            components: self.components,
        }
    }

    pub fn with<T: 'static + Sync + Send>(self, component: DataComponent<T>, value: T) -> Self {
        let Self {
            material,
            amount,
            mut components,
        } = self;
        components.insert(component.id(), Arc::new(value));

        Self {
            material,
            amount,
            components,
        }
    }

    pub fn get<T: 'static>(&self, component: DataComponent<T>) -> Option<&T> {
        self.components
            .get(&component.id())
            .and_then(|v| v.downcast_ref::<T>())
    }

    pub fn set<T: Send + Sync>(mut self, component: DataComponent<T>, value: &'static T) {
        self.components.insert(component.id(), Arc::new(value));
    }

    // Getters

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn amount(&self) -> i32 {
        self.amount
    }
}

impl From<Slot> for ItemStack {
    fn from(value: Slot) -> Self {
        if let Some(item_id) = value.item_id {
            Self {
                material: Material::from_id(item_id).unwrap(),
                amount: value.item_count,
                components: value.to_add,
            }
        } else {
            ItemStack::EMPTY
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_components() {
        let stack = ItemStack::of(Material::Stone)
            .with(DataComponent::MAX_STACK_SIZE, 16)
            .with(DataComponent::MAX_DAMAGE, 99)
            .with(DataComponent::UNBREAKABLE, ());

        assert_eq!(stack.get(DataComponent::MAX_STACK_SIZE), Some(&16));
        assert_eq!(stack.get(DataComponent::MAX_DAMAGE), Some(&99));
        assert_eq!(stack.get(DataComponent::UNBREAKABLE), Some(&()));
        assert_eq!(stack.get(DataComponent::DAMAGE), None);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnyDataComponent {
    id: i32,
    key: &'static str,
    type_id: TypeId,
}

impl AnyDataComponent {
    pub fn downcast<T: 'static>(self) -> Result<DataComponent<T>, AnyDataComponent> {
        if TypeId::of::<T>() == self.type_id {
            Ok(DataComponent {
                any: self,
                __phantom: PhantomData,
            })
        } else {
            Err(self)
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn key(&self) -> &'static str {
        self.key
    }

    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    pub fn from_id(id: i32) -> Option<AnyDataComponent> {
        match id {
            1 => Some(DataComponent::MAX_STACK_SIZE.into_any()),
            2 => Some(DataComponent::MAX_DAMAGE.into_any()),
            3 => Some(DataComponent::DAMAGE.into_any()),
            4 => Some(DataComponent::UNBREAKABLE.into_any()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataComponent<T> {
    any: AnyDataComponent,
    __phantom: PhantomData<T>,
}

impl<T> DataComponent<T> {
    const fn new(id: i32, key: &'static str) -> Self
    where
        T: 'static,
    {
        Self {
            any: AnyDataComponent {
                id,
                key,
                type_id: TypeId::of::<T>(),
            },
            __phantom: PhantomData,
        }
    }

    pub fn id(&self) -> i32 {
        self.any.id()
    }

    pub fn key(&self) -> &'static str {
        self.any.key()
    }

    pub fn into_any(self) -> AnyDataComponent {
        self.any
    }
}

macro_rules! define {
    ($id:expr, $key:expr) => {
        DataComponent::new($id, $key)
    };
}

impl DataComponent<Nbt> {
    pub const CUSTOM_DATA: DataComponent<Nbt> = define!(0, "minecraft:custom_data");
}

impl DataComponent<i32> {
    pub const MAX_STACK_SIZE: DataComponent<i32> = define!(1, "minecraft:max_stack_size");
    pub const MAX_DAMAGE: DataComponent<i32> = define!(2, "minecraft:max_damage");
    pub const DAMAGE: DataComponent<i32> = define!(3, "minecraft:damage");
}

impl DataComponent<()> {
    pub const UNBREAKABLE: DataComponent<()> = define!(4, "minecraft:unbreakable");
}
