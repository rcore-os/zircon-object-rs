use std::io::Write;

fn main() {

    if std::env::var("TARGET").unwrap().contains("riscv64") {
        let board = std::env::var("PLATFORM");
        let kernel_base_addr: u64 = if board.map_or(false, |x| x.contains("fu740")) {
            0xffffffe080200000
        } else {
            0xffffffc080200000
        };

        let mut fout = std::fs::File::create("src/platform/riscv/kernel-vars.ld").unwrap();
        writeln!(fout, "/* Generated by build.rs. DO NOT EDIT. */").unwrap();
        writeln!(
            fout,
            "PROVIDE_HIDDEN(BASE_ADDRESS = {:#x});",
            kernel_base_addr
        )
        .unwrap();
    }

    // 如果需要链接 rootfs 镜像，将镜像路径设置到环境变量
    #[cfg(feature = "link-user-img")]
    println!(
        "cargo:rustc-env=USER_IMG=zCore/{}.img",
        std::env::var("TARGET").unwrap()
    );
}
