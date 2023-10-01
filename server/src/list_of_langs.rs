use lazy_static::lazy_static;
use paste::paste;

use std::{fs, env};
use std::collections::HashMap;

macro_rules! new_lang {
    ($name:literal, $($lang_type:ident)*) => {
        Lang::new($name.to_string(), vec![$(LangType::$lang_type,)*])
    }
}

fn mapped_mode(mode: String) -> LangType {
    match &mode {
        "Array:" => ,
        "Authoring:" => ,
        "Concatenative:" => ,
        "Constraint:" => ,
        "CommandLine:" => ,
        "Compiled:" => ,
        "Concurrent:" => ,
        "Curly:" => ,
        "Dataflow:" => ,
        "DataOriented:" => ,
        "Decision:" => ,
        "Declarative:" => ,
        "Embeddable:" => ,
        "Educational:" => ,
        "Esoteric:" => ,
        "Extension:" => ,
        "Fourth-gen:" => ,
        "Functional Pure:" => ,
        "Functional Impure:" => ,
        "Hardware description:" => ,
        "Imperative:" => ,
        "Interactive:" => ,
        "Interpreted:" => ,
        "GC:" => ,
        "Manual:" => ,
        "Partial:" => ,
        "Optional:" => ,
        "Deterministic:" => ,
        "RC:" => ,
        "LISPS:" => ,
        "Little:" => ,
        "Logic:" => ,
        "Macro:" => ,
        "Metaprogramming:" => ,
        "Multiparadign:" => ,
        "Numerical:" => ,
        "Non-English:" => ,
        "OOP Class Multi:" => ,
        "OOP Class Single:" => ,
        "OO Prototype:" => ,
        "Offside:" => ,
        "Procedural:" => ,
        "Reflective:" => ,
        "Rule-based:" => ,
        "Scripting:" => ,
        "Stack:" => ,
        "Sync:" => ,
        "Shading Real:" => ,
        "Shading Offline:" => ,
        "Syntax Handling:" => ,
        "Transformation:" => ,
        "Visual:" => ,
        "Wirth:" => ,
        XML:
        _ => todo!(),
    }
}

lazy_static! {
    pub static ref LANGS: Vec<Lang> = {
        // Gotta get every lang and their categories
        let contents = fs::read_to_string("data.txt").unwrap().split('\n').map(|x| -> String { x.into() }).collect::<Vec<String>>();
        // What category are we in?
        let mut current_mode = String::new();
        // What we're gonna turn into the Lang at the end
        // let mut hashed = HashMap::new();

        for name in contents {
            if name.chars().last().unwrap() == ':' {
                current_mode = name.clone();
                println!("{name}");
                continue;
            }

            // let _ = hashed.entry(&name).or_insert(vec![]);
            // let mut entry = hashed.get(&name).unwrap();
            // hashed.insert(&name, )
        }

        vec![]
    };
}

#[derive(Debug)]
pub struct Lang {
    name: String,
    types: Vec<LangType>
}

impl Lang {
    fn new(name: String, types: Vec<LangType>) -> Self {
        Self {
            name,
            types
        }
    }
}

macro_rules! define_type {
    ($($field_name:ident => $debug_name:literal),*) => {
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
    }
}

// Based on wikipedia page: https://en.wikipedia.org/wiki/List_of_programming_languages_by_type
define_type! {
    Array => "Array",
    Agent => "Agent-oriented",
    Aspect => "Aspect-oriented",
    Authoring => "Authoring",
    Concatenative => "Concatenative",
    Constraint => "Constraint",
    CommandLine => "Command-line interface",
    Compiled => "Compiled",
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
    Functional => "Functional",
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
    Logig => "Logic-based",
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
    Query => "Query",
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
