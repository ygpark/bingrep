# TODO

## 완료된 작업 ✅

1. 에러 처리 일관성
   [x] - main.rs:23-24에서 map_err(BingrepError::Io)? 대신 ? 사용 가능
   [x] - pcre2 에러 처리가 누락됨 (pcre2_processor 모듈 확인 필요)

2. 코드 안정성
   [x] - stream.rs:93: 정규식 매치 위치를 벡터로 복사하는 부분이 메모리 오버헤드 발생
   [x] - 큰 파일에서 많은 매치 시 메모리 사용량 증가 가능

3. CLI 개선
   [x] - cli.rs:37: show_help()가 하드코딩된 문자열 사용 - clap의 자동 생성 help 활용 권장
   [x] - 입력 검증 로직이 main.rs에 흩어져 있음

4. 문서화
   [x] - 주요 함수들의 doc comment 부족
   [x] - README.md는 있지만 API 문서화 미흡

## 수행한 개선사항

### 1. 에러 처리 개선
- `main.rs`에서 불필요한 `map_err` 제거하고 `?` 연산자 직접 사용
- `pcre2_processor::find_matches` 함수가 Result를 반환하도록 수정하여 에러 처리 강화

### 2. 메모리 최적화
- `stream.rs`에서 모든 매치를 벡터로 수집하는 대신, 필요한 매치만 선별적으로 수집
- 중복 검사를 조기에 수행하여 메모리 사용량 감소
- limit 옵션을 고려하여 불필요한 매치 수집 방지

### 3. CLI 개선
- `clap`의 `long_about` 속성을 활용하여 자동 help 생성
- 수동으로 구현한 `show_help()` 메서드 제거
- 입력 검증 로직을 `Config::validate_cli()` 메서드로 중앙집중화

### 4. 문서화 개선
- 모든 주요 함수와 구조체에 doc comment 추가
- 파라미터, 반환값, 예제 코드 포함
- `lib.rs`에 크레이트 레벨 문서 추가
- API 사용 예제와 모듈 설명 추가
