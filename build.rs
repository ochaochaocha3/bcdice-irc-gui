fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
     * 定義を見たいとき用
    tonic_build::configure()
        .out_dir("pb")
        .compile(
            &["vendor/bcdice-irc-proto/bcdice_irc.proto"],
            &["vendor/bcdice-irc-proto"],
        )
        .expect("failed to compile protos");
    */
    tonic_build::compile_protos("vendor/bcdice-irc-proto/bcdice_irc.proto")?;
    Ok(())
}
