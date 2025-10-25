use std::{fmt::Debug, sync::Arc};

use rustc_hash::{FxBuildHasher, FxHashMap};

use crate::{
    inventory::Slot,
    item::{ComponentMap, DataComponent, Material},
};

#[derive(Debug, Clone)]
pub struct ItemStack {
    material: Material,
    amount: i32,
    components: ComponentMap,
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

    pub fn with<T: 'static + Sync + Send>(mut self, component: DataComponent<T>, value: T) -> Self {
        self.components.insert(component.id(), Arc::new(value));
        self
    }

    pub fn get<T: 'static>(&self, component: DataComponent<T>) -> Option<&T> {
        self.components
            .get(&component.id())
            .and_then(|v| v.downcast_ref::<T>())
    }

    pub fn set<T: Send + Sync>(mut self, component: DataComponent<T>, value: &'static T) {
        self.components.insert(component.id(), Arc::new(value));
    }

    pub fn has<T: 'static>(&self, component: DataComponent<T>) -> bool {
        self.get(component).is_some()
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn amount(&self) -> i32 {
        self.amount
    }
}

impl From<Material> for ItemStack {
    fn from(value: Material) -> Self {
        Self::new(value, 1)
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

impl Into<Slot> for ItemStack {
    fn into(self) -> Slot {
        Slot {
            item_count: self.amount,
            item_id: Some(self.material as i32),
            to_add: self.components,
            to_remove: vec![],
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
