use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct AddMovieForm {
    pub(crate) title: String,
}
