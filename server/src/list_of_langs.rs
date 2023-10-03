use lazy_static::lazy_static;
use paste::paste;
use serde_derive::{Serialize, Deserialize};
use serde_with::serde_as;
use serde_with::SerializeDisplay;

use std::fmt::Display;
use std::{fs, env};
use std::collections::HashMap;

macro_rules! mapped_mode {
    ($($name:literal => $enumed:ident),*) => {
        paste! {
            $(const [<$enumed:upper>]: &'static str = $name;)*
            fn mapped_mode(mode: &str) -> LangType {
                match mode {
                    $(
                        [<$enumed:upper>] => LangType::$enumed,
                    )*
                    _ => unreachable!(),
                }
            }
        }
    }
}

mapped_mode! {
    "Array" => Array,
    "Agent" => Agent,
    "Aspect:" => Aspect,
    "Assembly:" => Assembly,
    "Authoring:" => Authoring,
    "Concatenative:" => Concatenative,
    "Constraint:" => Constraint,
    "CommandLine:" => CommandLine,
    "Compiled:" => Compiled,
    "Concurrent:" => Concurrent,
    "Curly:" => Curly,
    "Dataflow:" => Dataflow,
    "DataOriented:" => Data,
    "Decision:" => Decision,
    "Declarative:" => Declarative,
    "Embeddable:" => Embeddable,
    "Educational:" => Educational,
    "Esoteric:" => Esoteric,
    "Extension:" => Extension,
    "Fourth-gen:" => FourthGen,
    "Functional Pure:" => PureFunctional,
    "Functional Impure:" => ImpureFunctional,
    "Hardware description:" => Hardware,
    "Imperative:" => Imperative,
    "Interactive:" => Interactive,
    "Interpreted:" => Interpreted,
    "GC:" => GarbageCollected,
    "Manual:" => ManualMemory,
    "Partial:" => PartialManual,
    "Optional:" => OptionalManual,
    "Deterministic:" => DeterministicManual,
    "RC:" => RcMemory,
    "LISPS:" => List,
    "Little:" => Little,
    "Logic:" => Logic,
    "Macro:" => Macro,
    "Metaprogramming:" => Meta,
    "Multiparadign:" => Multi,
    "Numerical:" => Numerical,
    "Non-English:" => NonEnglish,
    "OOP Class Multi:" => OOClassMultiple,
    "OOP Class Single:" => OOClassSingle,
    "OO Prototype:" => OOPrototype,
    "Offside:" => Offside,
    "Procedural:" => Procedural,
    "Reflective:" => Reflective,
    "Rule-based:" => RuleBased,
    "Scripting:" => Scripting,
    "Stack:" => StackBased,
    "Sync:" => Sync,
    "Shading Real:" => ShadingReal,
    "Shading Offline:" => ShadingOffline,
    "Syntax Handling:" => Syntax,
    "Transformation:" => Transformation,
    "Visual:" => Visual,
    "Wirth:" => Wirth,
    "XML:" => XMLBased
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
    pub static ref SERIALIZED_LANGS: String = {
        serde_json::to_string(&*LANGS).unwrap()
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        #[serde_as]
        #[derive(Clone, SerializeDisplay, Deserialize)]
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
