mod basics;
mod exporting;
mod metavariables;
mod patterns;
mod scoping;

fn main() {
    basics::demo();
    metavariables::demo();
    exporting::demo();
    scoping::demo();
    patterns::demo();
}
