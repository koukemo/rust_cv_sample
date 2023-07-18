use opencv::imgcodecs::IMREAD_COLOR;
use opencv::imgcodecs::imread;
use opencv::highgui;

fn main() {
    let image_path = "figures/sample_cat.png";

    // 画像の読み込み
    let image = match imread(image_path, IMREAD_COLOR) {
        Ok(mat) => mat,
        Err(err) => {
            println!("Failed to load image: {}", err);
            return;
        }
    };

    // 画像の表示
    highgui::named_window("Image", highgui::WINDOW_NORMAL).unwrap();
    highgui::imshow("Image", &image).unwrap();
    highgui::wait_key(0).unwrap();
}