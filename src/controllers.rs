use actix_web::{HttpRequest, Responder};

pub async fn happy_new_year(req: HttpRequest) -> impl Responder {
  let name = req.match_info().get("name").unwrap_or("");
  let chars_count = name.len();
  if name.eq_ignore_ascii_case("") {
    format!("Happy New Year!")
  } else {
    format!(
      "Happy new year, person whose name contains {} chars!",
      &chars_count
    )
  }
}
