use crate::Movie;
use maud::{DOCTYPE, Markup, html};

pub(crate) fn render_page(movies: &[Movie]) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Movie Watch List" }
                style {
                    r#"
                    body { font-family: system-ui, sans-serif; margin: 2rem auto; max-width: 42rem; padding: 0 1rem; }
                    form.inline { display: inline; margin-right: 0.75rem; }
                    ul { padding: 0; }
                    li { list-style: none; margin: 0.75rem 0; }
                    .seen { color: #555; text-decoration: line-through; }
                    input, button { font: inherit; }
                    input { padding: 0.45rem; width: 70%; max-width: 20rem; }
                    button { padding: 0.45rem 0.75rem; }
                    "#
                }
            }
            body {
                h1 { "Movie Watch List" }
                p { "Track movies you want to watch and mark them as seen." }
                form method="post" action="/movies" {
                    label for="title" { "Add a movie" }
                    br;
                    input id="title" name="title" placeholder="The Iron Giant" required;
                    button type="submit" { "Save" }
                }

                @if movies.is_empty() {
                    p { "No movies saved yet." }
                } @else {
                    ul {
                        @for movie in movies {
                            li {
                                form.inline method="post" action={ "/movies/" (movie.id()) "/toggle" } {
                                    button type="submit" {
                                        @if movie.seen() {
                                            "Mark unseen"
                                        } @else {
                                            "Mark seen"
                                        }
                                    }
                                }
                                span class={(if movie.seen() { "seen" } else { "pending" })} {
                                    (movie.title())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
