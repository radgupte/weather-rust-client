mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let user = model::User {
      username: "rad".into(),
      password: "abc54321".into(),
    };

    let auth = reqwest::Client::new()
    .post("http://172-31-13-213:3000/v1/auth")
    .json(&user)
    .send()
    .await?;

    let token_res = auth.json::<model::TokenResponse>().await?;

    // Fetch Weather from the EC2 instance
    let w_token = "Bearer ".to_owned() + &token_res.access_token;

    let weather_res = client
    .get("http://172-31-13-213:3000/v1/weather")
    .header("Authorization", w_token)
    .send()
    .await?;

    let weather = weather_res.json::<model::Weather>().await?;

    println!(
      "\nWeather from server running on EC2 Instance:\n {:?}",
      weather
    );

    // Fetch Greeting from the EC2 instance
    let g_token = "Bearer ".to_owned() + &token_res.access_token;

    let greet_res = client
    .get("http://172-31-13-213:3000/v1/hello")
    .header("Authorization", g_token)
    .send()
    .await?;

    let greet = greet_res.json::<model::GreetingResponse>().await?;

    println!(
      "\nGreeting from server running on EC2 Instance:\n {:?}",
      greet.message
    );

    Ok(())
}
