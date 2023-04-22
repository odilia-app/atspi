#[macro_export]
macro_rules! cmd {
  ($base:expr, $($value:expr),*) => {
    {   
      let cmd = std::process::Command::new($base)
        $(.arg($value))*
        .output()
        .unwrap()
        .stdout;
      String::from_utf8(cmd).unwrap()
    }   
  }
}

#[macro_export]
macro_rules! addr_via_cmd {
	() => {
	 {
		 let x = cmd!(
				"busctl",
				"--user",
				"call",
				"org.a11y.Bus",
				"/org/a11y/bus",
				"org.a11y.Bus",
				"GetAddress"
			);
			let y = x
				.strip_prefix("s \"")
				.unwrap()
				.trim()
				.strip_suffix('"')
				.unwrap();
			y.to_string()
	 }
	}
}
