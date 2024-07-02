use windows_sys::Win32::UI::WindowsAndMessaging::{SetCursorPos, GetCursorPos};
use windows_sys::Win32::Foundation::POINT;

async fn mouse_move_absolute(mut request: tide::Request<()>) -> tide::Result {
    let coordinate = request.body_json::<[i32; 2]>().await?;

    unsafe {
        SetCursorPos(coordinate[0], coordinate[1]);
    }

    Ok(tide::Response::new(200))
}

async fn mouse_move_relative(mut request: tide::Request<()>) -> tide::Result {
    let coordinate = request.body_json::<[i32; 2]>().await?;

    unsafe {
        let mut current_coordinate = POINT { x: 0, y: 0 };

        GetCursorPos(&mut current_coordinate);
        SetCursorPos(current_coordinate.x + coordinate[0], current_coordinate.y + coordinate[1]);
    }

    Ok(tide::Response::new(200))
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/mousemoveabs").post(mouse_move_absolute);
    app.at("/mousemoverel").post(mouse_move_relative);

    app.listen("localhost:8080").await?;

    Ok(())
}
