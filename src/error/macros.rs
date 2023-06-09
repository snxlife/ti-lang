#[macro_export]
macro_rules! build_ti_error {
  (@at $token: expr, @note $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}: (line {}:{}:{})\n\t", "note".white().bold(), $token.t_at.0, $token.t_at.1.0, $token.t_at.1.1);
      println!($($t) *);
      std::process::exit(1)
    }
  };
  (@at $token: expr, @info $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}: (line {}:{}:{})\n\t", "info".white().bold(), $token.t_at.0, $token.t_at.1.0, $token.t_at.1.1);
      println!($($t) *);
      std::process::exit(1)
    }
  };
  (@at $token: expr, @err $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}: (line {}:{}:{})\n\t", "error".red().bold(), $token.t_at.0, $token.t_at.1.0, $token.t_at.1.1);
      println!($($t) *);
      std::process::exit(1)
      // panic!()
    }
  };
  (@at $token: expr, @warn $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}: (line {}:{}:{})\n\t", "warn".yellow().bold(), $token.t_at.0, $token.t_at.1.0, $token.t_at.1.1);
      println!($($t) *);
      std::process::exit(1)
    }
  };
  (@at $token: expr, @help $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}: (line {}:{}:{})\n\t", "help".yellow().bold(), $token.t_at.0, $token.t_at.1.0, $token.t_at.1.1);
      println!($($t) *);
      std::process::exit(1)
    }
  };
  (@note $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}:\n\t", "note".white().bold());
      println!($($t) *);
      std::process::exit(1)
    }
  };
  (@info $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}:\n\t", "info".white().bold());
      println!($($t) *);
      std::process::exit(1)
    }
  };
  (@err $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}:\n\t", "error".red().bold());
      println!($($t) *);
      std::process::exit(1)
    }
  };
  (@warn $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}:\n\t", "warn".yellow().bold());
      println!($($t) *);
      std::process::exit(1)
    }
  };
  (@help $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}:\n\t", "help".green().bold());
      println!($($t) *);
      std::process::exit(1)
    }
  };
}