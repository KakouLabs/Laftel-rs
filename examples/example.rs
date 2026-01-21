//! Laftel API 사용 예제
//!
//! 
//! 실행 방법:
//! ```
//! cargo run --example example
//! ```

use laftel_rs::blocking::LaftelBlockingClient;
use std::io::{self, BufRead, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LaftelBlockingClient::new()?;

    // 검색어 입력
    print!("검색할 애니 이름을 입력하세요: ");
    io::stdout().flush()?;
    
    let stdin = io::stdin();
    let query = stdin.lock().lines().next().unwrap()?;

    // 애니메이션 검색
    let results = client.search_anime(&query)?;
    
    if results.is_empty() {
        println!("검색 결과가 없습니다.");
        return Ok(());
    }

    // 검색 결과 출력
    println!("\n검색 결과:");
    for (index, result) in results.iter().enumerate() {
        println!("{}. {} ({})", index, result.name, result.url());
    }

    // 상세 정보 조회할 애니 선택
    print!("\n번호를 입력하세요: ");
    io::stdout().flush()?;
    
    let selection: usize = stdin.lock().lines().next().unwrap()?.parse()?;
    
    if selection >= results.len() {
        println!("잘못된 번호입니다.");
        return Ok(());
    }

    // 상세 정보 조회
    let anime_info = client.get_anime_info(results[selection].id)?;
    
    // 상세 정보 출력
    println!("\n===== 애니메이션 상세 정보 =====");
    println!("ID: {}", anime_info.id);
    println!("제목: {}", anime_info.name);
    println!("URL: {}", anime_info.url());
    println!("이미지: {}", anime_info.image);
    println!("줄거리: {}", anime_info.content);
    println!("완결 여부: {}", if anime_info.ended { "완결" } else { "방영중" });
    println!("수상: {:?}", anime_info.awards);
    println!("등급: {}", anime_info.content_rating.as_deref().unwrap_or("정보 없음"));
    println!("19금: {}", if anime_info.adultonly { "예" } else { "아니오" });
    println!("시청 가능: {}", if anime_info.viewable { "예" } else { "아니오" });
    println!("장르: {:?}", anime_info.genres);
    println!("태그: {:?}", anime_info.tags);
    
    if let Some(quarter) = &anime_info.air_year_quarter {
        println!("방영 분기: {}", quarter);
    }
    if let Some(day) = &anime_info.air_day {
        println!("방영 요일: {}", day);
    }
    
    println!("평균 평점: {:.2}", anime_info.avg_rating);
    
    if let Some(series_id) = anime_info.series_id {
        println!("시리즈 ID: {}", series_id);
    }
    if let Some(production) = &anime_info.production {
        println!("제작사: {}", production);
    }

    // 에피소드 검색
    println!("\n===== 에피소드 목록 =====");
    let episodes = client.search_episodes(anime_info.id)?;
    
    for episode in episodes.iter().take(10) {
        println!(
            "  {} ({}): {}",
            episode.episode_num,
            episode.title,
            episode.subject.as_deref().unwrap_or("제목 없음")
        );
    }
    
    if episodes.len() > 10 {
        println!("  ... 외 {}개 에피소드", episodes.len() - 10);
    }

    Ok(())
}
