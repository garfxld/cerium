use std::sync::Mutex;

use crate::inventory::item::ItemStack;

#[derive(Debug)]
pub struct PlayerInventory {
    size: i32,
    content: Mutex<Vec<ItemStack>>,
}

impl PlayerInventory {
    pub fn new() -> Self {
        const SIZE: i32 = 54;
        let mut content = Vec::with_capacity(SIZE as usize);
        for _ in 0..SIZE {
            content.push(ItemStack::EMPTY);
        }

        Self {
            size: SIZE,
            content: Mutex::new(content),
        }
    }

    pub fn size(&self) -> i32 {
        self.size
    }

    pub fn set_item_stack(&self, slot: i32, item: ItemStack) {
        self.content.lock().unwrap().insert(slot as usize, item);
    }

    pub fn get_item_stack(&self, slot: i32) -> Option<ItemStack> {
        self.content.lock().unwrap().get(slot as usize).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::Material;

    #[test]
    pub fn test_player_inventory() {
        let inventory = PlayerInventory::new();
        inventory.set_item_stack(9, ItemStack::of(Material::AcaciaBoat));

        assert_eq!(
            inventory.get_item_stack(9).map(|v| v.material()),
            Some(Material::AcaciaBoat)
        );
        assert_eq!(
            inventory.get_item_stack(10).map(|v| v.material()),
            Some(Material::Air)
        );
    }
}
