use hls::send_request;
fn main() {
    let url = "https://videos-fms.jwpsrv.com/631fb3b6_0xc4c33e4ac3f312a591a1d147b14d8276f4cfa56b/content/conversions/LOPLPiDX/videos/IFBsp7yL-24721145.mp4-1.ts";
    send_request(url);
}
