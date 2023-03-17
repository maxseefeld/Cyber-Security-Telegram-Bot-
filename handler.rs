use std::sync::Arc;
use teloxide::prelude::*;
use crate::openai::get_openai_response;

#[teloxide(subtransition)]
async fn my_handler(cx: UpdateWithCx<Message>, next: Arc<NextMiddleware>) {
    let response = get_openai_response(&cx.update.text.unwrap()).await;
    cx.answer(response).await?;
    next.call(cx).await
}
