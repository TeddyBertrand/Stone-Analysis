#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgKind {
    Flag,
    Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Analyze,
    Cypher,
    Decypher,
    Help,
    Visualize,
}

impl Mode {
    pub const fn expected_positionals(self) -> usize {
        match self {
            Mode::Analyze  => 2,
            Mode::Cypher   => 3,
            Mode::Decypher => 1,
            Mode::Help     => 0,
            Mode::Visualize => 3,
        }
    }

    pub const fn positional_hint(self) -> &'static str {
        match self {
            Mode::Analyze  => "IN_FILE N",
            Mode::Cypher   => "IN_FILE OUT_FILE MESSAGE",
            Mode::Decypher => "IN_FILE",
            Mode::Help     => "",
            Mode::Visualize => "IN_FILE OUT_FILE MODE",
        }
    }
}

pub struct ArgDef {
    pub short: &'static str,
    pub long:  &'static str,
    pub kind:  ArgKind,
    pub mode:  Option<Mode>,
    pub help:  &'static str,
}

pub const ARG_DEFS: &[ArgDef] = &[
    ArgDef { short: "-a", long: "--analyze",  kind: ArgKind::Flag,  mode: Some(Mode::Analyze),  help: "Analyse un fichier de runes (IN_FILE N)"              },
    ArgDef { short: "-c", long: "--cypher",   kind: ArgKind::Flag,  mode: Some(Mode::Cypher),   help: "Chiffre un message dans une image (IN_FILE OUT_FILE MESSAGE)" },
    ArgDef { short: "-d", long: "--decypher", kind: ArgKind::Flag,  mode: Some(Mode::Decypher), help: "Déchiffre un message caché (IN_FILE)"                 },
    ArgDef { short: "-h", long: "--help",     kind: ArgKind::Flag,  mode: Some(Mode::Help),     help: "Affiche l'aide"          },
    ArgDef { short: "-v", long: "--visualize", kind: ArgKind::Flag, mode: Some(Mode::Visualize), help: "Affiche un spectrogramme (IN_FILE OUT_FILE MODE), graphical ou ascii" },
];
