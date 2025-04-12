// hashmaps2.rs
//
// We're collecting different fruits to bake a delicious fruit cake. For this,
// we have a basket, which we'll represent in the form of a hash map. The key
// represents the name of each fruit we collect and the value represents how
// many of that particular fruit we have collected. Three types of fruits -
// Apple (4), Mango (2) and Lychee (5) are already in the basket hash map. You
// must add fruit to the basket so that there is at least one of each kind and
// more than 11 in total - we have a lot of mouths to feed. You are not allowed
// to insert any more of these fruits!
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps2` or use the `hint` watch subcommand for a
// hint.



use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = vec![
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    
    for fruit in &fruit_kinds {
        if !basket.contains_key(fruit) {
            basket.insert(fruit.clone(), 1); 
        }
    }

    let current_count: u32 = basket.values().sum(); 
    if current_count <= 11 {
        let additional_fruits_needed = 12 - current_count;
        let mut additional_fruits_added = 0;


        for fruit in &fruit_kinds {
            if basket.contains_key(fruit) {
                continue;
            }

            basket.insert(fruit.clone(), 1);
            additional_fruits_added += 1;

            if additional_fruits_added >= additional_fruits_needed {
                break;
            }
        }
    }
}
