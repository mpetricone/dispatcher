use iced::widget::{ button, Column, scrollable , text};
use iced::Element;
use std::cmp::PartialEq;
use std::fmt::Display;
use std::clone::Clone;

pub struct ItemList<'a, T : Display + PartialEq + Clone> {
    items: Vec<T>,
    current_item: Option<&'a T>,
}

#[derive(Debug, Clone, Copy)]
pub enum ListMessage<T> {
    ItemSelected(T),
    Add,
    Edit(usize),
    Remove(usize),
    Close,
}

impl<'a, T: Display + PartialEq + Clone + 'a> ItemList<'_, T> {
    pub fn new(items: Vec<T>) -> ItemList<'a, T> {
        ItemList {
            items,
            current_item: None,
        }
    }

    pub fn view(self) -> iced::Element<'a, ListMessage<T>> {
        let mut button_list: Vec<Element<ListMessage<T>>> = Vec::new();
        for i in self.items {
            button_list.push(button(text(i.to_string())).on_press(ListMessage::ItemSelected(i)).into());
        }
        scrollable(Column::from_vec(button_list)).into()
    }
}
