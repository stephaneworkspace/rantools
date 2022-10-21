use convert_case::{Case, Casing};
use pulldown_cmark::{Event, Parser, Tag};

pub const LATEX_HEADER: &str = r#"\documentclass{scrartcl}
	\usepackage{graphicx}
	\usepackage{hyperref}
	\usepackage{listings}
	\usepackage{xcolor}
	\definecolor{colKeys}{rgb}{0,0.5,0}
	\definecolor{colIdentifier}{rgb}{0,0,0}
	\definecolor{colComments}{rgb}{0,0.5,1}
	\definecolor{colString}{rgb}{0.6,0.1,0.1}
	\definecolor{colBackground}{rgb}{0.95,0.95,1}
	\lstset{%configuration de listings
	   float=hbp,%
	   basicstyle=\ttfamily\small,%
	   %
	   identifierstyle=\color{colIdentifier}, %
	   keywordstyle=\color{colKeys}, %
	   stringstyle=\color{colString}, %
	   commentstyle=\color{colComments}\textit, %
	   %
	   backgroundcolor=\color{colBackground},%
	   %
	   columns=flexible, %
	   tabsize=2, %
	   frame=trbl, %
	   %frameround=tttt,%
	   extendedchars=true, %
	   showspaces=false, %
	   showstringspaces=false, %
	   numbers=left, %
	   numberstyle=\tiny, %
	   breaklines=true, %
	   breakautoindent=true, %
	   captionpos=b,%
	   xrightmargin=0.2cm, %
	   xleftmargin=0.2cm
	}
	\begin{document}
	"#;

pub const LATEX_FOOTER: &str = "\n\\end{document}\n";

/// Used to keep track of current pulldown_cmark "event".
/// TODO: Is there a native pulldown_cmark method to do this?
#[derive(Debug)]
enum EventType {
    //Code,
    Emphasis,
    Header,
    //Html,
    Strong,
    Table,
    TableHead,
    Text,
}

struct CurrentType {
    event_type: EventType,
}

/**
 * Part of this function is Copyright Liam Beckman <liam@liambeckman.com> (license: MPL-2.0)
 * Source: https://github.com/lbeckman314/md2tex/blob/25fa878ccce122c224c24659ee1c1dd30c8a5d51/src/lib.rs
 *
 */
pub fn markdown_to_latex(markdown: String) -> String {
    let mut output = String::from(LATEX_HEADER);

    let parser = Parser::new(&markdown);

    let mut header_value = String::new();

    let mut current: CurrentType = CurrentType {
        event_type: EventType::Text,
    };
    let mut cells = 0;

    let mut equation_mode = false;
    let mut buffer = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Header(level)) => {
                current.event_type = EventType::Header;
                output.push_str("\n");
                output.push_str("\\");
                match level {
                    -1 => output.push_str("part{"),
                    0 => output.push_str("chapter{"),
                    1 => output.push_str("section{"),
                    2 => output.push_str("subsection{"),
                    3 => output.push_str("subsubsection{"),
                    4 => output.push_str("paragraph{"),
                    5 => output.push_str("subparagraph{"),
                    _ => eprintln!("header is out of range."),
                }
            }
            Event::End(Tag::Header(_)) => {
                output.push_str("}\n");
                output.push_str("\\");
                output.push_str("label{");
                output.push_str(&header_value);
                output.push_str("}\n");

                output.push_str("\\");
                output.push_str("label{");
                output.push_str(&header_value.to_case(Case::Kebab));
                output.push_str("}\n");
            }
            Event::Start(Tag::Emphasis) => {
                current.event_type = EventType::Emphasis;
                output.push_str("\\emph{");
            }
            Event::End(Tag::Emphasis) => output.push_str("}"),

            Event::Start(Tag::Strong) => {
                current.event_type = EventType::Strong;
                output.push_str("\\textbf{");
            }
            Event::End(Tag::Strong) => output.push_str("}"),

            Event::Start(Tag::List(None)) => output.push_str("\\begin{itemize}\n"),
            Event::End(Tag::List(None)) => output.push_str("\\end{itemize}\n"),

            Event::Start(Tag::List(Some(_))) => output.push_str("\\begin{enumerate}\n"),
            Event::End(Tag::List(Some(_))) => output.push_str("\\end{enumerate}\n"),

            Event::Start(Tag::Paragraph) => {
                output.push_str("\n");
            }

            Event::End(Tag::Paragraph) => {
                // ~ adds a space to prevent
                // "There's no line here to end" error on empty lines.
                output.push_str(r"~\\");
                output.push_str("\n");
            }

            Event::Start(Tag::Link(_, url, _)) => {
                output.push_str("\\href{");
                output.push_str(&*url);
                output.push_str("}{");
            }

            Event::End(Tag::Link(_, _, _)) => {
                output.push_str("}");
            }

            Event::Start(Tag::Table(_)) => {
                current.event_type = EventType::Table;
                let table_start = vec![
                    "\n",
                    r"\begingroup",
                    r"\setlength{\LTleft}{-20cm plus -1fill}",
                    r"\setlength{\LTright}{\LTleft}",
                    r"\begin{longtable}{!!!}",
                    r"\hline",
                    r"\hline",
                    "\n",
                ];
                for element in table_start {
                    output.push_str(element);
                    output.push_str("\n");
                }
            }

            Event::Start(Tag::TableHead) => {
                current.event_type = EventType::TableHead;
            }

            Event::End(Tag::TableHead) => {
                output.truncate(output.len() - 2);
                output.push_str(r"\\");
                output.push_str("\n");

                output.push_str(r"\hline");
                output.push_str("\n");

                // we presume that a table follows every table head.
                current.event_type = EventType::Table;
            }

            Event::End(Tag::Table(_)) => {
                let table_end = vec![
                    r"\arrayrulecolor{black}\hline",
                    r"\end{longtable}",
                    r"\endgroup",
                    "\n",
                ];

                for element in table_end {
                    output.push_str(element);
                    output.push_str("\n");
                }

                let mut cols = String::new();
                for _i in 0..cells {
                    cols.push_str(&format!(
                        r"C{{{width}\textwidth}} ",
                        width = 1. / cells as f64
                    ));
                }
                output = output.replace("!!!", &cols);
                cells = 0;
                current.event_type = EventType::Text;
            }

            Event::Start(Tag::TableCell) => match current.event_type {
                EventType::TableHead => {
                    output.push_str(r"\bfseries{");
                }
                _ => (),
            },

            Event::End(Tag::TableCell) => {
                match current.event_type {
                    EventType::TableHead => {
                        output.push_str(r"}");
                        cells += 1;
                    }
                    _ => (),
                }

                output.push_str(" & ");
            }

            Event::Start(Tag::TableRow) => {
                current.event_type = EventType::Table;
            }

            Event::End(Tag::TableRow) => {
                output.truncate(output.len() - 2);
                output.push_str(r"\\");
                output.push_str(r"\arrayrulecolor{lightgray}\hline");
                output.push_str("\n");
            }

            Event::Start(Tag::Image(_, path, title)) => {
                output.push_str("\\begin{figure}\n");
                output.push_str("\\centering\n");
                output.push_str("\\includegraphics[width=\\textwidth]{");
                output.push_str(&*path);
                output.push_str("}\n");
                output.push_str("\\caption{");
                output.push_str(&*title);
                output.push_str("}\n\\end{figure}\n");
            }

            Event::Start(Tag::Item) => output.push_str("\\item "),
            Event::End(Tag::Item) => output.push_str("\n"),

            Event::Start(Tag::CodeBlock(lang)) => {
                if !lang.is_empty() {
                    output.push_str("\\begin{lstlisting}[language=");
                    output.push_str(&*lang);
                    output.push_str("]\n");
                } else {
                    output.push_str("\\begin{lstlisting}\n");
                }
            }

            Event::End(Tag::CodeBlock(_)) => {
                output.push_str("\n\\end{lstlisting}\n");
                current.event_type = EventType::Text;
            }

            Event::Code(t) => {
                output.push_str("\\lstinline|");
                match current.event_type {
                    EventType::Header => output
                        .push_str(&*t.replace("#", r"\#").replace("…", "...").replace("З", "3")),
                    _ => output
                        .push_str(&*t.replace("…", "...").replace("З", "3").replace("<22>", r"\<5C>")),
                }
                output.push_str("|");
            }

            Event::Text(t) => {
                // if "\(" or "\[" are encountered, then begin equation
                // and don't replace any characters.
                let delim_start = vec![r"\(", r"\["];
                let delim_end = vec![r"\)", r"\]"];

                if buffer.len() > 100 {
                    buffer.clear();
                }

                buffer.push_str(&t.clone().into_string());

                match current.event_type {
                    EventType::Strong
                    | EventType::Emphasis
                    | EventType::Text
                    | EventType::Header
                    | EventType::Table => {
                        // TODO more elegant way to do ordered `replace`s (structs?).
                        if delim_start
                            .into_iter()
                            .any(|element| buffer.contains(element))
                        {
                            let popped = output.pop().unwrap();
                            if popped != '\\' {
                                output.push(popped);
                            }
                            output.push_str(&*t);
                            equation_mode = true;
                        } else if delim_end
                            .into_iter()
                            .any(|element| buffer.contains(element))
                            || equation_mode == true
                        {
                            let popped = output.pop().unwrap();
                            if popped != '\\' {
                                output.push(popped);
                            }
                            output.push_str(&*t);
                            equation_mode = false;
                        } else {
                            output.push_str(
                                &*t.replace(r"\", r"\\")
                                    .replace("&", r"\&")
                                    .replace(r"\s", r"\textbackslash{}s")
                                    .replace(r"\w", r"\textbackslash{}w")
                                    .replace("_", r"\_")
                                    .replace(r"\<", "<")
                                    .replace(r"%", r"\%")
                                    .replace(r"$", r"\$")
                                    .replace(r"—", "---")
                                    .replace("#", r"\#"),
                            );
                        }
                        header_value = t.into_string();
                    }
                    _ => output.push_str(&*t),
                }
            }

            Event::SoftBreak => {
                output.push('\n');
            }

            Event::HardBreak => {
                output.push_str(r"\\");
                output.push('\n');
            }

            _ => (),
        }
    }

    output.push_str(LATEX_FOOTER);

    output
}

pub fn markdown_to_pdf(markdown: String) -> Result<Vec<u8>, tectonic::Error> {
    tectonic::latex_to_pdf(markdown_to_latex(markdown))
}

#[cfg(test)]
mod tests {
    use super::{markdown_to_latex, markdown_to_pdf};
    use lopdf::Document;
    use pretty_assertions::assert_eq;
    use std::io::Cursor;

    const MARKDOWN_IN: &str = r#"# First title
	Some content
	## Second level
	Text
	[link](https://example.com)
	**Bold**
	__Italic__

	some code:
	```sh
	sudo make-it-work
	```
	issue [#12345](https://example.com)
	"#;
    const LATEXT_OUT: &str = r#"\documentclass{scrartcl}
	\usepackage{graphicx}
	\usepackage{hyperref}
	\usepackage{listings}
	\usepackage{xcolor}
	\definecolor{colKeys}{rgb}{0,0.5,0}
	\definecolor{colIdentifier}{rgb}{0,0,0}
	\definecolor{colComments}{rgb}{0,0.5,1}
	\definecolor{colString}{rgb}{0.6,0.1,0.1}
	\definecolor{colBackground}{rgb}{0.95,0.95,1}
	\lstset{%configuration de listings
	   float=hbp,%
	   basicstyle=\ttfamily\small,%
	   %
	   identifierstyle=\color{colIdentifier}, %
	   keywordstyle=\color{colKeys}, %
	   stringstyle=\color{colString}, %
	   commentstyle=\color{colComments}\textit, %
	   %
	   backgroundcolor=\color{colBackground},%
	   %
	   columns=flexible, %
	   tabsize=2, %
	   frame=trbl, %
	   %frameround=tttt,%
	   extendedchars=true, %
	   showspaces=false, %
	   showstringspaces=false, %
	   numbers=left, %
	   numberstyle=\tiny, %
	   breaklines=true, %
	   breakautoindent=true, %
	   captionpos=b,%
	   xrightmargin=0.2cm, %
	   xleftmargin=0.2cm
	}
	\begin{document}

	\section{First title}
	\label{First title}
	\label{first-title}

	Some content~\\

	\subsection{Second level}
	\label{Second level}
	\label{second-level}

	Text
	\href{https://example.com}{link}
	\textbf{Bold}
	\textbf{Italic}~\\

	some code:~\\
	\begin{lstlisting}[language=sh]
	sudo make-it-work

	\end{lstlisting}

	issue \href{https://example.com}{\#12345}~\\

	\end{document}
	"#;

    #[test]
    fn test_md_to_latex() {
        let output = markdown_to_latex(MARKDOWN_IN.to_string());
        assert_eq!(LATEXT_OUT, output);
    }

    #[test]
    fn test_latex_to_pdf() {
        let output = markdown_to_pdf(MARKDOWN_IN.to_string());

        match output {
            Ok(data) => {
                let mut file = Cursor::new(data);
                match Document::load_from(&mut file) {
                    Ok(doc) => {
                        assert_eq!("1.5", doc.version);
                    }
                    Err(_) => assert!(true),
                }
            }
            Err(_) => assert!(true),
        }
    }
}