use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
    str::FromStr,
};
use tabled::{Table, Tabled};
use uom::si::{
    f32::Mass,
    mass::{self, gram},
};
// use eyre::Result;
use float_cmp::approx_eq;
use thiserror::Error;

// #[derive(Deserialize, Serialize, Clone, Debug)]
// enum Quantity {
//     Mass(Mass),
//     Pieces(u32),
//     Volume,
// }

// #[derive(Error, Debug)]
// pub enum QuantityError {
//     #[error("Mismatched type of ingredient")]
//     MimatchedType,
// }

// impl Quantity {
//     fn add(&mut self, rhs: Self) -> Result<(), QuantityError> {
//         if std::mem::discriminant(self) != std::mem::discriminant(&rhs) {
//             return Err(QuantityError::MimatchedType);
//         }
//         if let Quantity::Mass(self_mass) = self {
//             if let Quantity::Mass(rhs_mass) = rhs {
//                 *self_mass += rhs_mass;
//             }
//         }
//         if let Quantity::Pieces(self_pieces) = self {
//             if let Quantity::Pieces(rhs_pieces) = rhs {
//                 *self_pieces += rhs_pieces;
//             }
//         }
//         Ok(())
//     }

//     fn get_buy_list(&self, avail_list: Vec<Self>) -> Vec<(Self, u32)>{
//         if avail_list.iter().max() < self
//     }
// }

fn get_buy_list(mut to_buy: f32, mut available: Vec<f32>) -> Vec<f32> {
    let mut buy_list = vec![];
    available.sort_by(|a, b| a.partial_cmp(b).unwrap());
    while to_buy >= 0.1f32 {
        println!("{to_buy}");
        println!("{available:?}");
        if to_buy < *available.first().unwrap()
            || approx_eq!(f32, to_buy, *available.first().unwrap())
        {
            println!(" less {to_buy}");
            buy_list.push(available[0]);
            to_buy -= available[0];
            break;
        }
        if to_buy > *available.last().unwrap()
            || approx_eq!(f32, to_buy, *available.last().unwrap())
        {
            println!(" greater {to_buy}");
            buy_list.push(*available.last().unwrap());
            to_buy -= available.last().unwrap();
            println!("{to_buy}");
            break;
        }
        for x in &available {
            if *x > to_buy || approx_eq!(f32, *x, to_buy) {
                buy_list.push(*x);
                to_buy -= x;
                break;
            }
        }
    }
    buy_list
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct RecipeIngredient {
    pub name: String,
    pub quantity: f32,
}
// #[derive(Deserialize)]
// struct Recipe {
//     pub name: String,
//     pub ingredients: Vec<RecipeIngredient>,
// }

type Recipe = HashMap<String, Vec<RecipeIngredient>>;
type RecipeBuyable = HashMap<String, Vec<f32>>;

#[derive(Tabled)]
struct PrintableList {
    pub name: String,
    #[tabled(display_with = "display_vec")]
    pub list: Vec<f32>,
}

fn display_vec(input: &Vec<f32>) -> String {
    format!("{input:?}")
}
#[derive(Deserialize, Serialize)]
struct MealPlan {
    pub name: String,
    pub recipes: Vec<String>,
}
// #[derive(Deserialize)]
// struct RecipePackage {
//     pub name: String,
//     pub available_quantities: Vec<f32>,
// }

// impl RecipePackage {
//  fn get_buy_packets(&self, quant: Quantity) -> Vec<Quantity>{
//         if let Quantity::Mass(mass) = quant =  {
//             self.available_quantities.into_iter().filter_mass().collect();

//         }

// }
// }
slint::slint! {
    export component MainWindow inherits Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}

fn main() {
    MainWindow::new().unwrap().run().unwrap();
    let buyable = std::fs::read_to_string("buyable.json").unwrap();
    // let mut recipes = Recipe::new();
    // recipes.insert(
    //     "test".to_string(),
    //     vec![RecipeIngredient {
    //         name: "foo".to_string(),
    //         quantity: uom::si::mass::Mass::new::<gram>(200f32),
    //     }],
    // );
    // let recipes = serde_json::to_string_pretty(&recipes).unwrap();
    // std::fs::write("recipes.json", recipes).unwrap();
    let test = "200 g".parse::<Mass>().unwrap();
    let recipe = std::fs::read_to_string("recipes.json").unwrap();
    let meal_plan = std::fs::read_to_string("meal_plan.json").unwrap();
    let recipe_packages: RecipeBuyable = serde_json::from_str(&buyable).unwrap();
    let recipe_list: Recipe = serde_json::from_str(&recipe).unwrap();
    println!("{recipe}");
    let meal_plan: MealPlan = serde_json::from_str(&meal_plan).unwrap();

    let mut shopping_list = vec![];
    for recipe in meal_plan.recipes {
        let mut recipe = recipe_list[&recipe].clone();
        shopping_list.append(&mut recipe);
    }
    let output_list = shopping_list.into_iter().fold(
        HashMap::new(),
        |mut acc: HashMap<String, RecipeIngredient>, mut x| {
            let e = acc.entry(x.name.clone());
            match e {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().quantity += x.quantity;
                }
                std::collections::hash_map::Entry::Vacant(_) => {
                    acc.insert(x.name.clone(), x.clone());
                }
            }

            acc
        },
    );
    // let buying_list = shopping_list.into_iter().fold(vec![], |acc, x| x.)
    let buy_lists: Vec<PrintableList> = output_list
        .iter()
        .map(|x| PrintableList {
            name: x.0.clone(),
            list: get_buy_list(x.1.quantity, recipe_packages[x.0].clone()),
        })
        .collect();
    let output = Table::new(buy_lists).to_string();
    println!("{output}");
}
