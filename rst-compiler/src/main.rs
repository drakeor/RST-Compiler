
// Simplest compilation from smalltalk to assembly
use std::fs::*;
use std::io::*;

fn main() -> std::io::Result<()> {
    let source_code = "2";

    let mut output_file = File::create("output.s")?;

    write!(output_file, "   .globl main \n");
    write!(output_file, "main:\n");
    write!(output_file, "   movl $2, %eax \n");
    write!(output_file, "   ret \n");

    println!("Done");
    Ok(())
}
