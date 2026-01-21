//! 비동기 API 사용 예제
//! 
//! 실행 방법:
//! ```
//! cargo run --example async_example
//! ```

use laftel_rs::LaftelClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LaftelClient::new()?;

    // 애니메이션 검색 (예: "슬라임")
    println!("'슬라임' 검색 중...\n");
    let results = client.search_anime("죠죠").await?;

    println!("검색 결과 ({} 개):", results.len());
    for result in results.iter().take(5) {
        println!("  - {} ({})", result.name, result.url());
        println!("    장르: {:?}", result.genres);
        println!();
    }

    // 첫 번째 결과의 상세 정보 조회
    if let Some(first) = results.first() {
        println!("===== {} 상세 정보 조회 =====\n", first.name);
        
        let info = client.get_anime_info(first.id).await?;
        
        println!("줄거리: {}", info.content);
        println!("평점: {:.1}/5.0", info.avg_rating);
        println!("완결 여부: {}", if info.ended { "완결" } else { "방영중" });
        
        // 에피소드 조회
        println!("\n===== 에피소드 목록 =====\n");
        let episodes = client.search_episodes(info.id).await?;
        
        for episode in episodes.iter().take(5) {
            println!(
                "  {} - {}",
                episode.episode_num,
                episode.subject.as_deref().unwrap_or(&episode.title)
            );
        }
        
        if episodes.len() > 5 {
            println!("  ... 외 {}개 에피소드", episodes.len() - 5);
        }
    }

    Ok(())
}
