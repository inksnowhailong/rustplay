use trpl::Html;

fn main() {
    async fn page_title(url:&str) -> Option<String>{
        let response = trpl::get(url).await;
        let response_text =  response.text().await;
        Html::parse(&reponse_text)
        .select_first("title")
        .map(|title_tag| title_tag.inner_html())
    }
}
