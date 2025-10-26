use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use std::sync::{
    Arc,
    atomic::{AtomicI32, Ordering},
};

use crate::{
    entity::Player,
    inventory::{InventoryType, Slot},
    item::{ItemStack, Material},
    protocol::packet::{
        OpenScreenPacket, SetContainerContentPacket, SetContainerSlotPacket,
        server::CloseContainerPacket,
    },
    text::Component,
    util::{Viewable, Viewers},
};

#[derive(Clone)]
pub struct Inventory(Arc<Inner>);

impl Inventory {
    pub fn new(ty: InventoryType, title: impl Into<Component>) -> Self {
        Self(Arc::new(Inner::new(ty, title)))
    }

    /// Returns the id of the inventory.
    pub fn id(&self) -> i32 {
        self.0.id()
    }

    /// Returns the type of the inventory.
    pub fn r#type(&self) -> InventoryType {
        self.0.r#type()
    }

    /// Returns the title of the inventory.
    pub fn title(&self) -> &Component {
        self.0.title()
    }

    /// Returns the size of the inventory.
    pub fn size(&self) -> i32 {
        self.0.size()
    }

    /// Adds an [`ItemStack`] to the first available slot in the inventory.
    pub fn add_item_stack(&self, stack: ItemStack) {
        self.0.add_item_stack(stack)
    }

    /// Inserts an [`ItemStack`] into a given slot and overwrites the previous data.
    pub fn set_item_stack(&self, slot: i32, stack: ItemStack) {
        self.0.set_item_stack(slot, stack)
    }

    /// Returns the [`ItemStack`] in the current slot.
    pub fn get_item_stack(&self, slot: i32) -> ItemStack {
        self.0.get_item_stack(slot)
    }
}

impl Viewable for Inventory {
    fn add_viewer(&self, player: Player) {
        self.0.add_viewer(player);
    }

    fn remove_viewer(&self, player: Player) {
        self.0.remove_viewer(player);
    }

    fn viewers(&self) -> &Viewers {
        self.0.viewers()
    }
}

struct Inner {
    id: i32,
    ty: InventoryType,
    title: Component,
    content: Mutex<FxHashMap<i32, ItemStack>>,
    viewers: Viewers,
}

impl Inner {
    fn new(ty: InventoryType, title: impl Into<Component>) -> Self {
        let size = ty.size();
        let mut content = FxHashMap::with_capacity_and_hasher(size as usize, Default::default());
        for ix in 0..size {
            content.insert(ix, ItemStack::EMPTY);
        }

        Self {
            id: Self::generate_id(),
            ty,
            title: title.into(),
            content: Mutex::new(content),
            viewers: Viewers::new(),
        }
    }

    fn generate_id() -> i32 {
        static CURRENT_ID: AtomicI32 = AtomicI32::new(1);
        CURRENT_ID
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |i| {
                Some(if i + 1 >= 128 { 1 } else { i + 1 })
            })
            .unwrap()
    }

    fn id(&self) -> i32 {
        self.id
    }

    fn r#type(&self) -> InventoryType {
        self.ty
    }

    fn title(&self) -> &Component {
        &self.title
    }

    fn size(&self) -> i32 {
        self.ty.size()
    }

    fn add_item_stack(&self, stack: ItemStack) {
        let mut content = self.content.lock();
        for (ix, stck) in content.values().enumerate(){
            if stck.material() == Material::Air {
                content.insert(ix as i32, stack.clone());

                self.send_packet_to_viewers(SetContainerSlotPacket {
                    window_id: self.id(),
                    state_id: 0,
                    slot: ix as i16,
                    slot_data: stack.into(),
                });
                break;
            }
        }
    }

    fn set_item_stack(&self, slot: i32, stack: ItemStack) {
        self.content.lock().insert(slot, stack.clone());

        self.send_packet_to_viewers(SetContainerSlotPacket {
            window_id: self.id(),
            state_id: 0,
            slot: slot as i16,
            slot_data: stack.into(),
        });
    }

    /// Returns the [`ItemStack`] in the current slot.
    fn get_item_stack(&self, slot: i32) -> ItemStack {
        self.content
            .lock()
            .get(&slot)
            .cloned()
            .unwrap_or(ItemStack::EMPTY)
    }

    fn refresh_contents(&self, player: Player) {
        let content = self.content.lock().clone();
        println!(
            "{:?}",
            content
                .clone()
                .into_iter()
                .map(|(_, s)| s.into())
                .collect::<Vec<Slot>>()
        );
        player.send_packet(SetContainerContentPacket {
            window_id: self.id(),
            state_id: 0,
            slot_data: content.into_iter().map(|(_, s)| s.into()).collect(),
            carried_item: ItemStack::EMPTY.into(),
        });
    }
}

impl Viewable for Inner {
    fn add_viewer(&self, player: Player) {
        self.viewers.add_viewer(player.clone());

        player.send_packet(OpenScreenPacket {
            window_id: self.id(),
            window_type: self.r#type().id(),
            window_title: self.title().clone(),
        });
        self.refresh_contents(player);
    }

    fn remove_viewer(&self, player: Player) {
        self.viewers.remove_viewer(player.clone());

        player.send_packet(CloseContainerPacket {
            window_id: self.id(),
        });
    }

    fn viewers(&self) -> &Viewers {
        &self.viewers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn test_set_item_stack() {
        let inventory = Inventory::new(InventoryType::Generic9x6, "");
        inventory.set_item_stack(1, ItemStack::EMPTY);
        inventory.set_item_stack(22, ItemStack::new(Material::AcaciaBoat, 1));

        assert_eq!(inventory.get_item_stack(0).material(), Material::Air);
        assert_eq!(inventory.get_item_stack(1).material(), Material::Air);
        assert_eq!(inventory.get_item_stack(22).material(), Material::AcaciaBoat);
    }

    #[test]
    #[rustfmt::skip]
    fn test_add_item_stack() {
        let inventory = Inner::new(InventoryType::Generic9x6, "");
        inventory.add_item_stack(ItemStack::EMPTY);
        inventory.add_item_stack(ItemStack::new(Material::GraniteStairs, 1));
        inventory.add_item_stack(ItemStack::new(Material::RedCandle, 1));

        assert_eq!(inventory.get_item_stack(0).material(), Material::GraniteStairs);
        assert_eq!(inventory.get_item_stack(1).material(), Material::RedCandle);
    }
}
