use std::fmt;
use std::mem::size_of;

use crate::backend::type_c::*;

// 구현된 TR코드
pub static TR현물계좌_총평가_CSPAQ12200: &str = "CSPAQ12200";
pub static TR현물계좌_잔고내역_조회_CSPAQ12300: &str = "CSPAQ12300";
pub static TR현물계좌_주문체결내역_조회_CSPAQ13700: &str = "CSPAQ13700";
pub static TR현물계좌_예수금_주문가능금액_CSPAQ22200: &str = "CSPAQ22200";
pub static TR현물_정상_주문_CSPAT00600: &str = "CSPAT00600";
pub static TR현물_정정_주문_CSPAT00700: &str = "CSPAT00700";
pub static TR현물_취소_주문_CSPAT00800: &str = "CSPAT00800";
pub static TR현물_당일_매매일지_t0150: &str = "t0150";
pub static TR현물_일자별_매매일지_t0151: &str = "t0151";
pub static TR시간_조회_t0167: &str = "t0167";
pub static TR현물_체결_미체결_조회_t0425: &str = "t0425";
pub static TR현물_호가_조회_t1101: &str = "t1101";
pub static TR현물_시세_조회_t1102: &str = "t1102";
pub static TR현물_기간별_조회_t1305: &str = "t1305";
pub static TR현물_당일_전일_분틱_조회_t1310: &str = "t1310";
pub static TR관리_불성실_투자유의_조회_t1404: &str = "t1404";
pub static TR투자경고_매매정지_정리매매_조회_t1405: &str = "t1405";
pub static TR_ETF_시세_조회_t1901: &str = "t1901";
pub static TR_ETF_시간별_추이_t1902: &str = "t1902";
pub static TR_ETF_LP호가_조회_t1906: &str = "t1906";
pub static TR기업정보_요약_t3320: &str = "t3320";
pub static TR재무순위_종합_t3341: &str = "t3341";
pub static TR현물_멀티_현재가_조회_t8407: &str = "t8407";
pub static TR현물_차트_틱_t8411: &str = "t8411";
pub static TR현물_차트_분_t8412: &str = "t8412";
pub static TR현물_차트_일주월_t8413: &str = "t8413";
pub static TR증시_주변_자금_추이_t8428: &str = "t8428";
pub static TR현물_종목_조회_t8436: &str = "t8436";

// 구현된 RT코드;
pub static RT현물_주문_접수_SC0: &str = "SC0";
pub static RT현물_주문_체결_SC1: &str = "SC1";
pub static RT현물_주문_정정_SC2: &str = "SC2";
pub static RT현물_주문_취소_SC3: &str = "SC3";
pub static RT현물_주문_거부_SC4: &str = "SC4";
pub static RT코스피_호가_잔량_H1: &str = "H1_";
pub static RT코스피_시간외_호가_잔량_H2: &str = "H2_";
pub static RT코스닥_호가_잔량_HA: &str = "HA_";
pub static RT코스닥_시간외_호가_잔량_HB: &str = "HB_";
pub static RT코스피_체결_S3: &str = "S3_";
pub static RT코스피_예상_체결_YS3: &str = "YS3";
pub static RT코스닥_체결_K3: &str = "K3_";
pub static RT코스닥_예상_체결_YK3: &str = "YK3";
pub static RT코스피_ETF_NAV_I5: &str = "I5_";
pub static RT주식_VI발동해제_VI: &str = "VI_";
pub static RT시간외_단일가VI발동해제_DVI: &str = "DVI";
pub static RT장_운영정보_JIF: &str = "JIF";

// 자료형 크기
pub static Size_TR_DATA: usize = size_of::<TR_DATA>(); //104;
pub static Size_MSG_DATA: usize = size_of::<MSG_DATA>(); //24;
pub static Size_REALTIME_DATA: usize = size_of::<REALTIME_DATA>(); // 84;
pub static SizeSC0_OutBlock: usize = size_of::<SC0_OutBlock>();
pub static SizeSC1_OutBlock: usize = size_of::<SC1_OutBlock>();
pub static SizeSC2_OutBlock: usize = size_of::<SC2_OutBlock>();
pub static SizeSC3_OutBlock: usize = size_of::<SC3_OutBlock>();
pub static SizeSC4_OutBlock: usize = size_of::<SC4_OutBlock>();
pub static SizeH1_OutBlock: usize = size_of::<H1_OutBlock>();
pub static SizeH2_OutBlock: usize = size_of::<H2_OutBlock>();
pub static SizeHA_OutBlock: usize = size_of::<HA_OutBlock>();
pub static SizeHB_OutBlock: usize = size_of::<HB_OutBlock>();
pub static SizeS3_OutBlock: usize = size_of::<S3_OutBlock>();
pub static SizeYS3OutBlock: usize = size_of::<YS3OutBlock>();
pub static SizeK3_OutBlock: usize = size_of::<K3_OutBlock>();
pub static SizeYK3OutBlock: usize = size_of::<YK3OutBlock>();
pub static SizeI5_OutBlock: usize = size_of::<I5_OutBlock>();
pub static SizeVI_OutBlock: usize = size_of::<VI_OutBlock>();
pub static SizeDVIOutBlock: usize = size_of::<DVIOutBlock>();
pub static SizeJIFOutBlock: usize = size_of::<JIFOutBlock>();
pub static SizeCFOAQ00600InBlock1: usize = size_of::<CFOAQ00600InBlock1>();
pub static SizeCFOAQ00600OutBlock1: usize = size_of::<CFOAQ00600OutBlock1>();
pub static SizeCFOAQ00600OutBlock2: usize = size_of::<CFOAQ00600OutBlock2>();
pub static SizeCFOAQ00600OutBlock3: usize = size_of::<CFOAQ00600OutBlock3>();
pub static SizeCFOAT00100InBlock1: usize = size_of::<CFOAT00100InBlock1>();
pub static SizeCFOAT00100OutBlock: usize = size_of::<CFOAT00100OutBlock>();
pub static SizeCFOAT00100OutBlock1: usize = size_of::<CFOAT00100OutBlock1>();
pub static SizeCFOAT00100OutBlock2: usize = size_of::<CFOAT00100OutBlock2>();
pub static SizeCFOAT00200InBlock1: usize = size_of::<CFOAT00200InBlock1>();
pub static SizeCFOAT00200OutBlock: usize = size_of::<CFOAT00200OutBlock>();
pub static SizeCFOAT00200OutBlock1: usize = size_of::<CFOAT00200OutBlock1>();
pub static SizeCFOAT00200OutBlock2: usize = size_of::<CFOAT00200OutBlock2>();
pub static SizeCFOAT00300InBlock1: usize = size_of::<CFOAT00300InBlock1>();
pub static SizeCFOAT00300OutBlock: usize = size_of::<CFOAT00300OutBlock>();
pub static SizeCFOAT00300OutBlock1: usize = size_of::<CFOAT00300OutBlock1>();
pub static SizeCFOAT00300OutBlock2: usize = size_of::<CFOAT00300OutBlock2>();
pub static SizeCFOBQ10500InBlock1: usize = size_of::<CFOBQ10500InBlock1>();
pub static SizeCFOBQ10500OutBlock1: usize = size_of::<CFOBQ10500OutBlock1>();
pub static SizeCFOBQ10500OutBlock2: usize = size_of::<CFOBQ10500OutBlock2>();
pub static SizeCFOBQ10500OutBlock3: usize = size_of::<CFOBQ10500OutBlock3>();
pub static SizeCFOFQ02400InBlock1: usize = size_of::<CFOFQ02400InBlock1>();
pub static SizeCFOFQ02400OutBlock1: usize = size_of::<CFOFQ02400OutBlock1>();
pub static SizeCFOFQ02400OutBlock2: usize = size_of::<CFOFQ02400OutBlock2>();
pub static SizeCFOFQ02400OutBlock3: usize = size_of::<CFOFQ02400OutBlock3>();
pub static SizeCFOFQ02400OutBlock4: usize = size_of::<CFOFQ02400OutBlock4>();
pub static SizeCSPAQ12200InBlock1: usize = size_of::<CSPAQ12200InBlock1>();
pub static SizeCSPAQ12200OutBlock1: usize = size_of::<CSPAQ12200OutBlock1>();
pub static SizeCSPAQ12200OutBlock2: usize = size_of::<CSPAQ12200OutBlock2>();
pub static SizeCSPAQ12200OutBlock: usize = size_of::<CSPAQ12200OutBlock>();
pub static SizeCSPAQ12300InBlock1: usize = size_of::<CSPAQ12300InBlock1>();
pub static SizeCSPAQ12300OutBlock1: usize = size_of::<CSPAQ12300OutBlock1>();
pub static SizeCSPAQ12300OutBlock2: usize = size_of::<CSPAQ12300OutBlock2>();
pub static SizeCSPAQ12300OutBlock3: usize = size_of::<CSPAQ12300OutBlock3>();
pub static SizeCSPAQ13700InBlock1: usize = size_of::<CSPAQ13700InBlock1>();
pub static SizeCSPAQ13700OutBlock1: usize = size_of::<CSPAQ13700OutBlock1>();
pub static SizeCSPAQ13700OutBlock2: usize = size_of::<CSPAQ13700OutBlock2>();
pub static SizeCSPAQ13700OutBlock3: usize = size_of::<CSPAQ13700OutBlock3>();
pub static SizeCSPAQ22200InBlock1: usize = size_of::<CSPAQ22200InBlock1>();
pub static SizeCSPAQ22200OutBlock1: usize = size_of::<CSPAQ22200OutBlock1>();
pub static SizeCSPAQ22200OutBlock2: usize = size_of::<CSPAQ22200OutBlock2>();
pub static SizeCSPAQ22200OutBlock: usize = size_of::<CSPAQ22200OutBlock>();
pub static SizeCSPAT00600InBlock1: usize = size_of::<CSPAT00600InBlock1>();
pub static SizeCSPAT00600OutBlock: usize = size_of::<CSPAT00600OutBlock>();
pub static SizeCSPAT00600OutBlock1: usize = size_of::<CSPAT00600OutBlock1>();
pub static SizeCSPAT00600OutBlock2: usize = size_of::<CSPAT00600OutBlock2>();
pub static SizeCSPAT00700InBlock1: usize = size_of::<CSPAT00700InBlock1>();
pub static SizeCSPAT00700OutBlock: usize = size_of::<CSPAT00700OutBlock>();
pub static SizeCSPAT00700OutBlock1: usize = size_of::<CSPAT00700OutBlock1>();
pub static SizeCSPAT00700OutBlock2: usize = size_of::<CSPAT00700OutBlock2>();
pub static SizeCSPAT00800InBlock1: usize = size_of::<CSPAT00800InBlock1>();
pub static SizeCSPAT00800OutBlock: usize = size_of::<CSPAT00800OutBlock>();
pub static SizeCSPAT00800OutBlock1: usize = size_of::<CSPAT00800OutBlock1>();
pub static SizeCSPAT00800OutBlock2: usize = size_of::<CSPAT00800OutBlock2>();
pub static SizeT0150InBlock: usize = size_of::<T0150InBlock>();
pub static SizeT0150OutBlock: usize = size_of::<T0150OutBlock>();
pub static SizeT0150OutBlock1: usize = size_of::<T0150OutBlock1>();
pub static SizeT0151InBlock: usize = size_of::<T0151InBlock>();
pub static SizeT0151OutBlock: usize = size_of::<T0151OutBlock>();
pub static SizeT0151OutBlock1: usize = size_of::<T0151OutBlock1>();
pub static SizeT0167OutBlock: usize = size_of::<T0167OutBlock>();
pub static SizeT0425InBlock: usize = size_of::<T0425InBlock>();
pub static SizeT0425OutBlock: usize = size_of::<T0425OutBlock>();
pub static SizeT0425OutBlock1: usize = size_of::<T0425OutBlock1>();
pub static SizeT0434InBlock: usize = size_of::<T0434InBlock>();
pub static SizeT0434OutBlock: usize = size_of::<T0434OutBlock>();
pub static SizeT0434OutBlock1: usize = size_of::<T0434OutBlock1>();
pub static SizeT1101InBlock: usize = size_of::<T1101InBlock>();
pub static SizeT1101OutBlock: usize = size_of::<T1101OutBlock>();
pub static SizeT1102InBlock: usize = size_of::<T1102InBlock>();
pub static SizeT1102OutBlock: usize = size_of::<T1102OutBlock>();
pub static SizeT1305InBlock: usize = size_of::<T1305InBlock>();
pub static SizeT1305OutBlock: usize = size_of::<T1305OutBlock>();
pub static SizeT1305OutBlock1: usize = size_of::<T1305OutBlock1>();
pub static SizeT1310InBlock: usize = size_of::<T1310InBlock>();
pub static SizeT1310OutBlock: usize = size_of::<T1310OutBlock>();
pub static SizeT1310OutBlock1: usize = size_of::<T1310OutBlock1>();
pub static SizeT1404InBlock: usize = size_of::<T1404InBlock>();
pub static SizeT1404OutBlock: usize = size_of::<T1404OutBlock>();
pub static SizeT1404OutBlock1: usize = size_of::<T1404OutBlock1>();
pub static SizeT1405InBlock: usize = size_of::<T1405InBlock>();
pub static SizeT1405OutBlock: usize = size_of::<T1405OutBlock>();
pub static SizeT1405OutBlock1: usize = size_of::<T1405OutBlock1>();
pub static SizeT1901InBlock: usize = size_of::<T1901InBlock>();
pub static SizeT1901OutBlock: usize = size_of::<T1901OutBlock>();
pub static SizeT1902InBlock: usize = size_of::<T1902InBlock>();
pub static SizeT1902OutBlock: usize = size_of::<T1902OutBlock>();
pub static SizeT1902OutBlock1: usize = size_of::<T1902OutBlock1>();
pub static SizeT1906InBlock: usize = size_of::<T1906InBlock>();
pub static SizeT1906OutBlock: usize = size_of::<T1906OutBlock>();
pub static SizeT3320InBlock: usize = size_of::<T3320InBlock>();
pub static SizeT3320OutBlock: usize = size_of::<T3320OutBlock>();
pub static SizeT3320OutBlock1: usize = size_of::<T3320OutBlock1>();
pub static SizeT3341InBlock: usize = size_of::<T3341InBlock>();
pub static SizeT3341OutBlock: usize = size_of::<T3341OutBlock>();
pub static SizeT3341OutBlock1: usize = size_of::<T3341OutBlock1>();
pub static SizeT8407InBlock: usize = size_of::<T8407InBlock>();
pub static SizeT8407OutBlock1: usize = size_of::<T8407OutBlock1>();
pub static SizeT8411InBlock: usize = size_of::<T8411InBlock>();
pub static SizeT8411OutBlock: usize = size_of::<T8411OutBlock>();
pub static SizeT8411OutBlock1: usize = size_of::<T8411OutBlock1>();
pub static SizeT8412InBlock: usize = size_of::<T8412InBlock>();
pub static SizeT8412OutBlock: usize = size_of::<T8412OutBlock>();
pub static SizeT8412OutBlock1: usize = size_of::<T8412OutBlock1>();
pub static SizeT8413InBlock: usize = size_of::<T8413InBlock>();
pub static SizeT8413OutBlock: usize = size_of::<T8413OutBlock>();
pub static SizeT8413OutBlock1: usize = size_of::<T8413OutBlock1>();
pub static SizeT8428InBlock: usize = size_of::<T8428InBlock>();
pub static SizeT8428OutBlock: usize = size_of::<T8428OutBlock>();
pub static SizeT8428OutBlock1: usize = size_of::<T8428OutBlock1>();
pub static SizeT8432OutBlock: usize = size_of::<T8432OutBlock>();
pub static SizeT8436InBlock: usize = size_of::<T8436InBlock>();
pub static SizeT8436OutBlock: usize = size_of::<T8436OutBlock>();