// ============================================================
// Système de couleurs pour le terminal.
// Fournit un enum `Couleur` et une macro `colore!` pour colorer
// du texte sans écrire les codes ANSI à la main.
//
// Utilisation :
//   use crate::couleur::Couleur;
//
//   // colorer un morceau de texte
//   let s = Couleur::Vert.peindre("bravo !");
//   println!("{}", s);
//
//   // avec la macro (renvoie un String)
//   let s = colore!(Vert, "texte vert");
//   let s = colore!(RougeGras, "+25 000 Aura");
//
//   // format! intégré
//   let s = colore!(Cyan, "Heure : {}", world.format_time());
// ============================================================

/// Palette de couleurs ANSI utilisée dans le jeu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Couleur {
    Rouge,
    Vert,
    Jaune,
    Bleu,
    Magenta,
    Cyan,
    Gris,

    // variantes grasses (bold)
    RougeGras,
    VertGras,
    JauneGras,
    BleuGras,
    MagentaGras,
    CyanGras,
}

impl Couleur {
    /// Renvoie le code ANSI d'ouverture pour cette couleur.
    pub const fn code(&self) -> &'static str {
        match self {
            Couleur::Rouge       => "\x1B[31m",
            Couleur::Vert        => "\x1B[32m",
            Couleur::Jaune       => "\x1B[33m",
            Couleur::Bleu        => "\x1B[34m",
            Couleur::Magenta     => "\x1B[35m",
            Couleur::Cyan        => "\x1B[36m",
            Couleur::Gris        => "\x1B[90m",

            Couleur::RougeGras   => "\x1B[1;31m",
            Couleur::VertGras    => "\x1B[1;32m",
            Couleur::JauneGras   => "\x1B[1;33m",
            Couleur::BleuGras    => "\x1B[1;34m",
            Couleur::MagentaGras => "\x1B[1;35m",
            Couleur::CyanGras    => "\x1B[1;36m",
        }
    }

    /// Code ANSI de remise à zéro.
    pub const RESET: &'static str = "\x1B[0m";

    /// Colore `texte` avec cette couleur et renvoie un String.
    pub fn peindre(&self, texte: &str) -> String {
        format!("{}{}{}", self.code(), texte, Self::RESET)
    }
}

/// Macro principale : renvoie un `String` coloré.
///
/// ```ignore
/// colore!(Vert, "bravo")
/// colore!(RougeGras, "dégâts : {}", 42)
/// ```
#[macro_export]
macro_rules! colore {
    ($couleur:ident, $($arg:tt)*) => {{
        use $crate::couleur::Couleur;
        let texte = format!($($arg)*);
        Couleur::$couleur.peindre(&texte)
    }};
}
