use silent::prelude::*;

async fn index(_req: Request) -> Result<Response> {
    Ok(Response::empty())
}

#[tokio::main]
async fn main() {
    let route = Route::new("").get(index).append(
        Route::new("user")
            .append(
                Route::new("<id>").get(|req| async move { req.get_path_params::<String>("id") }),
            )
            .post(index),
    );
    Server::new()
        .bind("0.0.0.0:30001".parse().unwrap())
        .serve(route)
        .await;
}
