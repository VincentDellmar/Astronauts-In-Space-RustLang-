use std::error::Error;
use serde::Deserialize;
use druid::widget::{Flex, Label};
use druid::{AppLauncher, Widget, WidgetExt, WindowDesc, FontDescriptor, FontFamily, FontWeight, Color};


#[derive(Deserialize, Debug)]
struct Response {
    number: i64,
    people: Vec<PeopleStruct>,
}

#[derive(Deserialize, Debug)]
struct PeopleStruct {
    name: String,
    craft: String,
}



#[tokio::main]
    // Fun fact, I made this function in less than 10 minutes.
    async fn main() -> Result<(), Box<dyn Error>> {
        let http_response = reqwest::get("http://api.open-notify.org/astros.json").await?;
    
        let response = http_response.json::<Response>().await?;
    
        let main_window = WindowDesc::new(ui_builder(response))
            .title("Astronauts");
    
        let g: i8 = -1;
    
        
        AppLauncher::with_window(main_window)
            .launch(g);

        Ok(())
    }

    // Fun Fact, this function took 2 days to get it do what I wanted it to do because some rust packages have locks on some things to prevent errors. 
    // Yes I'm ranting in code comments literally no one is going to see.

    fn ui_builder(wo: Response) -> impl Widget<i8> {
        //TOP LABEL
        let font = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_weight(FontWeight::BOLD)
        .with_size(48.0);

        let top_label = Label::new("Astronaut Statistics")
            .with_font(font)
            .with_text_color(Color::rgb(0.592156863, 0.0156862745, 0.749019608))
            .padding(5.0)
            .center();
        
        //LABEL BELOW
        let font_1 = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_weight(FontWeight::LIGHT)
        .with_size(38.0);

        let mut new = String::new();
        new.push_str("There are ");
        new.push_str(&wo.number.to_string());
        new.push_str(" astronauts in space.");

        let label_1 = Label::new(new)
            .with_font(font_1)
            .with_text_color(Color::rgb(0.592156863, 0.0156862745, 0.749019608))
            .padding(0.5)
            .center();   

        

        // Astronaut String Creation which is extremely flawed but I've spent too long trying to find 
        // a different method and I'm not about to fork an entire package just to solve a single problem.
        // I've spent 2 hours trying to fix this, No I'm not joking.

        
        let mut astro_data = String::new(); 
        astro_data.push_str("
        ");

        let mut n = 0;
        
        while n < wo.number {
            let m = n as usize;
            
            astro_data.push_str("
            ");
            astro_data.push_str(&wo.people[m].name);
            astro_data.push_str(" is on the ");
            astro_data.push_str(&wo.people[m].craft);

            n += 1;
        }
       

        //Astronaut Label
        let font_2 = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_size(33.0);

        let new_label = Label::new(astro_data)
        .with_font(font_2)
        .with_text_color(Color::rgb(0.592156863, 0.0156862745, 0.749019608))
        .padding(0.5)
        .center(); 
        

        //Initialize UI.
        
        let flex_col = Flex::column()
        .with_child(top_label)
        .with_child(label_1)
        .with_child(new_label);

        return flex_col;
    }







