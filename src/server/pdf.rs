// src/server/pdf.rs
//! PDF generation via LaTeX → tectonic.
//!
//! Flow:
//!   1. `render_latex` builds the .tex source string from live PortfolioData.
//!   2. `generate_pdf` writes the .tex (and optional avatar image) to a temp
//!      directory, runs `tectonic`, reads back the .pdf bytes.
//!
//! The module is SSR-only (`#[cfg(feature = "ssr")]` is applied at the call-site
//! in mod.rs).  All types here are std / ssr-only crates.

use std::io::Write;
use std::process::Command;

use crate::models::*;

// ─── Public error type ────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum PdfError {
    Io(std::io::Error),
    Tectonic(String),
    Http(String),
}

impl std::fmt::Display for PdfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdfError::Io(e) => write!(f, "IO error: {e}"),
            PdfError::Tectonic(s) => write!(f, "tectonic error: {s}"),
            PdfError::Http(s) => write!(f, "HTTP error fetching avatar: {s}"),
        }
    }
}

impl From<std::io::Error> for PdfError {
    fn from(e: std::io::Error) -> Self {
        PdfError::Io(e)
    }
}

// ─── Public API ───────────────────────────────────────────────────────────────

/// Compile `data` into a PDF resume and return the raw bytes.
pub async fn generate_pdf(data: &PortfolioData) -> Result<Vec<u8>, PdfError> {
    let tmp = tempfile::TempDir::new()?;
    let tmp_path = tmp.path();

    // --- optionally download the avatar so LaTeX can embed it ----------------
    let avatar_path = if !data.profile.avatar_url.is_empty() {
        match download_avatar(&data.profile.avatar_url, tmp_path).await {
            Ok(p) => Some(p),
            Err(e) => {
                tracing::warn!("Could not fetch avatar for PDF: {e}");
                None
            }
        }
    } else {
        None
    };

    // --- write the .tex file -------------------------------------------------
    let tex_src = render_latex(data, avatar_path.as_deref());
    let tex_path = tmp_path.join("resume.tex");
    {
        let mut f = std::fs::File::create(&tex_path)?;
        f.write_all(tex_src.as_bytes())?;
    }

    // --- run tectonic with pdflatex fallback ────────────────────────────────
    let mut command = Command::new("tectonic");
    command.arg("--outdir").arg(tmp_path).arg(&tex_path);

    let output = match command.output() {
        Ok(out) => Ok(out),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            tracing::info!("tectonic not found in PATH, trying pdflatex as fallback...");
            Command::new("pdflatex")
                .arg("-interaction=nonstopmode")
                .arg("-output-directory")
                .arg(tmp_path)
                .arg(&tex_path)
                .output()
                .map_err(|pe| {
                    PdfError::Tectonic(format!(
                        "Neither 'tectonic' nor 'pdflatex' could be found in your PATH.\n\
                         Please install one of them to enable PDF generation.\n\
                         Tectonic error: {e}\n\
                         pdflatex error: {pe}"
                    ))
                })
        }
        Err(e) => Err(PdfError::Tectonic(format!("failed to spawn tectonic: {e}"))),
    }?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        return Err(PdfError::Tectonic(format!(
            "LaTeX compiler exited with status {:?}\nstdout:\n{stdout}\nstderr:\n{stderr}",
            output.status.code()
        )));
    }

    // --- read the PDF --------------------------------------------------------
    let pdf_path = tmp_path.join("resume.pdf");
    let bytes = std::fs::read(&pdf_path)?;
    Ok(bytes)
    // `tmp` is dropped here → temp directory is deleted automatically
}

// ─── Avatar downloader ───────────────────────────────────────────────────────

async fn download_avatar(url: &str, dir: &std::path::Path) -> Result<std::path::PathBuf, PdfError> {
    // Detect extension from URL (default to .jpg)
    let ext = url
        .rsplit('.')
        .next()
        .and_then(|e| {
            let e = e.split('?').next().unwrap_or(e);
            if ["jpg", "jpeg", "png", "gif", "webp"].contains(&e) {
                Some(e)
            } else {
                None
            }
        })
        .unwrap_or("jpg");

    let dest = dir.join(format!("avatar.{ext}"));

    // Use reqwest if available, otherwise fall back to curl
    #[cfg(feature = "ssr")]
    {
        // We use a simple blocking approach via std process so we don't need
        // to add reqwest as a dependency.
        let status = Command::new("curl")
            .args(["--silent", "--max-time", "8", "--output"])
            .arg(&dest)
            .arg(url)
            .status()
            .map_err(|e| PdfError::Http(e.to_string()))?;

        if status.success() && dest.exists() {
            return Ok(dest);
        }
    }

    Err(PdfError::Http("curl failed or file not created".into()))
}

// ─── LaTeX escaping ──────────────────────────────────────────────────────────

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for ch in s.chars() {
        match ch {
            '&' => out.push_str(r"\&"),
            '%' => out.push_str(r"\%"),
            '$' => out.push_str(r"\$"),
            '#' => out.push_str(r"\#"),
            '_' => out.push_str(r"\_"),
            '{' => out.push_str(r"\{"),
            '}' => out.push_str(r"\}"),
            '~' => out.push_str(r"\textasciitilde{}"),
            '^' => out.push_str(r"\textasciicircum{}"),
            '\\' => out.push_str(r"\textbackslash{}"),
            '<' => out.push_str(r"\textless{}"),
            '>' => out.push_str(r"\textgreater{}"),
            c => out.push(c),
        }
    }
    out
}

fn esc(s: &str) -> String {
    escape(s)
}

#[derive(Clone, Copy)]
enum ResumeSection {
    Experience,
    Projects,
    Skills,
    Education,
    Certifications,
}

impl ResumeSection {
    fn from_slug(slug: &str) -> Option<Self> {
        match slug {
            "experience" => Some(Self::Experience),
            "projects" => Some(Self::Projects),
            "skills" => Some(Self::Skills),
            "education" => Some(Self::Education),
            "certifications" => Some(Self::Certifications),
            _ => None,
        }
    }

    fn default_order() -> [Self; 5] {
        [
            Self::Experience,
            Self::Projects,
            Self::Skills,
            Self::Education,
            Self::Certifications,
        ]
    }
}

fn ordered_resume_sections(section_order: &str) -> Vec<ResumeSection> {
    let mut sections: Vec<ResumeSection> = section_order
        .split(',')
        .filter_map(|section| ResumeSection::from_slug(section.trim()))
        .collect();

    if sections.is_empty() {
        sections.extend(ResumeSection::default_order());
    }

    sections
}

// ─── LaTeX template ──────────────────────────────────────────────────────────

/// Build the complete .tex source for the resume.
pub fn render_latex(data: &PortfolioData, avatar_path: Option<&std::path::Path>) -> String {
    let p = &data.profile;
    let mut doc = String::new();

    // ── Preamble ─────────────────────────────────────────────────────────────
    doc.push_str(
        r#"\documentclass[10pt,a4paper]{article}

% ---- packages ----------------------------------------------------------------
\usepackage[T1]{fontenc}
\usepackage[utf8]{inputenc}
\usepackage{lmodern}
\usepackage[a4paper, top=1.8cm, bottom=1.8cm, left=1.8cm, right=1.8cm]{geometry}
\usepackage{xcolor}
\usepackage{graphicx}
\usepackage{tikz}
\usepackage{tabularx}
\usepackage{enumitem}
\usepackage{hyperref}
\usepackage{microtype}
\usepackage{array}
\usepackage{calc}

% ---- colours -----------------------------------------------------------------
\definecolor{accent}{HTML}{0F766E}      % premium teal-700
\definecolor{accentdk}{HTML}{115E59}    % teal-800
\definecolor{textPrimary}{HTML}{1E293B}  % slate-800
\definecolor{textSecondary}{HTML}{475569}% slate-600
\definecolor{textMuted}{HTML}{64748B}    % slate-500
\definecolor{ruleclr}{HTML}{E2E8F0}     % slate-200
\definecolor{tagBg}{HTML}{F0FDFA}       % teal-50
\definecolor{tagText}{HTML}{0F766E}     % teal-700

% ---- hyperref ----------------------------------------------------------------
\hypersetup{
  colorlinks=true,
  urlcolor=accentdk,
  pdfauthor={}"#,
    );
    doc.push_str(&format!(
        ",\n  pdftitle={{{} --- Resume}}\n}}",
        esc(&p.name)
    ));

    doc.push_str(
        r#"

% ---- misc helpers ------------------------------------------------------------
\setlength{\parindent}{0pt}
\pagestyle{empty}

% ---- tag box command (with proper padding) -----------------------------------
\newcommand{\tag}[1]{%
  \colorbox{tagBg}{\textcolor{tagText}{\fontsize{7.5}{9.5}\selectfont\sffamily #1}}%
}

% ---- section heading ---------------------------------------------------------
\newcommand{\resumesection}[1]{%
  \vspace{10pt}%
  {\color{accent}\fontsize{9}{11}\selectfont\bfseries\sffamily\MakeUppercase{#1}}%
  \vspace{4pt}%
  {\color{ruleclr}\hrule height 0.8pt}%
  \vspace{6pt}%
}

% ---- bullet list style -------------------------------------------------------
\setlist[itemize]{
  leftmargin=1.2em,
  topsep=2pt,
  itemsep=2pt,
  parsep=0pt,
  label={\color{accent}\scriptsize$\bullet$}
}

\begin{document}

% =============================================================================
%  HEADER (Modern Clean Top)
% =============================================================================
\color{textPrimary}
"#,
    );

    // Header layout: minipages for avatar and contact info
    doc.push_str("\\begin{minipage}[c]{\\textwidth}\n");
    if let Some(av) = avatar_path {
        let av_str = av.to_string_lossy().replace('\\', "/");
        doc.push_str(&format!(
            r#"  \begin{{minipage}}[c]{{2.2cm}}
    \begin{{tikzpicture}}
      \clip (0,0) circle (1.0cm);
      \node at (0,0) {{\includegraphics[width=2.0cm,height=2.0cm,keepaspectratio]{{{av_str}}}}};
    \end{{tikzpicture}}
  \end{{minipage}}\hfill%
  \begin{{minipage}}[c]{{\dimexpr\textwidth-2.5cm}}
"#,
            av_str = av_str
        ));
    } else {
        doc.push_str("  \\begin{minipage}[c]{\\textwidth}\n");
    }

    doc.push_str(&format!(
        r#"    {{\fontsize{{24}}{{28}}\selectfont\bfseries\sffamily\color{{textPrimary}} {name}}}\\[4pt]
    {{\fontsize{{11}}{{14}}\selectfont\sffamily\color{{accent}}\bfseries {title}}}\\[6pt]
"#,
        name = esc(&p.name),
        title = esc(&p.title)
    ));

    // Build contact elements
    let mut contact_parts: Vec<String> = Vec::new();
    if !p.email.is_empty() {
        contact_parts.push(format!(
            r"\href{{mailto:{e}}}{{\sffamily {e}}}",
            e = esc(&p.email)
        ));
    }
    if !p.location.is_empty() {
        contact_parts.push(format!(r"{{\sffamily {loc}}}", loc = esc(&p.location)));
    }
    if !p.github.is_empty() {
        // Clean URL to display a nice label
        let display_github = p
            .github
            .trim_start_matches("https://")
            .trim_start_matches("http://");
        contact_parts.push(format!(
            r"\href{{{url}}}{{\sffamily {disp}}}",
            url = p.github,
            disp = esc(display_github)
        ));
    }
    if !p.linkedin.is_empty() {
        let display_li = p
            .linkedin
            .trim_start_matches("https://")
            .trim_start_matches("http://");
        contact_parts.push(format!(
            r"\href{{{url}}}{{\sffamily {disp}}}",
            url = p.linkedin,
            disp = esc(display_li)
        ));
    }

    let contact_line = contact_parts.join(r" ~~\textbullet~~ ");
    doc.push_str(&format!(
        "    {{\\fontsize{{8.5}}{{11}}\\selectfont\\color{{textSecondary}} {contact_line}}}\n"
    ));
    doc.push_str("  \\end{minipage}\n");
    doc.push_str("\\end{minipage}\n");

    // Bio / Summary block
    if !p.summary.is_empty() || !p.bio.is_empty() {
        let text = if !p.summary.is_empty() {
            &p.summary
        } else {
            &p.bio
        };
        doc.push_str(&format!(
            r#"\vspace{{10pt}}
{{\fontsize{{9}}{{13.5}}\selectfont\color{{textSecondary}} {text}}}
"#,
            text = esc(text)
        ));
    }

    doc.push_str(
        r#"\vspace{10pt}
"#,
    );

    for section in ordered_resume_sections(&p.section_order) {
        match section {
            ResumeSection::Experience => render_experience_section(&mut doc, data),
            ResumeSection::Projects => render_projects_section(&mut doc, data),
            ResumeSection::Skills => render_skills_section(&mut doc, data),
            ResumeSection::Education => render_education_section(&mut doc, data),
            ResumeSection::Certifications => render_certifications_section(&mut doc, data),
        }
    }

    doc.push_str("\\end{document}\n");
    doc
}

fn render_experience_section(doc: &mut String, data: &PortfolioData) {
    // ── Experience ───────────────────────────────────────────────────────────
    if !data.experiences.is_empty() {
        doc.push_str(r"\resumesection{Experience}");
        doc.push('\n');
        for exp in &data.experiences {
            let date_range = if exp.current {
                format!("{} -- Present", esc(&exp.start_date))
            } else {
                let end = exp
                    .end_date
                    .as_deref()
                    .filter(|s| !s.is_empty())
                    .map(esc)
                    .unwrap_or_default();
                format!("{} -- {}", esc(&exp.start_date), end)
            };

            doc.push_str(&format!(
                r#"\noindent\textbf{{\fontsize{{10}}{{12}}\selectfont\color{{textPrimary}}{role}}} \hfill {{\fontsize{{8.5}}{{10}}\selectfont\color{{textMuted}}\sffamily {date}}}\\
\noindent{{\fontsize{{9.5}}{{11}}\selectfont\color{{accent}}\bfseries {company}}}\\
"#,
                role    = esc(&exp.role),
                date    = date_range,
                company = esc(&exp.company),
            ));

            if !exp.description.is_empty() {
                doc.push_str(
                    "\\begin{itemize}[leftmargin=1.2em, topsep=2pt, itemsep=2pt, parsep=0pt]\n",
                );
                for bullet in &exp.description {
                    doc.push_str(&format!("  \\item {{\\fontsize{{9}}{{12.5}}\\selectfont\\color{{textSecondary}} {}}}\n", esc(bullet)));
                }
                doc.push_str("\\end{itemize}\n");
            }

            if !exp.technologies.is_empty() {
                doc.push_str("\\vspace{3pt}\n\\noindent ");
                for tech in &exp.technologies {
                    doc.push_str(&format!("\\tag{{{}}} ", esc(tech)));
                }
                doc.push_str("\n");
            }

            doc.push_str("\\vspace{8pt}\n\n");
        }
    }
}

fn render_education_section(doc: &mut String, data: &PortfolioData) {
    // ── Education ────────────────────────────────────────────────────────────
    if !data.educations.is_empty() {
        doc.push_str(r"\resumesection{Education}");
        doc.push('\n');
        for edu in &data.educations {
            let years = if edu.current {
                format!("{} -- Present", esc(&edu.start_year))
            } else {
                let end = edu
                    .end_year
                    .as_deref()
                    .filter(|s| !s.is_empty())
                    .map(esc)
                    .unwrap_or_default();
                format!("{} -- {}", esc(&edu.start_year), end)
            };

            doc.push_str(&format!(
                r#"\noindent\textbf{{\fontsize{{10}}{{12}}\selectfont\color{{textPrimary}}{degree} in {field}}} \hfill {{\fontsize{{8.5}}{{10}}\selectfont\color{{textMuted}}\sffamily {years}}}\\
\noindent{{\fontsize{{9.5}}{{11}}\selectfont\color{{accent}}\bfseries {inst}}}\\
"#,
                degree = esc(&edu.degree),
                field  = esc(&edu.field),
                years  = years,
                inst   = esc(&edu.institution),
            ));

            if !edu.description.is_empty() {
                doc.push_str(&format!(
                    "\\noindent{{\\fontsize{{9}}{{12.5}}\\selectfont\\color{{textSecondary}} {}}}\\\\\n",
                    esc(&edu.description)
                ));
            }
            if let Some(gpa) = &edu.gpa {
                if !gpa.is_empty() {
                    doc.push_str(&format!(
                        "\\noindent{{\\fontsize{{8.5}}{{10}}\\selectfont\\color{{textMuted}} GPA: {}}}\\\\\n",
                        esc(gpa)
                    ));
                }
            }
            doc.push_str("\\vspace{8pt}\n\n");
        }
    }
}

fn render_skills_section(doc: &mut String, data: &PortfolioData) {
    // ── Skills ───────────────────────────────────────────────────────────────
    if !data.skills.is_empty() {
        doc.push_str(r"\resumesection{Skills}");
        doc.push('\n');

        // Group by category
        let mut cats: std::collections::BTreeMap<&str, Vec<&Skill>> =
            std::collections::BTreeMap::new();
        for s in &data.skills {
            cats.entry(&s.category).or_default().push(s);
        }

        doc.push_str("\\noindent\\begin{tabularx}{\\linewidth}{@{}>{\\color{textPrimary}\\fontsize{9}{11}\\selectfont\\bfseries\\sffamily}p{3.2cm}X@{}}\n");
        for (cat, skills) in &cats {
            let names: Vec<String> = skills.iter().map(|s| esc(&s.name)).collect();
            doc.push_str(&format!(
                "  {} & {{\\fontsize{{9}}{{12}}\\selectfont\\color{{textSecondary}} {}}} \\\\\n",
                esc(cat),
                names.join(", ")
            ));
        }
        doc.push_str("\\end{tabularx}\n\\vspace{6pt}\n\n");
    }
}

fn render_certifications_section(doc: &mut String, data: &PortfolioData) {
    // ── Certifications ───────────────────────────────────────────────────────
    if !data.certifications.is_empty() {
        doc.push_str(r"\resumesection{Certifications}");
        doc.push('\n');

        for cert in &data.certifications {
            let name_part = if let Some(url) = &cert.credential_url {
                format!(
                    r"\href{{{url}}}{{\textbf{{{name}}}}} \tiny$\nearrow$",
                    url = url,
                    name = esc(&cert.name)
                )
            } else {
                format!(r"\textbf{{{}}}", esc(&cert.name))
            };

            doc.push_str(&format!(
                "\\noindent{{\\fontsize{{9.5}}{{11.5}}\\selectfont {name_part}}} --- {{\\fontsize{{9}}{{11}}\\selectfont\\color{{accent}}\\bfseries {issuer}}} \
                \\hfill {{\\fontsize{{8.5}}{{10}}\\selectfont\\color{{textMuted}}\\sffamily {date}}}\\\\\n",
                issuer = esc(&cert.issuer),
                date   = esc(&cert.date),
            ));
        }
        doc.push_str("\\vspace{6pt}\n\n");
    }
}

fn render_projects_section(doc: &mut String, data: &PortfolioData) {
    // ── Featured Projects ────────────────────────────────────────────────────
    let featured_projects: Vec<&Project> = data
        .projects
        .iter()
        .filter(|p| p.featured)
        .take(3)
        .collect();

    if !featured_projects.is_empty() {
        doc.push_str(r"\resumesection{Selected Projects}");
        doc.push('\n');
        for proj in featured_projects {
            let title_part = match (&proj.github_url, &proj.live_url) {
                (Some(gh), _) => format!(
                    r"\href{{{gh}}}{{\textbf{{{}}}}} \tiny$\nearrow$",
                    esc(&proj.title)
                ),
                (_, Some(lv)) => format!(
                    r"\href{{{lv}}}{{\textbf{{{}}}}} \tiny$\nearrow$",
                    esc(&proj.title)
                ),
                _ => format!(r"\textbf{{{}}}", esc(&proj.title)),
            };

            let desc = if !proj.long_description.is_empty() {
                &proj.long_description
            } else {
                &proj.description
            };

            doc.push_str(&format!(
                "\\noindent{{\\fontsize{{9.5}}{{11.5}}\\selectfont {title_part}}}\\\\\n\
                 \\noindent{{\\fontsize{{9}}{{12.5}}\\selectfont\\color{{textSecondary}} {desc}}}\\\\\n",
                desc = esc(desc)
            ));

            if !proj.technologies.is_empty() {
                doc.push_str("\\vspace{3pt}\n\\noindent ");
                for tech in &proj.technologies {
                    doc.push_str(&format!("\\tag{{{}}} ", esc(tech)));
                }
                doc.push('\n');
            }
            doc.push_str("\\vspace{8pt}\n\n");
        }
    }
}
