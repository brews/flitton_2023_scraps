use actix_web::HttpResponse;

// Code on demand. This is tightly coupled to frontend implementation.

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            "<html>\
            <script>\
                localStorage.removeItem('user-token'); \
                // // Flushes client's local cache on logout. Avoid data leak when account switching.
                // localStorage.removeItem('item-cache-date'); \
                // localStorage.removeItem('item-cache-data-pending'); \
                // localStorage.removeItem('item-cache-data-done'); \
                window.location.replace(document.location.origin);\
            </script>\
        </html>",
        )
}
