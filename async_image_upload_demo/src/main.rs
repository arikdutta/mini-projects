use std::time::Duration;
use tokio::time::sleep;

// Simulates one image upload to cloud storage (e.g. S3)
// BLOCKING version — freezes the whole thread while "uploading"
fn upload_image_blocking(image_name: &str) -> String {
    println!("  [BLOCKING] Uploading {} ...", image_name);
    std::thread::sleep(Duration::from_secs(2)); // pretend network latency
    let url = format!("https://cdn.example.com/images/{}", image_name);
    println!("  [BLOCKING] {} uploaded -> {}", image_name, url);
    url
}

// NON-BLOCKING version — yields control while awaiting the network
async fn upload_image_async(image_name: String) -> String {
    println!("  [ASYNC] Uploading {} ...", image_name);
    sleep(Duration::from_secs(2)).await; // pretend network latency
    let url = format!("https://cdn.example.com/images/{}", image_name);
    println!("  [ASYNC] {} uploaded -> {}", image_name, url);
    url
}

// After uploads finish, notify CDN to invalidate old cached version
async fn invalidate_cdn_cache(url: String) {
    println!("  [CDN] Invalidating cache for {} ...", url);
    sleep(Duration::from_millis(300)).await;
    println!("  [CDN] Cache cleared for {}", url);
}

/* ========================================================== */
/*                         MAIN                               */
/* ========================================================== */

#[tokio::main]
async fn main() {
    let images = vec![
        "photo_1.jpg",
        "photo_2.jpg",
        "photo_3.jpg",
        "photo_4.jpg",
        "photo_5.jpg",
    ];

    // -- BLOCKING --------------------------------------------------
    println!("=== BLOCKING UPLOAD (one at a time) ===");
    let start = std::time::Instant::now();

    let mut blocking_urls: Vec<String> = Vec::new();
    for image in &images {
        let url = upload_image_blocking(image);
        blocking_urls.push(url);
    }

    println!(
        "\nBlocking: uploaded {} images in {:.2?}\n",
        images.len(),
        start.elapsed()
    );
    // Expected: 5 images x 2s = ~10 seconds

    // -- NON-BLOCKING ----------------------------------------------
    println!("=== NON-BLOCKING UPLOAD (concurrent) ===");
    let start = std::time::Instant::now();

    // Spawn all uploads concurrently; each runs without blocking others
    let upload_handles: Vec<_> = images
        .iter()
        .map(|name| {
            let name = name.to_string();
            tokio::spawn(async move { upload_image_async(name).await })
        })
        .collect();

    // Wait for all uploads to finish and collect URLs
    let mut async_urls: Vec<String> = Vec::new();
    for handle in upload_handles {
        let url = handle.await.unwrap();
        async_urls.push(url);
    }

    println!(
        "\nAsync: uploaded {} images in {:.2?}\n",
        images.len(),
        start.elapsed()
    );
    // Expected: all 5 run at the same time = ~2 seconds

    // -- CDN INVALIDATION (concurrent async after uploads) ---------
    println!("=== CDN CACHE INVALIDATION ===");
    let start = std::time::Instant::now();

    let cdn_handles: Vec<_> = async_urls
        .into_iter()
        .map(|url| tokio::spawn(async move { invalidate_cdn_cache(url).await }))
        .collect();

    for handle in cdn_handles {
        handle.await.unwrap();
    }

    println!(
        "\nCDN: invalidated {} URLs in {:.2?}",
        images.len(),
        start.elapsed()
    );
}
