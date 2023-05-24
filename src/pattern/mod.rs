pub use argument::{Argument, ArgumentType};
pub use input_pattern::InputPattern;
pub use output_pattern::{OutputPattern, ResponseType};
pub use identifier::{Identifier, IdentifierType};
pub use internal_pattern::InternalPattern;

pub mod argument;
pub mod input_pattern;
pub mod output_pattern;
pub mod identifier;
pub mod internal_pattern;

pub struct Pattern {
    input_pattern: InputPattern,
    internal_pattern: InternalPattern,
    output_pattern: OutputPattern,
    identifier: Identifier,
}