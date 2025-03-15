use warp::Filter;
use rustic_math::{tokenize, parse, simplify_expression, eval, to_latex};
use urlencoding;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct JsonResponse {
    simplified: String,
    result: String,
}

fn handle_result<T: std::fmt::Debug, E: std::fmt::Display>(result: Result<T, E>) -> String {
    match result {
        Ok(value) => format!("{:?}", value), // Format the successful result
        Err(err) => format!("Error: {}", err), // Format the error message
    }
}

#[tokio::main]
async fn main() {
    // Define the `/simplify` endpoint
    let simplify = warp::path!("simplify" / String)
        .map(|input: String| {
            // un-url-encode the input
            let input = urlencoding::decode(&input).unwrap();
            // Tokenize the input
            let tokens = tokenize(input.to_string());
            // Parse the tokens into expressions
            let expressions = parse(tokens);

            if expressions.len() == 1 {
                // Simplify the expression
                let simplified = simplify_expression(expressions[0].clone());
                // Evaluate the simplified expression
                let result = eval(&simplified);
                // Return the result as a json response
                serde_json::to_string(&JsonResponse {
                    simplified: to_latex(&simplified),
                    result: handle_result(result),
                }).unwrap()
            } else {
                "Error: Multiple expressions are not supported.".to_string()
            }
        });

    // Combine routes and start the server
    let routes = simplify.with(warp::cors().allow_any_origin());
    println!("Starting server on http://localhost:3000");
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}