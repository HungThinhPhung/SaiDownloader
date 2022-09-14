use saidl_hls::download;
use saidl_cli as cli;

fn main() {
    let input = vec![
        "https://videos-fms.jwpsrv.com/6321d963_0x13908ee06145da2ec0089282d29e8aad1df219bb/content/conversions/LOPLPiDX/videos/IFBsp7yL-24721145.mp4-1.ts",
        "https://videos-fms.jwpsrv.com/6321d963_0x13908ee06145da2ec0089282d29e8aad1df219bb/content/conversions/LOPLPiDX/videos/IFBsp7yL-24721145.mp4-2.ts",
        "https://videos-fms.jwpsrv.com/6321d963_0x13908ee06145da2ec0089282d29e8aad1df219bb/content/conversions/LOPLPiDX/videos/IFBsp7yL-24721145.mp4-3.ts",
        "https://videos-fms.jwpsrv.com/6321d963_0x13908ee06145da2ec0089282d29e8aad1df219bb/content/conversions/LOPLPiDX/videos/IFBsp7yL-24721145.mp4-4.ts",
        "https://videos-fms.jwpsrv.com/6321d963_0x13908ee06145da2ec0089282d29e8aad1df219bb/content/conversions/LOPLPiDX/videos/IFBsp7yL-24721145.mp4-5.ts",
        "https://videos-fms.jwpsrv.com/6321d963_0x13908ee06145da2ec0089282d29e8aad1df219bb/content/conversions/LOPLPiDX/videos/IFBsp7yL-24721145.mp4-6.ts",
        "https://videos-fms.jwpsrv.com/6321d963_0x13908ee06145da2ec0089282d29e8aad1df219bb/content/conversions/LOPLPiDX/videos/IFBsp7yL-24721145.mp4-7.ts",
    ];
    // download(input, false);
    cli::run();
}
