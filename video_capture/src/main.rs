use opencv::{
    core::Mat,
    highgui::{self, named_window, imshow},
    videoio::{self, VideoCaptureTrait},
};

fn main() {
    let device_id = 0;

    // カメラをキャプチャするためのVideoCaptureオブジェクトを作成
    let mut capture = videoio::VideoCapture::new(device_id, videoio::CAP_ANY).unwrap();
    capture.set(videoio::CAP_PROP_FRAME_WIDTH, 640.0).unwrap();
    capture.set(videoio::CAP_PROP_FRAME_HEIGHT, 480.0).unwrap();

    // ウィンドウを作成して表示
    named_window("Webcam", highgui::WINDOW_NORMAL).unwrap();

    loop {
        let mut frame = Mat::default();
        capture.read(&mut frame).unwrap();

        imshow("Webcam", &frame).unwrap();
        
        // キーボード入力を待つ
        if highgui::wait_key(10).unwrap() > 0 {
            break;
        }
    }
}
