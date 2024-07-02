use windows_sys::Win32::{
    Foundation::POINT,
    UI::WindowsAndMessaging::{GetCursorPos, SetCursorPos},
};

async fn mouse_move_absolute(mut request: tide::Request<()>) -> tide::Result {
    let coordinate = request.body_json::<[i32; 2]>().await?;

    unsafe {
        SetCursorPos(coordinate[0], coordinate[1]);
    }

    Ok(tide::Response::new(200))
}

async fn mouse_move_relative(mut request: tide::Request<()>) -> tide::Result {
    let coordinate = request.body_json::<[i32; 2]>().await?;
    let mut current_coordinate = POINT { x: 0, y: 0 };

    unsafe {
        GetCursorPos(&mut current_coordinate);
    };
    unsafe {
        SetCursorPos(
            current_coordinate.x + coordinate[0],
            current_coordinate.y + coordinate[1],
        );
    };

    Ok(tide::Response::new(200))
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut server = tide::new();

    server.at("/mousemoveabs").post(mouse_move_absolute);
    server.at("/mousemoverel").post(mouse_move_relative);

    server
        .listen(std::net::SocketAddr::from(([0, 0, 0, 0], 8080)))
        .await?;

    Ok(())
}
