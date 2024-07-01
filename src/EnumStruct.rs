// Problem 1: 
/* You are tasked with implementing a library management system using Rust. 
Your goal is to design a program that can handle books and magazines. 
To fulfill the requirements, follow the steps below:

Create a structure called Item with the following fields:
id: An integer representing the unique identifier of the item.
title: A string representing the title of the item.
year: An integer representing the publication year of the item.
type: an enumeration type. The details are coming below.

Create an enumeration called ItemType with two variants:
Book: Represents a book.
Magazine: Represents a magazine.

Implement a function called display_item_info() that takes an Item as input 
and displays its information. The function should output 
the item's ID, title, publication year, and type (book or magazine). 
*/ 


/* Expected solution

#[derive(Debug)]
struct Item {
    id: i32,
    title: String,
    year: i32,
    type_: ItemType,
}

#[derive(Debug)]
enum ItemType {
    Book,
    Magazine,
}

fn display_item_info(item: &Item) {
    println!("Item ID: {:?}", item.id);
    println!("Title: {:?}", item.title);
    println!("Publication Year: {:?}", item.year);
    println!("Publication Type: {:?}", item.type_);
}

*/

// This works though

struct Item {
    id: i32,
    title: String,
    year: i32,
    item_type: ItemType,
}

enum ItemType {
    Book,
    Magazine
}

fn display_item_info(item: Item) {
    let it = match item.item_type {
        ItemType::Book => "Book".to_string(),
        ItemType::Magazine => "Magazine".to_string()
    };
    print!("
    ID:  {}
    Title:  {}
    Publication Year:  {}
    Publication Type:  {}", 
    item.id, 
    item.title, 
    item.year, 
    it);
}

fn main(){
    let item = Item {
        id: 12345,
        title: "Pepsi".to_string(),
        year: 2024,
        item_type: ItemType::Book
    };
    display_item_info(item)
}