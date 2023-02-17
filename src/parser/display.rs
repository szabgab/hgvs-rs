//! Implementation of Display trait.

use std::fmt::Display;

use crate::parser::ds::*;

impl<T> Display for Mu<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mu::Certain(value) => write!(f, "{value}"),
            Mu::Uncertain(value) => write!(f, "({value})"),
        }
    }
}

impl Display for GeneSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for NaEdit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NaEdit::RefAlt {
                reference,
                alternative,
            } => match (reference.len(), alternative.len()) {
                (0, 0) => write!(f, "="),
                (1, 1) => write!(f, "{reference}>{alternative}"),
                (0, _) => write!(f, "delins{alternative}"),
                (_, 0) => write!(f, "del{reference}ins"),
                (_, _) => write!(f, "del{reference}ins{alternative}"),
            },
            NaEdit::NumAlt { count, alternative } => match (count, alternative.len()) {
                (0, 0) => write!(f, "="),
                (0, _) => write!(f, "delins{alternative}"),
                (_, 0) => write!(f, "del{count}ins"),
                (_, _) => write!(f, "del{count}ins{alternative}"),
            },
            NaEdit::DelRef { reference } => write!(f, "del{reference}"),
            NaEdit::DelNum { count } => write!(f, "del{count}"),
            NaEdit::Ins { alternative } => write!(f, "ins{alternative}"),
            NaEdit::Dup { reference } => write!(f, "dup{reference}"),
            NaEdit::InvRef { reference } => write!(f, "inv{reference}"),
            NaEdit::InvNum { count } => write!(f, "inv{count}"),
        }
    }
}

impl Display for UncertainLengthChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UncertainLengthChange::None => write!(f, ""),
            UncertainLengthChange::Unknown => write!(f, "?"),
            UncertainLengthChange::Known(count) => write!(f, "{count}"),
        }
    }
}

impl Display for Accession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for ProteinEdit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProteinEdit::Fs {
                alternative,
                terminal,
                length,
            } => match (alternative, terminal, length) {
                (None, None, UncertainLengthChange::None) => write!(f, "fs"),
                (None, None, UncertainLengthChange::Unknown) => write!(f, "fs?"),
                (None, None, UncertainLengthChange::Known(count)) => write!(f, "fs{count}"),
                (Some(alt), None, UncertainLengthChange::None) => write!(f, "{alt}fs"),
                (Some(alt), None, UncertainLengthChange::Unknown) => write!(f, "{alt}fs?"),
                (Some(alt), None, UncertainLengthChange::Known(count)) => {
                    write!(f, "{alt}fs{count}")
                }
                (None, Some(ter), UncertainLengthChange::None) => write!(f, "fs{ter}"),
                (None, Some(ter), UncertainLengthChange::Unknown) => write!(f, "fs{ter}?"),
                (None, Some(ter), UncertainLengthChange::Known(count)) => {
                    write!(f, "fs{ter}{count}")
                }
                (Some(alt), Some(ter), UncertainLengthChange::None) => {
                    write!(f, "{alt}fs{ter}")
                }
                (Some(alt), Some(ter), UncertainLengthChange::Unknown) => {
                    write!(f, "{alt}fs{ter}?")
                }
                (Some(alt), Some(ter), UncertainLengthChange::Known(count)) => {
                    write!(f, "{alt}fs{ter}{count}")
                }
            },
            ProteinEdit::Ext {
                aa_ext,
                ext_aa,
                change,
            } => match (aa_ext, ext_aa, change) {
                (None, None, UncertainLengthChange::None) => write!(f, "ext"),
                (None, None, UncertainLengthChange::Unknown) => write!(f, "ext?"),
                (None, None, UncertainLengthChange::Known(count)) => write!(f, "ext{count}"),
                (Some(alt), None, UncertainLengthChange::None) => write!(f, "{alt}ext"),
                (Some(alt), None, UncertainLengthChange::Unknown) => write!(f, "{alt}ext?"),
                (Some(alt), None, UncertainLengthChange::Known(count)) => {
                    write!(f, "{alt}ext{count}")
                }
                (None, Some(ter), UncertainLengthChange::None) => write!(f, "ext{ter}"),
                (None, Some(ter), UncertainLengthChange::Unknown) => write!(f, "ext{ter}?"),
                (None, Some(ter), UncertainLengthChange::Known(count)) => {
                    write!(f, "ext{ter}{count}")
                }
                (Some(alt), Some(ter), UncertainLengthChange::None) => {
                    write!(f, "{alt}ext{ter}")
                }
                (Some(alt), Some(ter), UncertainLengthChange::Unknown) => {
                    write!(f, "{alt}ext{ter}?")
                }
                (Some(alt), Some(ter), UncertainLengthChange::Known(count)) => {
                    write!(f, "{alt}ext{ter}{count}")
                }
            },
            ProteinEdit::Subst { alternative } => write!(f, "{alternative}"),
            ProteinEdit::DelIns { alternative } => write!(f, "delins{alternative}"),
            ProteinEdit::Ins { alternative } => write!(f, "ins{alternative}"),
            ProteinEdit::Del => write!(f, "del"),
            ProteinEdit::Dup => write!(f, "dup"),
            ProteinEdit::Ident => write!(f, "="),
        }
    }
}

impl Display for ProtPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.aa, self.number)
    }
}

impl Display for ProtInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.begin)?;
        if self.begin != self.end {
            write!(f, "_{}", self.end)?;
        }
        Ok(())
    }
}

impl Display for ProtLocEdit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtLocEdit::Ordinary { loc, edit } => write!(f, "{loc}{edit}"),
            ProtLocEdit::NoChange => write!(f, "="),
            ProtLocEdit::NoChangeUncertain => write!(f, "(=)"),
            ProtLocEdit::NoProtein => write!(f, "0"),
            ProtLocEdit::NoProteinUncertain => write!(f, "0?"),
        }
    }
}

impl Display for CdsLocEdit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.loc, self.edit)
    }
}

impl Display for CdsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.begin)?;
        if self.begin != self.end {
            write!(f, "_{}", self.end)?;
        }
        Ok(())
    }
}

impl Display for CdsPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.cds_from == CdsFrom::End {
            write!(f, "*")?;
        }

        write!(f, "{}", self.base)?;

        if let Some(offset) = self.offset {
            if offset > 0 {
                write!(f, "+")?;
            }
            write!(f, "{offset}")?;
        }

        Ok(())
    }
}

impl Display for TxLocEdit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.loc, self.edit)
    }
}

impl Display for TxInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.begin)?;
        if self.begin != self.end {
            write!(f, "_{}", self.end)?;
        }
        Ok(())
    }
}

impl Display for TxPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)?;

        if let Some(offset) = self.offset {
            if offset > 0 {
                write!(f, "+")?;
            }
            write!(f, "{offset}")?;
        }

        Ok(())
    }
}

impl Display for RnaLocEdit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.loc, self.edit)
    }
}

impl Display for RnaInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.begin)?;
        if self.begin != self.end {
            write!(f, "_{}", self.end)?;
        }
        Ok(())
    }
}

impl Display for RnaPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)?;

        if let Some(offset) = self.offset {
            if offset > 0 {
                write!(f, "+")?;
            }
            write!(f, "{offset}")?;
        }

        Ok(())
    }
}

impl Display for GenomeLocEdit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.loc, self.edit)
    }
}

impl Display for GenomeInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.begin {
            Some(begin) => write!(f, "{begin}")?,
            None => write!(f, "?")?,
        }
        if self.begin != self.end {
            match self.end {
                Some(end) => write!(f, "_{end}")?,
                None => write!(f, "_?")?,
            }
        }
        Ok(())
    }
}

impl Display for MtLocEdit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.loc, self.edit)
    }
}

impl Display for MtInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.begin {
            Some(begin) => write!(f, "{begin}")?,
            None => write!(f, "?")?,
        }
        if self.begin != self.end {
            match self.end {
                Some(end) => write!(f, "_{end}")?,
                None => write!(f, "_?")?,
            }
        }
        Ok(())
    }
}

impl Display for HgvsVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HgvsVariant::CdsVariant {
                accession,
                gene_symbol,
                loc_edit,
            } => {
                write!(f, "{accession}")?;
                if let Some(gene_symbol) = gene_symbol {
                    write!(f, "({gene_symbol})")?;
                }
                write!(f, ":c.{loc_edit}")
            }
            HgvsVariant::GenomeVariant {
                accession,
                gene_symbol,
                loc_edit,
            } => {
                write!(f, "{accession}")?;
                if let Some(gene_symbol) = gene_symbol {
                    write!(f, "({gene_symbol})")?;
                }
                write!(f, ":g.{loc_edit}")
            }
            HgvsVariant::MtVariant {
                accession,
                gene_symbol,
                loc_edit,
            } => {
                write!(f, "{accession}")?;
                if let Some(gene_symbol) = gene_symbol {
                    write!(f, "({gene_symbol})")?;
                }
                write!(f, ":m.{loc_edit}")
            }
            HgvsVariant::TxVariant {
                accession,
                gene_symbol,
                loc_edit,
            } => {
                write!(f, "{accession}")?;
                if let Some(gene_symbol) = gene_symbol {
                    write!(f, "({gene_symbol})")?;
                }
                write!(f, ":n.{loc_edit}")
            }
            HgvsVariant::ProtVariant {
                accession,
                gene_symbol,
                loc_edit,
            } => {
                write!(f, "{accession}")?;
                if let Some(gene_symbol) = gene_symbol {
                    write!(f, "({gene_symbol})")?;
                }
                write!(f, ":p.{loc_edit}")
            }
            HgvsVariant::RnaVariant {
                accession,
                gene_symbol,
                loc_edit,
            } => {
                write!(f, "{accession}")?;
                if let Some(gene_symbol) = gene_symbol {
                    write!(f, "({gene_symbol})")?;
                }
                write!(f, ":r.{loc_edit}")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        str::FromStr,
    };

    use pretty_assertions::assert_eq;

    use crate::parser::{
        Accession, CdsFrom, CdsInterval, CdsLocEdit, CdsPos, GeneSymbol, GenomeInterval,
        GenomeLocEdit, HgvsVariant, MtInterval, MtLocEdit, Mu, NaEdit, ProtInterval, ProtLocEdit,
        ProtPos, ProteinEdit, RnaInterval, RnaLocEdit, RnaPos, TxInterval, TxLocEdit, TxPos,
        UncertainLengthChange,
    };

    #[test]
    fn mu() {
        assert_eq!(format!("{}", Mu::Certain(42)), "42".to_string());
        assert_eq!(format!("{}", Mu::Uncertain(42)), "(42)".to_string());
    }

    #[test]
    fn gene_symbol() {
        assert_eq!(
            format!(
                "{}",
                GeneSymbol {
                    value: "TTN".to_string()
                }
            ),
            "TTN".to_string()
        );
    }

    #[test]
    fn na_edit_ref_alt() {
        assert_eq!(
            format!(
                "{}",
                NaEdit::RefAlt {
                    reference: "".to_string(),
                    alternative: "".to_string()
                }
            ),
            "=".to_string()
        );

        assert_eq!(
            format!(
                "{}",
                NaEdit::RefAlt {
                    reference: "C".to_string(),
                    alternative: "T".to_string()
                }
            ),
            "C>T".to_string()
        );

        assert_eq!(
            format!(
                "{}",
                NaEdit::RefAlt {
                    reference: "CC".to_string(),
                    alternative: "T".to_string()
                }
            ),
            "delCCinsT".to_string()
        );

        assert_eq!(
            format!(
                "{}",
                NaEdit::RefAlt {
                    reference: "C".to_string(),
                    alternative: "".to_string()
                }
            ),
            "delCins".to_string()
        );

        assert_eq!(
            format!(
                "{}",
                NaEdit::RefAlt {
                    reference: "".to_string(),
                    alternative: "C".to_string()
                }
            ),
            "delinsC".to_string()
        );
    }

    #[test]
    fn na_edit_num_alt() {
        assert_eq!(
            format!(
                "{}",
                NaEdit::NumAlt {
                    count: 0,
                    alternative: "".to_string()
                }
            ),
            "=".to_string()
        );

        assert_eq!(
            format!(
                "{}",
                NaEdit::NumAlt {
                    count: 0,
                    alternative: "T".to_string()
                }
            ),
            "delinsT".to_string()
        );
        assert_eq!(
            format!(
                "{}",
                NaEdit::NumAlt {
                    count: 3,
                    alternative: "".to_string()
                }
            ),
            "del3ins".to_string()
        );

        assert_eq!(
            format!(
                "{}",
                NaEdit::NumAlt {
                    count: 3,
                    alternative: "T".to_string()
                }
            ),
            "del3insT".to_string()
        );
    }

    #[test]
    fn na_edit_del_ref() {
        assert_eq!(
            format!(
                "{}",
                NaEdit::DelRef {
                    reference: "T".to_string()
                }
            ),
            "delT".to_string()
        );
    }

    #[test]
    fn na_edit_del_num() {
        assert_eq!(
            format!("{}", NaEdit::DelNum { count: 3 }),
            "del3".to_string()
        );
    }

    #[test]
    fn na_edit_ins() {
        assert_eq!(
            format!(
                "{}",
                NaEdit::Ins {
                    alternative: "T".to_string()
                }
            ),
            "insT".to_string()
        );
    }

    #[test]
    fn na_edit_dup() {
        assert_eq!(
            format!(
                "{}",
                NaEdit::Dup {
                    reference: "T".to_string()
                }
            ),
            "dupT".to_string()
        );
    }

    #[test]
    fn na_edit_inv_ref() {
        assert_eq!(
            format!(
                "{}",
                NaEdit::InvRef {
                    reference: "T".to_string()
                }
            ),
            "invT".to_string()
        );
    }

    #[test]
    fn na_edit_inv_num() {
        assert_eq!(
            format!("{}", NaEdit::InvNum { count: 3 }),
            "inv3".to_string()
        );
    }

    #[test]
    fn uncertain_length_change() {
        assert_eq!(format!("{}", UncertainLengthChange::None), "".to_string(),);
        assert_eq!(
            format!("{}", UncertainLengthChange::Unknown),
            "?".to_string(),
        );
        assert_eq!(
            format!("{}", UncertainLengthChange::Known(42)),
            "42".to_string(),
        );
    }

    #[test]
    fn accession() {
        assert_eq!(
            format!(
                "{}",
                Accession {
                    value: "TTN".to_string()
                }
            ),
            "TTN".to_string()
        )
    }

    #[test]
    fn protein_edit_fs() {
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Fs {
                    alternative: None,
                    terminal: None,
                    length: UncertainLengthChange::None,
                }
            ),
            "fs".to_string(),
        );
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Fs {
                    alternative: None,
                    terminal: None,
                    length: UncertainLengthChange::Unknown,
                }
            ),
            "fs?".to_string(),
        );
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Fs {
                    alternative: None,
                    terminal: None,
                    length: UncertainLengthChange::Known(42),
                }
            ),
            "fs42".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Fs {
                    alternative: Some("Leu".to_string()),
                    terminal: None,
                    length: UncertainLengthChange::None,
                }
            ),
            "Leufs".to_string(),
        );
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Fs {
                    alternative: Some("Leu".to_string()),
                    terminal: None,
                    length: UncertainLengthChange::Unknown,
                }
            ),
            "Leufs?".to_string(),
        );
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Fs {
                    alternative: Some("Leu".to_string()),
                    terminal: None,
                    length: UncertainLengthChange::Known(42),
                }
            ),
            "Leufs42".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Fs {
                    alternative: None,
                    terminal: Some("Ter".to_string()),
                    length: UncertainLengthChange::None,
                }
            ),
            "fsTer".to_string(),
        );
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Fs {
                    alternative: None,
                    terminal: Some("Ter".to_string()),
                    length: UncertainLengthChange::Unknown,
                }
            ),
            "fsTer?".to_string(),
        );
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Fs {
                    alternative: None,
                    terminal: Some("Ter".to_string()),
                    length: UncertainLengthChange::Known(42),
                }
            ),
            "fsTer42".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Fs {
                    alternative: Some("Leu".to_string()),
                    terminal: Some("Ter".to_string()),
                    length: UncertainLengthChange::None,
                }
            ),
            "LeufsTer".to_string(),
        );
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Fs {
                    alternative: Some("Leu".to_string()),
                    terminal: Some("Ter".to_string()),
                    length: UncertainLengthChange::Unknown,
                }
            ),
            "LeufsTer?".to_string(),
        );
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Fs {
                    alternative: Some("Leu".to_string()),
                    terminal: Some("Ter".to_string()),
                    length: UncertainLengthChange::Known(42),
                }
            ),
            "LeufsTer42".to_string(),
        );
    }

    #[test]
    fn protein_edit_ext() {
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Ext {
                    aa_ext: None,
                    ext_aa: None,
                    change: UncertainLengthChange::None,
                }
            ),
            "ext".to_string(),
        );
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Ext {
                    aa_ext: None,
                    ext_aa: None,
                    change: UncertainLengthChange::Unknown,
                }
            ),
            "ext?".to_string(),
        );
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Ext {
                    aa_ext: None,
                    ext_aa: None,
                    change: UncertainLengthChange::Known(42),
                }
            ),
            "ext42".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Ext {
                    aa_ext: Some("Leu".to_string()),
                    ext_aa: None,
                    change: UncertainLengthChange::None,
                }
            ),
            "Leuext".to_string(),
        );
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Ext {
                    aa_ext: Some("Leu".to_string()),
                    ext_aa: None,
                    change: UncertainLengthChange::Unknown,
                }
            ),
            "Leuext?".to_string(),
        );
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Ext {
                    aa_ext: Some("Leu".to_string()),
                    ext_aa: None,
                    change: UncertainLengthChange::Known(42),
                }
            ),
            "Leuext42".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Ext {
                    aa_ext: None,
                    ext_aa: Some("Thr".to_string()),
                    change: UncertainLengthChange::None,
                }
            ),
            "extThr".to_string(),
        );
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Ext {
                    aa_ext: None,
                    ext_aa: Some("Thr".to_string()),
                    change: UncertainLengthChange::Unknown,
                }
            ),
            "extThr?".to_string(),
        );
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Ext {
                    aa_ext: None,
                    ext_aa: Some("Thr".to_string()),
                    change: UncertainLengthChange::Known(42),
                }
            ),
            "extThr42".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Ext {
                    aa_ext: Some("Leu".to_string()),
                    ext_aa: Some("Thr".to_string()),
                    change: UncertainLengthChange::None,
                }
            ),
            "LeuextThr".to_string(),
        );
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Ext {
                    aa_ext: Some("Leu".to_string()),
                    ext_aa: Some("Thr".to_string()),
                    change: UncertainLengthChange::Unknown,
                }
            ),
            "LeuextThr?".to_string(),
        );
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Ext {
                    aa_ext: Some("Leu".to_string()),
                    ext_aa: Some("Thr".to_string()),
                    change: UncertainLengthChange::Known(42),
                }
            ),
            "LeuextThr42".to_string(),
        );
    }

    #[test]
    fn protein_edit_subst() {
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Subst {
                    alternative: "Leu".to_string()
                }
            ),
            "Leu".to_string(),
        );
    }

    #[test]
    fn protein_edit_del_ins() {
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::DelIns {
                    alternative: "Leu".to_string()
                }
            ),
            "delinsLeu".to_string(),
        );
    }

    #[test]
    fn protein_edit_ins() {
        assert_eq!(
            format!(
                "{}",
                ProteinEdit::Ins {
                    alternative: "Leu".to_string()
                }
            ),
            "insLeu".to_string(),
        );
    }

    #[test]
    fn protein_edit_del() {
        assert_eq!(format!("{}", ProteinEdit::Del), "del".to_string(),);
    }

    #[test]
    fn protein_edit_dup() {
        assert_eq!(format!("{}", ProteinEdit::Dup), "dup".to_string(),);
    }

    #[test]
    fn protein_edit_ident() {
        assert_eq!(format!("{}", ProteinEdit::Ident), "=".to_string(),);
    }

    #[test]
    fn cds_pos() {
        assert_eq!(
            format!(
                "{}",
                CdsPos {
                    base: 42,
                    offset: None,
                    cds_from: CdsFrom::Start,
                }
            ),
            "42".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                CdsPos {
                    base: 42,
                    offset: None,
                    cds_from: CdsFrom::End,
                }
            ),
            "*42".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                CdsPos {
                    base: 42,
                    offset: Some(10),
                    cds_from: CdsFrom::Start,
                }
            ),
            "42+10".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                CdsPos {
                    base: 42,
                    offset: Some(10),
                    cds_from: CdsFrom::End,
                }
            ),
            "*42+10".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                CdsPos {
                    base: 42,
                    offset: Some(-10),
                    cds_from: CdsFrom::Start,
                }
            ),
            "42-10".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                CdsPos {
                    base: 42,
                    offset: Some(-10),
                    cds_from: CdsFrom::End,
                }
            ),
            "*42-10".to_string(),
        );
    }

    #[test]
    fn cds_interval() {
        assert_eq!(
            format!(
                "{}",
                CdsInterval {
                    begin: CdsPos {
                        base: 42,
                        offset: Some(-10),
                        cds_from: CdsFrom::Start,
                    },
                    end: CdsPos {
                        base: 42,
                        offset: Some(10),
                        cds_from: CdsFrom::Start,
                    }
                }
            ),
            "42-10_42+10".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                CdsInterval {
                    begin: CdsPos {
                        base: 42,
                        offset: Some(10),
                        cds_from: CdsFrom::Start,
                    },
                    end: CdsPos {
                        base: 42,
                        offset: Some(10),
                        cds_from: CdsFrom::Start,
                    }
                }
            ),
            "42+10".to_string(),
        );
    }

    #[test]
    fn cds_loc_edit() {
        assert_eq!(
            format!(
                "{}",
                CdsLocEdit {
                    loc: Mu::Certain(CdsInterval {
                        begin: CdsPos {
                            base: 42,
                            offset: Some(-10),
                            cds_from: CdsFrom::Start,
                        },
                        end: CdsPos {
                            base: 42,
                            offset: Some(10),
                            cds_from: CdsFrom::Start,
                        }
                    }),
                    edit: Mu::Certain(NaEdit::RefAlt {
                        reference: "".to_string(),
                        alternative: "".to_string()
                    })
                }
            ),
            "42-10_42+10=".to_string(),
        );
    }

    #[test]
    fn tx_pos() {
        assert_eq!(
            format!(
                "{}",
                TxPos {
                    base: 42,
                    offset: None,
                }
            ),
            "42".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                TxPos {
                    base: 42,
                    offset: Some(10),
                }
            ),
            "42+10".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                TxPos {
                    base: 42,
                    offset: Some(-10),
                }
            ),
            "42-10".to_string(),
        );
    }

    #[test]
    fn tx_interval() {
        assert_eq!(
            format!(
                "{}",
                TxInterval {
                    begin: TxPos {
                        base: 42,
                        offset: Some(-10),
                    },
                    end: TxPos {
                        base: 42,
                        offset: Some(10),
                    }
                }
            ),
            "42-10_42+10".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                TxInterval {
                    begin: TxPos {
                        base: 42,
                        offset: Some(10),
                    },
                    end: TxPos {
                        base: 42,
                        offset: Some(10),
                    }
                }
            ),
            "42+10".to_string(),
        );
    }

    #[test]
    fn tx_loc_edit() {
        assert_eq!(
            format!(
                "{}",
                TxLocEdit {
                    loc: Mu::Certain(TxInterval {
                        begin: TxPos {
                            base: 42,
                            offset: Some(-10),
                        },
                        end: TxPos {
                            base: 42,
                            offset: Some(10),
                        }
                    }),
                    edit: Mu::Certain(NaEdit::RefAlt {
                        reference: "".to_string(),
                        alternative: "".to_string()
                    })
                }
            ),
            "42-10_42+10=".to_string(),
        );
    }
    #[test]
    fn rna_pos() {
        assert_eq!(
            format!(
                "{}",
                RnaPos {
                    base: 42,
                    offset: None,
                }
            ),
            "42".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                RnaPos {
                    base: 42,
                    offset: Some(10),
                }
            ),
            "42+10".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                RnaPos {
                    base: 42,
                    offset: Some(-10),
                }
            ),
            "42-10".to_string(),
        );
    }

    #[test]
    fn rna_interval() {
        assert_eq!(
            format!(
                "{}",
                RnaInterval {
                    begin: RnaPos {
                        base: 42,
                        offset: Some(-10),
                    },
                    end: RnaPos {
                        base: 42,
                        offset: Some(10),
                    }
                }
            ),
            "42-10_42+10".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                RnaInterval {
                    begin: RnaPos {
                        base: 42,
                        offset: Some(10),
                    },
                    end: RnaPos {
                        base: 42,
                        offset: Some(10),
                    }
                }
            ),
            "42+10".to_string(),
        );
    }

    #[test]
    fn rna_loc_edit() {
        assert_eq!(
            format!(
                "{}",
                RnaLocEdit {
                    loc: Mu::Certain(RnaInterval {
                        begin: RnaPos {
                            base: 42,
                            offset: Some(-10),
                        },
                        end: RnaPos {
                            base: 42,
                            offset: Some(10),
                        }
                    }),
                    edit: Mu::Certain(NaEdit::RefAlt {
                        reference: "".to_string(),
                        alternative: "".to_string()
                    })
                }
            ),
            "42-10_42+10=".to_string(),
        );
    }

    #[test]
    fn genome_interval() {
        assert_eq!(
            format!(
                "{}",
                GenomeInterval {
                    begin: None,
                    end: None
                }
            ),
            "?".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                GenomeInterval {
                    begin: Some(10),
                    end: None
                }
            ),
            "10_?".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                GenomeInterval {
                    begin: None,
                    end: Some(10)
                }
            ),
            "?_10".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                GenomeInterval {
                    begin: Some(10),
                    end: Some(20)
                }
            ),
            "10_20".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                GenomeInterval {
                    begin: Some(10),
                    end: Some(10)
                }
            ),
            "10".to_string(),
        );
    }

    #[test]
    fn genome_loc_edit() {
        assert_eq!(
            format!(
                "{}",
                GenomeLocEdit {
                    loc: Mu::Certain(GenomeInterval {
                        begin: Some(10),
                        end: Some(20)
                    }),
                    edit: Mu::Certain(NaEdit::RefAlt {
                        reference: "C".to_string(),
                        alternative: "T".to_string()
                    })
                }
            ),
            "10_20C>T".to_string(),
        );
    }

    #[test]
    fn mt_interval() {
        assert_eq!(
            format!(
                "{}",
                MtInterval {
                    begin: None,
                    end: None
                }
            ),
            "?".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                MtInterval {
                    begin: Some(10),
                    end: None
                }
            ),
            "10_?".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                MtInterval {
                    begin: None,
                    end: Some(10)
                }
            ),
            "?_10".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                MtInterval {
                    begin: Some(10),
                    end: Some(20)
                }
            ),
            "10_20".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                MtInterval {
                    begin: Some(10),
                    end: Some(10)
                }
            ),
            "10".to_string(),
        );
    }

    #[test]
    fn mt_loc_edit() {
        assert_eq!(
            format!(
                "{}",
                MtLocEdit {
                    loc: Mu::Certain(MtInterval {
                        begin: Some(10),
                        end: Some(20)
                    }),
                    edit: Mu::Certain(NaEdit::RefAlt {
                        reference: "C".to_string(),
                        alternative: "T".to_string()
                    })
                }
            ),
            "10_20C>T".to_string(),
        );
    }

    #[test]
    fn prot_pos() {
        assert_eq!(
            format!(
                "{}",
                ProtPos {
                    aa: "Leu".to_string(),
                    number: 42
                }
            ),
            "Leu42".to_string()
        );
    }

    #[test]
    fn prot_interval() {
        assert_eq!(
            format!(
                "{}",
                ProtInterval {
                    begin: ProtPos {
                        aa: "Leu".to_string(),
                        number: 42
                    },
                    end: ProtPos {
                        aa: "Thr".to_string(),
                        number: 43
                    },
                }
            ),
            "Leu42_Thr43".to_string()
        );

        assert_eq!(
            format!(
                "{}",
                ProtInterval {
                    begin: ProtPos {
                        aa: "Leu".to_string(),
                        number: 42
                    },
                    end: ProtPos {
                        aa: "Leu".to_string(),
                        number: 42
                    },
                }
            ),
            "Leu42".to_string()
        );
    }

    #[test]
    fn prot_loc_edit() {
        assert_eq!(
            format!(
                "{}",
                ProtLocEdit::Ordinary {
                    loc: Mu::Certain(ProtInterval {
                        begin: ProtPos {
                            aa: "Leu".to_string(),
                            number: 42
                        },
                        end: ProtPos {
                            aa: "Thr".to_string(),
                            number: 43
                        },
                    },),
                    edit: Mu::Certain(ProteinEdit::Ident),
                }
            ),
            "Leu42_Thr43=".to_string()
        );

        assert_eq!(format!("{}", ProtLocEdit::NoChange,), "=".to_string());

        assert_eq!(
            format!("{}", ProtLocEdit::NoChangeUncertain,),
            "(=)".to_string()
        );

        assert_eq!(format!("{}", ProtLocEdit::NoProtein,), "0".to_string());

        assert_eq!(
            format!("{}", ProtLocEdit::NoProteinUncertain,),
            "0?".to_string()
        );
    }

    #[test]
    fn hgvs_variant_cds() {
        assert_eq!(
            format!(
                "{}",
                HgvsVariant::CdsVariant {
                    accession: Accession {
                        value: "NA12345.1".to_string()
                    },
                    gene_symbol: Some(GeneSymbol {
                        value: "TTN".to_string()
                    }),
                    loc_edit: CdsLocEdit {
                        loc: Mu::Certain(CdsInterval {
                            begin: CdsPos {
                                base: 100,
                                offset: None,
                                cds_from: CdsFrom::Start,
                            },
                            end: CdsPos {
                                base: 100,
                                offset: None,
                                cds_from: CdsFrom::Start,
                            }
                        }),
                        edit: Mu::Certain(NaEdit::RefAlt {
                            reference: "C".to_string(),
                            alternative: "T".to_string()
                        })
                    }
                }
            ),
            "NA12345.1(TTN):c.100C>T".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                HgvsVariant::CdsVariant {
                    accession: Accession {
                        value: "NA12345.1".to_string()
                    },
                    gene_symbol: None,
                    loc_edit: CdsLocEdit {
                        loc: Mu::Certain(CdsInterval {
                            begin: CdsPos {
                                base: 100,
                                offset: None,
                                cds_from: CdsFrom::Start,
                            },
                            end: CdsPos {
                                base: 100,
                                offset: None,
                                cds_from: CdsFrom::Start,
                            }
                        }),
                        edit: Mu::Certain(NaEdit::RefAlt {
                            reference: "C".to_string(),
                            alternative: "T".to_string()
                        })
                    }
                }
            ),
            "NA12345.1:c.100C>T".to_string(),
        );
    }

    #[test]
    fn hgvs_variant_genome() {
        assert_eq!(
            format!(
                "{}",
                HgvsVariant::GenomeVariant {
                    accession: Accession {
                        value: "NA12345.1".to_string()
                    },
                    gene_symbol: Some(GeneSymbol {
                        value: "TTN".to_string()
                    }),
                    loc_edit: GenomeLocEdit {
                        loc: Mu::Certain(GenomeInterval {
                            begin: Some(100),
                            end: Some(100)
                        }),
                        edit: Mu::Certain(NaEdit::RefAlt {
                            reference: "C".to_string(),
                            alternative: "T".to_string()
                        })
                    }
                }
            ),
            "NA12345.1(TTN):g.100C>T".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                HgvsVariant::GenomeVariant {
                    accession: Accession {
                        value: "NA12345.1".to_string()
                    },
                    gene_symbol: None,
                    loc_edit: GenomeLocEdit {
                        loc: Mu::Certain(GenomeInterval {
                            begin: Some(100),
                            end: Some(100)
                        }),
                        edit: Mu::Certain(NaEdit::RefAlt {
                            reference: "C".to_string(),
                            alternative: "T".to_string()
                        })
                    }
                }
            ),
            "NA12345.1:g.100C>T".to_string(),
        );
    }

    #[test]
    fn hgvs_variant_mt() {
        assert_eq!(
            format!(
                "{}",
                HgvsVariant::MtVariant {
                    accession: Accession {
                        value: "NA12345.1".to_string()
                    },
                    gene_symbol: Some(GeneSymbol {
                        value: "TTN".to_string()
                    }),
                    loc_edit: MtLocEdit {
                        loc: Mu::Certain(MtInterval {
                            begin: Some(100),
                            end: Some(100)
                        }),
                        edit: Mu::Certain(NaEdit::RefAlt {
                            reference: "C".to_string(),
                            alternative: "T".to_string()
                        })
                    }
                }
            ),
            "NA12345.1(TTN):m.100C>T".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                HgvsVariant::MtVariant {
                    accession: Accession {
                        value: "NA12345.1".to_string()
                    },
                    gene_symbol: None,
                    loc_edit: MtLocEdit {
                        loc: Mu::Certain(MtInterval {
                            begin: Some(100),
                            end: Some(100)
                        }),
                        edit: Mu::Certain(NaEdit::RefAlt {
                            reference: "C".to_string(),
                            alternative: "T".to_string()
                        })
                    }
                }
            ),
            "NA12345.1:m.100C>T".to_string(),
        );
    }

    #[test]
    fn hgvs_variant_tx() {
        assert_eq!(
            format!(
                "{}",
                HgvsVariant::TxVariant {
                    accession: Accession {
                        value: "NA12345.1".to_string()
                    },
                    gene_symbol: Some(GeneSymbol {
                        value: "TTN".to_string()
                    }),
                    loc_edit: TxLocEdit {
                        loc: Mu::Certain(TxInterval {
                            begin: TxPos {
                                base: 100,
                                offset: None
                            },
                            end: TxPos {
                                base: 100,
                                offset: None
                            },
                        }),
                        edit: Mu::Certain(NaEdit::RefAlt {
                            reference: "C".to_string(),
                            alternative: "T".to_string()
                        })
                    }
                }
            ),
            "NA12345.1(TTN):n.100C>T".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                HgvsVariant::TxVariant {
                    accession: Accession {
                        value: "NA12345.1".to_string()
                    },
                    gene_symbol: None,
                    loc_edit: TxLocEdit {
                        loc: Mu::Certain(TxInterval {
                            begin: TxPos {
                                base: 100,
                                offset: None
                            },
                            end: TxPos {
                                base: 100,
                                offset: None
                            },
                        }),
                        edit: Mu::Certain(NaEdit::RefAlt {
                            reference: "C".to_string(),
                            alternative: "T".to_string()
                        })
                    }
                }
            ),
            "NA12345.1:n.100C>T".to_string(),
        );
    }

    #[test]
    fn hgvs_variant_rna() {
        assert_eq!(
            format!(
                "{}",
                HgvsVariant::RnaVariant {
                    accession: Accession {
                        value: "NA12345.1".to_string()
                    },
                    gene_symbol: Some(GeneSymbol {
                        value: "TTN".to_string()
                    }),
                    loc_edit: RnaLocEdit {
                        loc: Mu::Certain(RnaInterval {
                            begin: RnaPos {
                                base: 100,
                                offset: None
                            },
                            end: RnaPos {
                                base: 100,
                                offset: None
                            },
                        }),
                        edit: Mu::Certain(NaEdit::RefAlt {
                            reference: "C".to_string(),
                            alternative: "T".to_string()
                        })
                    }
                }
            ),
            "NA12345.1(TTN):r.100C>T".to_string(),
        );

        assert_eq!(
            format!(
                "{}",
                HgvsVariant::RnaVariant {
                    accession: Accession {
                        value: "NA12345.1".to_string()
                    },
                    gene_symbol: None,
                    loc_edit: RnaLocEdit {
                        loc: Mu::Certain(RnaInterval {
                            begin: RnaPos {
                                base: 100,
                                offset: None
                            },
                            end: RnaPos {
                                base: 100,
                                offset: None
                            },
                        }),
                        edit: Mu::Certain(NaEdit::RefAlt {
                            reference: "C".to_string(),
                            alternative: "T".to_string()
                        })
                    }
                }
            ),
            "NA12345.1:r.100C>T".to_string(),
        );
    }

    #[test]
    fn hgvs_variant_prot() {
        assert_eq!(
            format!(
                "{}",
                HgvsVariant::ProtVariant {
                    accession: Accession {
                        value: "NA12345.1".to_string()
                    },
                    gene_symbol: Some(GeneSymbol {
                        value: "TTN".to_string()
                    }),
                    loc_edit: ProtLocEdit::NoChange
                }
            ),
            "NA12345.1(TTN):p.=".to_string(),
        );
    }

    // This test uses the "gauntlet" file from the hgvs package for round-tripping.
    #[test]
    fn roundtrip_hgvs_gauntlet() -> Result<(), anyhow::Error> {
        let reader = BufReader::new(File::open("tests/data/gauntlet")?);

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if !line.starts_with('#') && !line.is_empty() {
                let hgvs_variant = HgvsVariant::from_str(line)?;
                let hgvs_str = format!("{}", &hgvs_variant);
                assert_eq!(
                    hgvs_str, line,
                    "round-trip failed for variant {:?}; line= {}",
                    &hgvs_variant, line
                );
            }
        }

        Ok(())
    }
}