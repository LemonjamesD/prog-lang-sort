use lazy_static::lazy_static;
use paste::paste;

use std::{fs, env};
use std::collections::HashMap;

macro_rules! mapped_mode {
    ($name:literal => $enumed:ident) => {
        
    }
}

fn mapped_mode(mode: &str) -> LangType {
    match mode {
        "Array" => LangType::Array,
        "Agent" => LangType::Agent,
        "Aspect:" => LangType::Aspect,
        "Assembly:" => LangType::Assembly,
        "Authoring:" => LangType::Authoring,
        "Concatenative:" => LangType::Concatenative,
        "Constraint:" => LangType::Constraint,
        "CommandLine:" => LangType::CommandLine,
        "Compiled:" => LangType::Compiled,
        "Concurrent:" => LangType::Concurrent,
        "Curly:" => LangType::Curly,
        "Dataflow:" => LangType::Dataflow,
        "DataOriented:" => LangType::Data,
        "Decision:" => LangType::Decision,
        "Declarative:" => LangType::Declarative,
        "Embeddable:" => LangType::Embeddable,
        "Educational:" => LangType::Educational,
        "Esoteric:" => LangType::Esoteric,
        "Extension:" => LangType::Extension,
        "Fourth-gen:" => LangType::FourthGen,
        "Functional Pure:" => LangType::PureFunctional,
        "Functional Impure:" => LangType::ImpureFunctional,
        "Hardware description:" => LangType::Hardware,
        "Imperative:" => LangType::Imperative,
        "Interactive:" => LangType::Interactive,
        "Interpreted:" => LangType::Interpreted,
        "GC:" => LangType::GarbageCollected,
        "Manual:" => LangType::ManualMemory,
        "Partial:" => LangType::PartialManual,
        "Optional:" => LangType::OptionalManual,
        "Deterministic:" => LangType::DeterministicManual,
        "RC:" => LangType::RcMemory,
        "LISPS:" => LangType::List,
        "Little:" => LangType::Little,
        "Logic:" => LangType::Logic,
        "Macro:" => LangType::Macro,
        "Metaprogramming:" => LangType::Meta,
        "Multiparadign:" => LangType::Multi,
        "Numerical:" => LangType::Numerical,
        "Non-English:" => LangType::NonEnglish,
        "OOP Class Multi:" => LangType::OOClassMultiple,
        "OOP Class Single:" => LangType::OOClassSingle,
        "OO Prototype:" => LangType::OOPrototype,
        "Offside:" => LangType::Offside,
        "Procedural:" => LangType::Procedural,
        "Reflective:" => LangType::Reflective,
        "Rule-based:" => LangType::RuleBased,
        "Scripting:" => LangType::Scripting,
        "Stack:" => LangType::StackBased,
        "Sync:" => LangType::Sync,
        "Shading Real:" => LangType::ShadingReal,
        "Shading Offline:" => LangType::ShadingOffline,
        "Syntax Handling:" => LangType::Syntax,
        "Transformation:" => LangType::Transformation,
        "Visual:" => LangType::Visual,
        "Wirth:" => LangType::Wirth,
        "XML:" => LangType::XMLBased,
        _ => todo!(),
    }
}

lazy_static! {
    pub static ref LANGS: Vec<Lang> = {
        // Gotta get every lang and their categories
        let contents = fs::read_to_string("data2.txt").unwrap().split('\n').map(|x| -> String { x.into() }).collect::<Vec<String>>();
        let mut vec = vec![];

        for name in contents {
            let splitted = name.split("|:|").collect::<Vec<&str>>();
            if splitted[0] == "" {
                continue;
            }
            let names = splitted[0].split("|").collect::<Vec<&str>>();
            let categories = splitted[1].split(", ").map(|x| x.inverse_display()).collect::<Vec<LangType>>();

            vec.push(Lang {
                name: names[0].to_string(),
                display: names[1].to_string(),
                types: categories,
            })
        }

        vec
    };
}

#[derive(Debug, Clone)]
pub struct Lang {
    pub name: String,
    pub display: String,
    pub types: Vec<LangType>
}

impl Lang {
    fn new(name: String, types: Vec<LangType>) -> Self {
        Self {
            name: name.to_lowercase(),
            display: name,
            types
        }
    }
}

macro_rules! define_type {
    ($($field_name:ident => $debug_name:literal),*) => {
        #[derive(Clone)]
        pub enum LangType {
            $(
                $field_name,
            )*
        }

        impl ::std::fmt::Display for LangType {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                f.write_str(match self {
                    $(
                        LangType::$field_name => $debug_name,
                    )*
                })
            }
        }
        impl ::std::fmt::Debug for LangType {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                f.write_str(match self {
                    $(
                        LangType::$field_name => $debug_name,
                    )*
                })
            }
        }

        pub trait LangTypeInverseDisplay<T> {
            fn inverse_display(&self) -> T;
        }
        
        impl LangTypeInverseDisplay<LangType> for String {
            fn inverse_display(&self) -> LangType {
                match self.as_str() {
                    $(
                        $debug_name => LangType::$field_name,
                    )*
                    _ => unreachable!()
                }
            }
        }
        impl LangTypeInverseDisplay<LangType> for &str {
            fn inverse_display(&self) -> LangType {
                match *self {
                    $(
                        $debug_name => LangType::$field_name,
                    )*
                    _ => unreachable!(),
                }
            }
        }
    }
}

// Based on wikipedia page: https://en.wikipedia.org/wiki/List_of_programming_languages_by_type
define_type! {
    Array => "Array",
    Agent => "Agent-oriented",
    Aspect => "Aspect-oriented",
    Assembly => "Assembly",
    Authoring => "Authoring",
    Concatenative => "Concatenative",
    Constraint => "Constraint",
    CommandLine => "Command-line interface",
    Compiled => "Compiled",
    Concurrent => "Concurrent",
    Curly => "Curly-bracket",
    Dataflow => "Dataflow",
    Data => "Data-orientened",
    Decision => "Decision table",
    Declarative => "Declarative",
    Embeddable => "Embeddable",
    Educational => "Educational",
    Esoteric => "Esoteric",
    Extension => "Extension",
    FourthGen => "Fourth-generation",
    PureFunctional => "Pure Functional",
    ImpureFunctional => "Impure Functional",
    Hardware => "Hardware description languages",
    Imperative => "Imperative",
    Interactive => "Interactive",
    Interpreted => "Interpreted",
    Iterative => "Iterative",
    GarbageCollected => "Garbage Collected",
    ManualMemory => "Manual Memory Management",
    PartialManual => "Partial Manual Memory Management",
    OptionalManual => "Optional Manual Memory Management",
    DeterministicManual => "Deterministic Memory Management",
    RcMemory => "Automatic Reference Counting",
    List => "List-based (LISPs)",
    Little => "Little",
    Logic => "Logic-based",
    Macro => "Macro",
    Meta => "Metaprogramming",
    Multi => "Multiparadigm",
    Numerical => "Numerical",
    NonEnglish => "Non-English-based",
    OOClassSingle => "Object-oriented class-based single dispatch",
    OOClassMultiple => "Object-oriented class-based multiple dispatch",
    OOPrototype => "Object-oriented prototype-based",
    Offside => "Off-side rule",
    Procedural => "Procedural",
    Reflective => "Reflective",
    RuleBased => "Rule-based",
    Scripting => "Scripting",
    StackBased => "Stack-based",
    Sync => "Synchronous",
    ShadingReal => "Shading Real-time",
    ShadingOffline => "Shading Offline",
    Syntax => "Syntax-handling",
    Transformation => "Transformation",
    Visual => "Visual",
    Wirth => "Wirth",
    XMLBased => "XML-based"
}
