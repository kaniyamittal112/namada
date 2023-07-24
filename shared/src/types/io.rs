//! Traits for implementing IO handlers. This is to enable
//! generic IO. The defaults are the obvious Rust native
//! functions.

/// Rust native I/O handling.
pub struct DefaultIo;

impl Io for DefaultIo {}

#[allow(missing_docs)]
pub trait Io {
    fn print(output: impl AsRef<str>) {
        print!("{}", output.as_ref());
    }

    fn flush() {
        use std::io::Write;
        std::io::stdout().flush().unwrap();
    }

    fn println(output: impl AsRef<str>) {
        println!("{}", output.as_ref());
    }

    fn write<W: std::io::Write>(
        mut writer: W,
        output: impl AsRef<str>,
    ) -> std::io::Result<()> {
        write!(writer, "{}", output.as_ref())
    }

    fn writeln<W: std::io::Write>(
        mut writer: W,
        output: impl AsRef<str>,
    ) -> std::io::Result<()> {
        writeln!(writer, "{}", output.as_ref())
    }

    fn eprintln(output: impl AsRef<str>) {
        eprintln!("{}", output.as_ref());
    }

    fn read() -> std::io::Result<String> {
        read_aux(std::io::stdin().lock())
    }

    fn prompt(question: impl AsRef<str>) -> String {
        prompt_aux(
            std::io::stdin().lock(),
            std::io::stdout(),
            question.as_ref(),
        )
    }
}

/// A generic function for displaying a prompt to users and reading
/// in their response.
pub fn prompt_aux<R, W>(mut reader: R, mut writer: W, question: &str) -> String
where
    R: std::io::Read,
    W: std::io::Write,
{
    write!(&mut writer, "{}", question).expect("Unable to write");
    writer.flush().unwrap();
    let mut s = String::new();
    reader.read_to_string(&mut s).expect("Unable to read");
    s
}

/// A generic function for reading input from users
pub fn read_aux<R>(mut reader: R) -> std::io::Result<String>
where
    R: std::io::Read,
{
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    Ok(s)
}

/// Convenience macro for formatting arguments to
/// [`Io::print`]
#[macro_export]
macro_rules! display {
    ($io:ty) => {
      <$io>::print("")
    };
    ($io:ty, $w:expr; $($args:tt)*) => {
        <$io>::write($w, format_args!($($args)*).to_string())
    };
    ($io:ty,$($args:tt)*) => {
        <$io>::print(format_args!($($args)*).to_string())
    };
}

/// Convenience macro for formatting arguments to
/// [`Io::println`] and [`Io::writeln`]
#[macro_export]
macro_rules! display_line {
    ($io:ty) => {
      <$io>::println("")
    };
    ($io:ty, $w:expr; $($args:tt)*) => {
        <$io>::writeln($w, format_args!($($args)*).to_string())
    };
    ($io:ty,$($args:tt)*) => {
        <$io>::println(format_args!($($args)*).to_string())
    };
}

/// Convenience macro for formatting arguments to
/// [`Io::eprintln`]
#[macro_export]
macro_rules! edisplay {
    ($io:ty,$($args:tt)*) => {
        <$io>::eprintln(format_args!($($args)*).to_string())
    };
}

#[macro_export]
/// A convenience macro for formatting the user prompt before
/// forwarding it to the [`Io::prompt`] method.
macro_rules! prompt {
    ($io:ty,$($arg:tt)*) => {{
        <$io>::prompt(format!("{}", format_args!($($arg)*)))
    }}
}
