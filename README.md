# xing-grpc

------------------------------------
 현재 개발 극 초기 단계입니다.
------------------------------------

이베스트 증권 Xing API를 gRPC로 서비스 하는 모듈

계획
- Go언어 최신 버전이 DLL 호출에서 불안정한 모습을 보임.
- Go언어 구버전은 Generic기능이 없어서 소스 코드 가독성이 떨어짐.
  
- 증권사 DLL 호출 모듈을 Rust 언어로 재개발.
- 매매 전략 코드는 Go언어 최신 버전으로 업그레이드. 가독성 개선.

- gRPC를 이용해서 다양한 프로그램 언어에서 사용 가능..
- 기타 사용 편의성 개선.

