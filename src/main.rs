/*
By: <Aum markandey>
Date: 2026-09-22
Program Details: <You have been hired by the owner of "Bob's General Store" to create a program that all his employees can use to find the cost and change for each purchase. 
Bob's store has 5 items that it sells and each item is a different cost that you can choice. Once the employee calculates the total for the customer 
it should then let them input how much money they were given and tell them the change they need to give to the customer.

The program should be easy to use and should catch all possible errors. It should also only display 2 decimals and look nice.>
*/

mod ui;
mod utils;

//use crate::ui::grid::draw_grid;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::ui::text_button::TextButton;
use macroquad::prelude::*;
use crate::utils::preload_image::TextureManager;
use crate::utils::preload_image::LoadingScreenOptions; // If you want to customize the loading screen
use crate::utils::preload_image::GifLoadingScreenInfo;


use crate::utils::scale::use_virtual_resolution; // If you want to add animated GIFs to loading screen
use crate::ui::text_input::TextInput;
/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "hello_11u".to_string(),
        window_width: 1700,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let texture_manager = TextureManager::new();
    texture_manager.preload_with_loading_screen(&["assets/pie.png", "assets/balls.png", "assets/hassan.png", "assets/cake.png", "assets/cookies.png", "assets/edward.png"], None, None).await;
    
    let img_balls = StillImage::from_preload(
        texture_manager.get_preload("assets/balls.png").unwrap(),
        100.0,
        200.0,
        500.0,
        60.0,
        true,
        1.0,
    );
    let img_hassan = StillImage::from_preload(
        texture_manager.get_preload("assets/hassan.png").unwrap(),
        100.0,
        200.0,
        750.0,
        60.0,
        true,
        1.0,
    );
    let img_cookies = StillImage::from_preload(
        texture_manager.get_preload("assets/cookies.png").unwrap(),
        100.0,
        200.0,
        950.0,
        60.0,
        true,
        1.0,
    );
    let img_pie = StillImage::from_preload(
        texture_manager.get_preload("assets/pie.png").unwrap(),
        100.0,
        200.0,
        1150.0,
        60.0,
        true,
        1.0,
    );
    let img_cake = StillImage::from_preload(
        texture_manager.get_preload("assets/cake.png").unwrap(),
        100.0,
        200.0,
        1350.0,
        60.0,
        true,
        1.0,
    );
    let img_edward = StillImage::from_preload(
        texture_manager.get_preload("assets/edward.png").unwrap(),
        1700.0,
        768.0,
        0.0,
        0.0,
        true,
        1.0,
    );
let mut input_balls  = TextInput::new(500.0, 300.0, 150.0, 40.0, 25.0);
let mut input_hassan  = TextInput::new(750.0, 300.0, 150.0, 40.0, 25.0);
let mut input_cookies  = TextInput::new(950.0, 300.0, 150.0, 40.0, 25.0);
let mut input_cake  = TextInput::new(1150.0, 300.0, 150.0, 40.0, 25.0);
let mut input_pie  = TextInput::new(1350.0, 300.0, 150.0, 40.0, 25.0);
let mut input_given_money  = TextInput::new(50.0, 350.0, 350.0, 40.0, 20.0);
let mut total: f64 = 0.0;
input_balls.set_allowed_chars("0123456789");

input_hassan.set_allowed_chars("0123456789");

input_cookies.set_allowed_chars("0123456789");

input_cake.set_allowed_chars("0123456789");

input_pie.set_allowed_chars("0123456789");
input_given_money.set_prompt("Enter amount of money given to pay:");
input_given_money.set_allowed_chars("0123456789.");
    let mut lbl_text = Label::new("Input number of items in each text box \n each image above the text box \n is the item type your inputting", 10.0, 100.0, 30);
    let mut btn_calc_total = TextButton::new(50.0, 250.0, 200.0, 60.0, "Calculate Total", WHITE, RED, 30);
    let mut btn_calc_change = TextButton::new(50.0, 450.0, 200.0, 60.0, "Calculate Change", WHITE, RED, 30);
    let mut btn_exit = TextButton::new(50.0, 650.0, 200.0, 60.0, "Exit", WHITE, RED, 30);
    input_given_money.set_prompt_color(BLACK);
    btn_calc_total.with_text_color(BLACK); // Sets the normal text color
    btn_calc_total.with_hover_text_color(WHITE);
    btn_calc_change.with_text_color(BLACK); // Sets the normal text color
    btn_calc_change.with_hover_text_color(WHITE);
    btn_exit.with_text_color(BLACK); // Sets the normal text color
    btn_exit.with_hover_text_color(WHITE);
    btn_calc_change.enabled = false;
    input_given_money.set_enabled(false);
let mut edward: bool = false;
    loop {
        clear_background(WHITE);
        //draw_grid(50.0, BROWN);
        use_virtual_resolution(1700.0, 768.0);
        if btn_calc_total.click() {
            
            lbl_text.set_text("Calculating...");
            
            let balls_text = input_balls.get_text();
            let input_balls = balls_text.trim().parse::<f64>();
            let edwardcheck = balls_text.trim().parse::<f64>();
            let hassans_text = input_hassan.get_text();
            let input_hassan = hassans_text.trim().parse::<f64>();
            let cookies_text = input_cookies.get_text();
            let input_cookies = cookies_text.trim().parse::<f64>();
            let cake_text = input_cake.get_text();
            let input_cake = cake_text.trim().parse::<f64>();
            let pie_text = input_pie.get_text();
            let input_pie = pie_text.trim().parse::<f64>();
            total = (1.5 * input_balls.unwrap_or(0.0))  + (0.5 * input_hassan.unwrap_or(0.0)) + (1.3  * input_cookies.unwrap_or(0.0)) + (4.0 * input_cake.unwrap_or(0.0)) +( 6.0 * input_pie.unwrap_or(0.0));
            
            lbl_text.set_text(format!("Total: ${:.2}", total));
            btn_calc_change.enabled = true;
    input_given_money.set_enabled(true);
    if edwardcheck.unwrap_or(0.0) == 2010.0{
        edward = true;
    }
        };
        
        
        if btn_exit.click() {
            break;
        };
        
        img_balls.draw();
        input_balls.draw();
        img_hassan.draw();
        input_hassan.draw();
        img_cookies.draw();
        input_cookies.draw();
        img_cake.draw();
        input_cake.draw();
        img_pie.draw();
        input_pie.draw();
        
        lbl_text.draw();
        input_given_money.draw();
        if btn_calc_change.click(){
           let given_money_text = input_given_money.get_text();
            let given_money = given_money_text.trim().parse::<f64>();
            let change = given_money.unwrap_or(0.0) - total;
            if given_money_text.trim().is_empty() {
                lbl_text.set_text("Please enter the amount of money given.");
            } else if input_given_money.get_text() == "."{
                lbl_text.set_text("Please enter a valid amount of money.");
            } else
            if change > 0.0 {
                lbl_text.set_text(format!("Change: ${:.2}", change));
            }
            else if change == 0.0 {
                lbl_text.set_text("No change needed");

            }
            else {
                lbl_text.set_text(format!("still owe money : ${:.2}", change.abs()));
            }
            
        }
         if edward == true {
            img_edward.draw();
            lbl_text.set_text("i dont know him him");
            lbl_text.with_colors(WHITE, Some(DARKGRAY));
        }
        lbl_text.draw();
        next_frame().await;
       
    }
}
