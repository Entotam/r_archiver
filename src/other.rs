// fn archive() -> Result<(), std::io::Error> {
    // let path = "//home//sergei/projects";

    // let tar_gz = File::create("/home/sergei/test.tar.gz")?;
    // let enc = GzEncoder::new(tar_gz, Compression::default());
    // let mut tar = tar::Builder::new(enc);

    // // tar.append_dir_all("", "/home/sergei/projects")?;
    // tar.append_dir("", "/home/sergei/projects")?;
    // Ok(())
// }