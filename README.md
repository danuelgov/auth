# Auth

다뉴엘 거버넌스를 위한 IDaaS입니다.
유저 식별과 인증을 위한 클라우드 기반 서비스를 제공하며, 여러 어플리케이션에 걸쳐 보안을 중심으로 유저 관리를 간소화 합니다.

## 기능

1. 유저 등록과 관리
2. 인증과 인가
3. 다중인증 지원
4. 유저 세션 관리
5. 보안 이벤트 모니터링 및 알림
6. API와 SDK 제공

## 기술 스택

- 언어: Rust, TypeScript
- 프레임워크: Rocket, Next.js
- 데이터베이스: MySQL
- 클라우드 플랫폼: AWS

## 설치와 실행

1. 이 저장소를 클론합니다.
2. 서버 폴더로 이동한 후 빌드합니다. `cargo build`
3. 클라이언트 폴더로 이동한 후 의존성을 설치합니다. `yarn`
4. 서버를 실행하려면 `cargo run --package auth_{server_name}` 를 입력합니다.
5. 클라이언트를 실행하려면 `yarn dev` 혹은 `yarn start` 를 입력합니다.

## 기여 방법

1. 이 저장소를 포크합니다.
2. 피처 혹은 버그를 수정하는 브랜치를 생성합니다. `git checkout -b feature/feature-name` 혹은 `git checkout -b bugfix/bug-name`
3. 변경사항을 커밋합니다. `git commit -m 'Description of added feature'`
4. 원격 저장소에 해당 브랜치를 푸시합니다. `git push origin feature/feature-name`
5. 풀리퀘스트를 생성합니다.

## 라이선스

이 프로젝트는 MIT 라이선스를 따릅니다. 자세한 내용은 [라이선스](LICENSE) 파일을 확인하세요.
